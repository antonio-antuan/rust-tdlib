use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns stickers from the installed sticker sets that correspond to any of the given emoji or can be found by sticker-specific keywords. If the query is non-empty, then favorite, recently used or trending stickers may also be returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the stickers to return

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// Search query; a space-separated list of emoji or a keyword prefix. If empty, returns all known installed stickers

    #[serde(default)]
    query: String,
    /// The maximum number of stickers to be returned

    #[serde(default)]
    limit: i32,
    /// Chat identifier for which to return stickers. Available custom emoji stickers may be different for different chats

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStickers {}

impl GetStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStickersBuilder {
        let mut inner = GetStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStickers".to_string();

        GetStickersBuilder { inner }
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetStickersBuilder {
    inner: GetStickers,
}

#[deprecated]
pub type RTDGetStickersBuilder = GetStickersBuilder;

impl GetStickersBuilder {
    pub fn build(&self) -> GetStickers {
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

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetStickers> for GetStickers {
    fn as_ref(&self) -> &GetStickers {
        self
    }
}

impl AsRef<GetStickers> for GetStickersBuilder {
    fn as_ref(&self) -> &GetStickers {
        &self.inner
    }
}
