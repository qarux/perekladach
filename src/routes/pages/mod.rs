use crate::storage;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use futures::TryStreamExt;
use sqlx::PgPool;
use tokio::fs::{remove_file, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

pub async fn page_info(
    uuid: web::Path<Uuid>,
    db_pool: web::Data<PgPool>,
) -> actix_web::Result<impl Responder> {
    // TODO
    Ok(HttpResponse::Ok().finish())
}

pub async fn source_image(uuid: web::Path<Uuid>) -> actix_web::Result<NamedFile> {
    let path = storage::get_source_image_path(uuid.into_inner());
    Ok(NamedFile::open(path)?)
}

pub async fn translated_image(uuid: web::Path<Uuid>) -> actix_web::Result<NamedFile> {
    let path = storage::get_translated_image_path(uuid.into_inner());
    Ok(NamedFile::open(path)?)
}

pub async fn save_source_image(
    payload: Multipart,
    uuid: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let path = storage::get_source_image_path(uuid.into_inner());
    storage::save_file(payload, &path)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn save_translated_image(
    payload: Multipart,
    uuid: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let path = storage::get_translated_image_path(uuid.into_inner());
    storage::save_file(payload, &path)?;
    Ok(HttpResponse::Ok().finish())
}
