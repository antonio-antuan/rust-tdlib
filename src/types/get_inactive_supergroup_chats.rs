use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInactiveSupergroupChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInactiveSupergroupChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInactiveSupergroupChats {}

impl GetInactiveSupergroupChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetInactiveSupergroupChatsBuilder {
        let mut inner = GetInactiveSupergroupChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInactiveSupergroupChats".to_string();

        GetInactiveSupergroupChatsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetInactiveSupergroupChatsBuilder {
    inner: GetInactiveSupergroupChats,
}

#[deprecated]
pub type RTDGetInactiveSupergroupChatsBuilder = GetInactiveSupergroupChatsBuilder;

impl GetInactiveSupergroupChatsBuilder {
    pub fn build(&self) -> GetInactiveSupergroupChats {
        self.inner.clone()
    }
}

impl AsRef<GetInactiveSupergroupChats> for GetInactiveSupergroupChats {
    fn as_ref(&self) -> &GetInactiveSupergroupChats {
        self
    }
}

impl AsRef<GetInactiveSupergroupChats> for GetInactiveSupergroupChatsBuilder {
    fn as_ref(&self) -> &GetInactiveSupergroupChats {
        &self.inner
    }
}
