use crate::database::InsertError;
use crate::slug::Slug;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct PageInfo {
    uuid: Uuid,
    number: u32,
    translated: bool,
    project_slug: Slug,
    chapter_index: f32,
}

#[derive(Deserialize)]
pub struct InsertablePageInfo {
    number: u32,
    project_slug: Slug,
    chapter_index: f32,
}

impl PageInfo {
    // TODO
}

impl InsertablePageInfo {
    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        let uuid = Uuid::new_v4();
        let translated = false;
        sqlx::query!(
            r#"
            INSERT INTO pages (uuid, number, translated, project_slug, chapter_index)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            uuid,
            self.number,
            translated,
            self.project_slug,
            self.chapter_index,
        )
        .execute(db_pool)
        .await?;

        Ok(())
    }
}
