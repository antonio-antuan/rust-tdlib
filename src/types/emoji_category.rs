use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of similar emoji to search for in getStickers and searchStickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the category

    #[serde(default)]
    name: String,
    /// Custom emoji sticker, which represents icon of the category
    icon: Sticker,
    /// List of emojis in the category

    #[serde(default)]
    emojis: Vec<String>,
}

impl RObject for EmojiCategory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl EmojiCategory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiCategoryBuilder {
        let mut inner = EmojiCategory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiCategoryBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn icon(&self) -> &Sticker {
        &self.icon
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct EmojiCategoryBuilder {
    inner: EmojiCategory,
}

#[deprecated]
pub type RTDEmojiCategoryBuilder = EmojiCategoryBuilder;

impl EmojiCategoryBuilder {
    pub fn build(&self) -> EmojiCategory {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn icon<T: AsRef<Sticker>>(&mut self, icon: T) -> &mut Self {
        self.inner.icon = icon.as_ref().clone();
        self
    }

    pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }
}

impl AsRef<EmojiCategory> for EmojiCategory {
    fn as_ref(&self) -> &EmojiCategory {
        self
    }
}

impl AsRef<EmojiCategory> for EmojiCategoryBuilder {
    fn as_ref(&self) -> &EmojiCategory {
        &self.inner
    }
}
