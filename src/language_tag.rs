use language_tags::LanguageTag as LangTag;
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode};

#[derive(Deserialize, Serialize, sqlx::Encode)]
#[serde(try_from = "String")]
pub struct LanguageTag(LangTag);

impl<'r, DB: Database> Decode<'r, DB> for LanguageTag
where
    String: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <String as Decode<DB>>::decode(value)?;
        Ok(value.try_into()?)
    }
}

impl<DB: Database> sqlx::Type<DB> for LanguageTag
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl TryFrom<String> for LanguageTag {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LanguageTag(LangTag::parse(&value)?))
    }
}

impl AsRef<str> for LanguageTag {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
