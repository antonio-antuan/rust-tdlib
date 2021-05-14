use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to return stickers and masks that were recently attached to photos or video files; pass false to return recently sent stickers
    is_attached: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecentStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecentStickers {}

impl GetRecentStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetRecentStickersBuilder {
        let mut inner = GetRecentStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecentStickers".to_string();

        RTDGetRecentStickersBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }
}

#[doc(hidden)]
pub struct RTDGetRecentStickersBuilder {
    inner: GetRecentStickers,
}

impl RTDGetRecentStickersBuilder {
    pub fn build(&self) -> GetRecentStickers {
        self.inner.clone()
    }

    pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
        self.inner.is_attached = is_attached;
        self
    }
}

impl AsRef<GetRecentStickers> for GetRecentStickers {
    fn as_ref(&self) -> &GetRecentStickers {
        self
    }
}

impl AsRef<GetRecentStickers> for RTDGetRecentStickersBuilder {
    fn as_ref(&self) -> &GetRecentStickers {
        &self.inner
    }
}
