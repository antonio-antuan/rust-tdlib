use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of trending sticker sets. For the optimal performance the number of returned sticker sets is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTrendingStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The offset from which to return the sticker sets; must be non-negative
    offset: i32,
    /// The maximum number of sticker sets to be returned; must be non-negative. Fewer sticker sets may be returned than specified by the limit, even if the end of the list has not been reached
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetTrendingStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetTrendingStickerSets {}

impl GetTrendingStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetTrendingStickerSetsBuilder {
        let mut inner = GetTrendingStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getTrendingStickerSets".to_string();

        RTDGetTrendingStickerSetsBuilder { inner }
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetTrendingStickerSetsBuilder {
    inner: GetTrendingStickerSets,
}

impl RTDGetTrendingStickerSetsBuilder {
    pub fn build(&self) -> GetTrendingStickerSets {
        self.inner.clone()
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetTrendingStickerSets> for GetTrendingStickerSets {
    fn as_ref(&self) -> &GetTrendingStickerSets {
        self
    }
}

impl AsRef<GetTrendingStickerSets> for RTDGetTrendingStickerSetsBuilder {
    fn as_ref(&self) -> &GetTrendingStickerSets {
        &self.inner
    }
}
