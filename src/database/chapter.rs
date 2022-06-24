use crate::database::{InsertError, SelectError};
use crate::slug::Slug;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Chapter {
    index: f32,
    title: String,
    page_count: u32,
    translation_progress: f32,
    project_slug: Slug,
}

#[derive(Deserialize)]
pub struct InsertableChapter {
    index: Option<f32>,
    title: String,
    project_slug: Slug,
}

// TODO
impl Chapter {
    pub async fn get(
        project_slug: Slug,
        index: f32,
        db_pool: &PgPool,
    ) -> Result<Self, SelectError> {
        let chapter = sqlx::query_as!(
            Chapter,
            r#"
            SELECT index,
            title,
            0 AS "page_count!: u32",
            0 AS "translation_progress!: f32",
            project_slug AS "project_slug: Slug"
            FROM chapters
            WHERE project_slug = $1 AND index = $2
            "#,
            project_slug.as_ref(),
            index
        )
        .fetch_one(db_pool)
        .await?;

        Ok(chapter)
    }

    pub async fn get_with_project_slug(
        project_slug: Slug,
        db_pool: &PgPool,
    ) -> Result<Vec<Self>, SelectError> {
        let chapters = sqlx::query_as!(
            Chapter,
            r#"
            SELECT index,
            title,
            0 AS "page_count!: u32",
            0 AS "translation_progress!: f32",
            project_slug AS "project_slug: Slug"
            FROM chapters
            WHERE project_slug = $1
            "#,
            project_slug.as_ref(),
        )
        .fetch_all(db_pool)
        .await?;

        Ok(chapters)
    }
}

impl InsertableChapter {
    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        let index = if let Some(idx) = self.index {
            idx
        } else {
            self.get_last_index(db_pool)
                .await
                .map_err(|err| InsertError::Other(err.into()))?
                .trunc()
                + 1.0
        };

        sqlx::query!(
            r#"
            INSERT INTO chapters (index, title, project_slug)
            VALUES ($1, $2, $3)
            "#,
            index,
            self.title,
            self.project_slug.as_ref()
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    async fn get_last_index(&self, db_pool: &PgPool) -> Result<f32, SelectError> {
        let index = sqlx::query!(
            r#"
            SELECT MAX(index)
            FROM chapters
            WHERE project_slug = $1
            "#,
            self.project_slug.as_ref()
        )
        .fetch_one(db_pool)
        .await?;

        Ok(index.max.unwrap_or(0.0))
    }
}
