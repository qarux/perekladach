use crate::database::project::Project;
use crate::database::{InsertError, SelectError};
use crate::slug::Slug;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound};
use actix_web::{get, web, HttpResponse, Responder};
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::borrow::Cow;

pub async fn new_project(
    db_pool: web::Data<PgPool>,
    payload: web::Json<Project>,
) -> actix_web::Result<HttpResponse> {
    payload
        .into_inner()
        .insert(db_pool.get_ref())
        .await
        .map_err(|err| match err {
            InsertError::UniqueViolation(e) => ErrorBadRequest(e),
            InsertError::Other(e) => ErrorInternalServerError(e),
        })?;

    Ok(HttpResponse::Created().finish())
}

#[get("/{slug}")]
pub async fn project(
    db_pool: web::Data<PgPool>,
    slug: web::Path<Slug>,
) -> actix_web::Result<impl Responder> {
    let project = Project::get(slug.into_inner(), db_pool.get_ref())
        .await
        .map_err(|err| match err {
            SelectError::NoDataFound(e) => ErrorNotFound(e),
            SelectError::Other(e) => ErrorInternalServerError(e),
        })?;

    Ok(web::Json(project))
}
