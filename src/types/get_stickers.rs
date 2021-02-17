use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String representation of emoji. If empty, returns all known installed stickers
    emoji: String,
    /// The maximum number of stickers to be returned
    limit: i32,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetStickersBuilder {
        let mut inner = GetStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStickers".to_string();

        RTDGetStickersBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetStickersBuilder {
    inner: GetStickers,
}

impl RTDGetStickersBuilder {
    pub fn build(&self) -> GetStickers {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetStickers> for GetStickers {
    fn as_ref(&self) -> &GetStickers {
        self
    }
}

impl AsRef<GetStickers> for RTDGetStickersBuilder {
    fn as_ref(&self) -> &GetStickers {
        &self.inner
    }
}
