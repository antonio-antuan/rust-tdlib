use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Clears the list of recently found chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearRecentlyFoundChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearRecentlyFoundChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearRecentlyFoundChats {}

impl ClearRecentlyFoundChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDClearRecentlyFoundChatsBuilder {
        let mut inner = ClearRecentlyFoundChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearRecentlyFoundChats".to_string();

        RTDClearRecentlyFoundChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDClearRecentlyFoundChatsBuilder {
    inner: ClearRecentlyFoundChats,
}

impl RTDClearRecentlyFoundChatsBuilder {
    pub fn build(&self) -> ClearRecentlyFoundChats {
        self.inner.clone()
    }
}

impl AsRef<ClearRecentlyFoundChats> for ClearRecentlyFoundChats {
    fn as_ref(&self) -> &ClearRecentlyFoundChats {
        self
    }
}

impl AsRef<ClearRecentlyFoundChats> for RTDClearRecentlyFoundChatsBuilder {
    fn as_ref(&self) -> &ClearRecentlyFoundChats {
        &self.inner
    }
}
