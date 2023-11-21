use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns available emojis categories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetEmojiCategories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of emoji categories to return; pass null to get default emoji categories

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "EmojiCategoryType::_is_default")]
    type_: EmojiCategoryType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetEmojiCategories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetEmojiCategories {}

impl GetEmojiCategories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetEmojiCategoriesBuilder {
        let mut inner = GetEmojiCategories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getEmojiCategories".to_string();

        GetEmojiCategoriesBuilder { inner }
    }

    pub fn type_(&self) -> &EmojiCategoryType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct GetEmojiCategoriesBuilder {
    inner: GetEmojiCategories,
}

#[deprecated]
pub type RTDGetEmojiCategoriesBuilder = GetEmojiCategoriesBuilder;

impl GetEmojiCategoriesBuilder {
    pub fn build(&self) -> GetEmojiCategories {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<EmojiCategoryType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<GetEmojiCategories> for GetEmojiCategories {
    fn as_ref(&self) -> &GetEmojiCategories {
        self
    }
}

impl AsRef<GetEmojiCategories> for GetEmojiCategoriesBuilder {
    fn as_ref(&self) -> &GetEmojiCategories {
        &self.inner
    }
}
