use crate::database::{InsertError, SelectError};
use crate::slug::Slug;
use language_tags::LanguageTag;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub slug: Slug,
    pub name: String,
    pub source_language: LanguageTag,
    pub target_language: LanguageTag,
}

impl Project {
    pub async fn insert(self, db_pool: &PgPool) -> Result<(), InsertError> {
        sqlx::query!(
            r#"
            INSERT INTO projects (slug, name, source_language, target_language)
            VALUES ($1, $2, $3, $4)
            "#,
            self.slug.as_ref(),
            self.name,
            self.source_language.primary_language(),
            self.target_language.primary_language()
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }

    pub async fn get(slug: Slug, db_pool: &PgPool) -> Result<Self, SelectError> {
        let record = sqlx::query!(
            r#"
            SELECT *
            FROM projects
            WHERE slug = $1
            "#,
            slug.as_ref(),
        )
        .fetch_one(db_pool)
        .await?;

        let project = Project {
            slug: Slug::try_from(record.slug)?,
            name: record.name,
            source_language: LanguageTag::parse(&record.source_language)
                .map_err(|err| SelectError::Other(err.into()))?,
            target_language: LanguageTag::parse(&record.target_language)
                .map_err(|err| SelectError::Other(err.into()))?,
        };
        Ok(project)
    }
}
