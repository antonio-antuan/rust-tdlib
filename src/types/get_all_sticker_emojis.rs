use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns unique emoji that correspond to stickers to be found by the getStickers(sticker_type, query, 1000000, chat_id)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAllStickerEmojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the stickers to search for

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// Search query

    #[serde(default)]
    query: String,
    /// Chat identifier for which to find stickers

    #[serde(default)]
    chat_id: i64,
    /// Pass true if only main emoji for each found sticker must be included in the result

    #[serde(default)]
    return_only_main_emoji: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAllStickerEmojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAllStickerEmojis {}

impl GetAllStickerEmojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetAllStickerEmojisBuilder {
        let mut inner = GetAllStickerEmojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAllStickerEmojis".to_string();

        GetAllStickerEmojisBuilder { inner }
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn return_only_main_emoji(&self) -> bool {
        self.return_only_main_emoji
    }
}

#[doc(hidden)]
pub struct GetAllStickerEmojisBuilder {
    inner: GetAllStickerEmojis,
}

#[deprecated]
pub type RTDGetAllStickerEmojisBuilder = GetAllStickerEmojisBuilder;

impl GetAllStickerEmojisBuilder {
    pub fn build(&self) -> GetAllStickerEmojis {
        self.inner.clone()
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn return_only_main_emoji(&mut self, return_only_main_emoji: bool) -> &mut Self {
        self.inner.return_only_main_emoji = return_only_main_emoji;
        self
    }
}

impl AsRef<GetAllStickerEmojis> for GetAllStickerEmojis {
    fn as_ref(&self) -> &GetAllStickerEmojis {
        self
    }
}

impl AsRef<GetAllStickerEmojis> for GetAllStickerEmojisBuilder {
    fn as_ref(&self) -> &GetAllStickerEmojis {
        &self.inner
    }
}
