use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of custom emoji stickers by their identifiers. Stickers are returned in arbitrary order. Only found stickers are returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCustomEmojiStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of custom emoji stickers. At most 200 custom emoji stickers can be received simultaneously

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
    #[serde(default)]
    custom_emoji_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCustomEmojiStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCustomEmojiStickers {}

impl GetCustomEmojiStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCustomEmojiStickersBuilder {
        let mut inner = GetCustomEmojiStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCustomEmojiStickers".to_string();

        GetCustomEmojiStickersBuilder { inner }
    }

    pub fn custom_emoji_ids(&self) -> &Vec<i64> {
        &self.custom_emoji_ids
    }
}

#[doc(hidden)]
pub struct GetCustomEmojiStickersBuilder {
    inner: GetCustomEmojiStickers,
}

#[deprecated]
pub type RTDGetCustomEmojiStickersBuilder = GetCustomEmojiStickersBuilder;

impl GetCustomEmojiStickersBuilder {
    pub fn build(&self) -> GetCustomEmojiStickers {
        self.inner.clone()
    }

    pub fn custom_emoji_ids(&mut self, custom_emoji_ids: Vec<i64>) -> &mut Self {
        self.inner.custom_emoji_ids = custom_emoji_ids;
        self
    }
}

impl AsRef<GetCustomEmojiStickers> for GetCustomEmojiStickers {
    fn as_ref(&self) -> &GetCustomEmojiStickers {
        self
    }
}

impl AsRef<GetCustomEmojiStickers> for GetCustomEmojiStickersBuilder {
    fn as_ref(&self) -> &GetCustomEmojiStickers {
        &self.inner
    }
}
