use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of emoji categories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of categories

    #[serde(default)]
    categories: Vec<EmojiCategory>,
}

impl RObject for EmojiCategories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl EmojiCategories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiCategoriesBuilder {
        let mut inner = EmojiCategories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiCategoriesBuilder { inner }
    }

    pub fn categories(&self) -> &Vec<EmojiCategory> {
        &self.categories
    }
}

#[doc(hidden)]
pub struct EmojiCategoriesBuilder {
    inner: EmojiCategories,
}

#[deprecated]
pub type RTDEmojiCategoriesBuilder = EmojiCategoriesBuilder;

impl EmojiCategoriesBuilder {
    pub fn build(&self) -> EmojiCategories {
        self.inner.clone()
    }

    pub fn categories(&mut self, categories: Vec<EmojiCategory>) -> &mut Self {
        self.inner.categories = categories;
        self
    }
}

impl AsRef<EmojiCategories> for EmojiCategories {
    fn as_ref(&self) -> &EmojiCategories {
        self
    }
}

impl AsRef<EmojiCategories> for EmojiCategoriesBuilder {
    fn as_ref(&self) -> &EmojiCategories {
        &self.inner
    }
}
