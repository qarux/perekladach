use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct Slug(String);

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
