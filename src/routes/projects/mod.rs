use crate::slug::Slug;
use crate::utils::postgres_error_codes::UNIQUE_VIOLATION;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, web, HttpResponse, Responder};
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::borrow::Cow;

#[derive(Deserialize, Serialize)]
pub struct Project {
    // TODO Limit length
    pub slug: Slug,
    pub name: String,
    pub source_language: LanguageTag,
    pub target_language: LanguageTag,
}

pub async fn new_project(
    db_pool: web::Data<PgPool>,
    payload: web::Json<Project>,
) -> actix_web::Result<HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO projects (slug, name, source_lang, target_lang)
        VALUES ($1, $2, $3, $4)
        "#,
        payload.slug.as_ref(),
        payload.name,
        payload.source_language.primary_language(),
        payload.target_language.primary_language()
    )
    .execute(db_pool.get_ref())
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(db_error)
            if db_error.code() == Some(Cow::Borrowed(UNIQUE_VIOLATION)) =>
        {
            ErrorBadRequest(db_error)
        }
        _ => ErrorInternalServerError(err),
    })?;

    Ok(HttpResponse::Created().finish())
}

#[get("/{slug}")]
pub async fn project(
    db_pool: web::Data<PgPool>,
    slug: web::Path<Slug>,
) -> actix_web::Result<impl Responder> {
    let slug = slug.into_inner();
    let query = sqlx::query!(
        r#"
        SELECT *
        FROM projects
        WHERE slug = $1
        "#,
        slug.as_ref(),
    )
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => ErrorNotFound(e),
        _ => ErrorInternalServerError(e),
    })?;

    let project = Project {
        slug: Slug::try_from(query.slug).map_err(ErrorInternalServerError)?,
        name: query.name,
        source_language: LanguageTag::parse(&query.source_lang).map_err(ErrorInternalServerError)?,
        target_language: LanguageTag::parse(&query.target_lang).map_err(ErrorInternalServerError)?,
    };

    Ok(web::Json(project))
}
