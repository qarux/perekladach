use crate::database::chapter::{Chapter, InsertableChapter};
use crate::database::InsertError;
use crate::slug::Slug;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

pub async fn new_chapter(
    db_pool: web::Data<PgPool>,
    payload: web::Json<InsertableChapter>,
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

pub async fn chapters(
    db_pool: web::Data<PgPool>,
    project_slug: web::Path<Slug>,
) -> actix_web::Result<impl Responder> {
    let chapters = Chapter::get_with_project_slug(project_slug.into_inner(), db_pool.get_ref())
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(web::Json(chapters))
}
