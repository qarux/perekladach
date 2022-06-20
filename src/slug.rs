use anyhow::bail;
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode};

#[derive(Deserialize, Serialize, sqlx::Encode)]
#[serde(try_from = "String")]
pub struct Slug(String);

impl<'r, DB: Database> Decode<'r, DB> for Slug
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(value.try_into()?)
    }
}

impl<DB: Database> sqlx::Type<DB> for Slug
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl TryFrom<String> for Slug {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.split('-').all(|substring| {
            !substring.is_empty()
                && substring
                    .chars()
                    .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
        }) {
            Ok(Slug(value))
        } else {
            bail!("Illegal characters");
        }
    }
}

impl AsRef<str> for Slug {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::slug::Slug;

    #[test]
    fn test_slug() {
        assert!(Slug::try_from("slug".to_owned()).is_ok());
        assert!(Slug::try_from("42".to_owned()).is_ok());
        assert!(Slug::try_from("project-name".to_owned()).is_ok());
        assert!(Slug::try_from("project1-name-test".to_owned()).is_ok());

        assert!(Slug::try_from("-".to_owned()).is_err());
        assert!(Slug::try_from("project--name".to_owned()).is_err());
        assert!(Slug::try_from("-slug".to_owned()).is_err());
        assert!(Slug::try_from("slug-".to_owned()).is_err());
        assert!(Slug::try_from("".to_owned()).is_err());
        assert!(Slug::try_from("project_name".to_owned()).is_err());
        assert!(Slug::try_from("project-(name".to_owned()).is_err());
    }
}
