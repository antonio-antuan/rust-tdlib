use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Clears the list of recently used emoji statuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentEmojiStatuses {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearRecentEmojiStatuses {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearRecentEmojiStatuses {}

impl ClearRecentEmojiStatuses {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClearRecentEmojiStatusesBuilder {
        let mut inner = ClearRecentEmojiStatuses::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearRecentEmojiStatuses".to_string();

        ClearRecentEmojiStatusesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ClearRecentEmojiStatusesBuilder {
    inner: ClearRecentEmojiStatuses,
}

#[deprecated]
pub type RTDClearRecentEmojiStatusesBuilder = ClearRecentEmojiStatusesBuilder;

impl ClearRecentEmojiStatusesBuilder {
    pub fn build(&self) -> ClearRecentEmojiStatuses {
        self.inner.clone()
    }
}

impl AsRef<ClearRecentEmojiStatuses> for ClearRecentEmojiStatuses {
    fn as_ref(&self) -> &ClearRecentEmojiStatuses {
        self
    }
}

impl AsRef<ClearRecentEmojiStatuses> for ClearRecentEmojiStatusesBuilder {
    fn as_ref(&self) -> &ClearRecentEmojiStatuses {
        &self.inner
    }
}
