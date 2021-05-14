use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Clears the list of recently used stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to clear the list of stickers recently attached to photo or video files; pass false to clear the list of recently sent stickers
    is_attached: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearRecentStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearRecentStickers {}

impl ClearRecentStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDClearRecentStickersBuilder {
        let mut inner = ClearRecentStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearRecentStickers".to_string();

        RTDClearRecentStickersBuilder { inner }
    }

    pub fn is_attached(&self) -> bool {
        self.is_attached
    }
}

#[doc(hidden)]
pub struct RTDClearRecentStickersBuilder {
    inner: ClearRecentStickers,
}

impl RTDClearRecentStickersBuilder {
    pub fn build(&self) -> ClearRecentStickers {
        self.inner.clone()
    }

    pub fn is_attached(&mut self, is_attached: bool) -> &mut Self {
        self.inner.is_attached = is_attached;
        self
    }
}

impl AsRef<ClearRecentStickers> for ClearRecentStickers {
    fn as_ref(&self) -> &ClearRecentStickers {
        self
    }
}

impl AsRef<ClearRecentStickers> for RTDClearRecentStickersBuilder {
    fn as_ref(&self) -> &ClearRecentStickers {
        &self.inner
    }
}
