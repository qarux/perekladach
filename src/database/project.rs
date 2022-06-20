use crate::database::{InsertError, SelectError};
use crate::slug::Slug;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::language_tag::LanguageTag;

#[derive(Serialize, sqlx::FromRow)]
pub struct Project {
    pub slug: Slug,
    pub name: String,
    pub source_language: LanguageTag,
    pub target_language: LanguageTag,
    pub chapters_count: u32,
    pub translation_progress: f32,
}

#[derive(Deserialize)]
pub struct InsertableProject {
    pub slug: Slug,
    pub name: String,
    pub source_language: LanguageTag,
    pub target_language: LanguageTag,
}

impl Project {
    pub async fn get(slug: Slug, db_pool: &PgPool) -> Result<Self, SelectError> {
        // TODO
        let project = sqlx::query_as!(
            Project,
            r#"
            SELECT slug AS "slug: Slug",
            name,
            source_language AS "source_language: LanguageTag",
            target_language AS "target_language: LanguageTag",
            0 AS "chapters_count!: u32",
            0 AS "translation_progress!: f32"
            FROM projects
            WHERE slug = $1
            "#,
            slug.as_ref(),
        )
        .fetch_one(db_pool)
        .await?;

        Ok(project)
    }

    pub async fn get_all(db_pool: &PgPool) -> Result<Vec<Self>, SelectError> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT slug AS "slug: Slug",
            name,
            source_language AS "source_language: LanguageTag",
            target_language AS "target_language: LanguageTag",
            0 AS "chapters_count!: u32",
            0 AS "translation_progress!: f32"
            FROM projects
            "#
        )
        .fetch_all(db_pool)
        .await?;

        Ok(projects)
    }
}

impl InsertableProject {
    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        sqlx::query!(
            r#"
            INSERT INTO projects (slug, name, source_language, target_language)
            VALUES ($1, $2, $3, $4)
            "#,
            self.slug.as_ref(),
            self.name,
            self.source_language.as_ref(),
            self.target_language.as_ref()
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }
}
