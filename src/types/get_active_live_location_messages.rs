use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all active live locations that need to be updated by the application. The list is persistent across application restarts only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetActiveLiveLocationMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetActiveLiveLocationMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetActiveLiveLocationMessages {}

impl GetActiveLiveLocationMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetActiveLiveLocationMessagesBuilder {
        let mut inner = GetActiveLiveLocationMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getActiveLiveLocationMessages".to_string();

        GetActiveLiveLocationMessagesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetActiveLiveLocationMessagesBuilder {
    inner: GetActiveLiveLocationMessages,
}

#[deprecated]
pub type RTDGetActiveLiveLocationMessagesBuilder = GetActiveLiveLocationMessagesBuilder;

impl GetActiveLiveLocationMessagesBuilder {
    pub fn build(&self) -> GetActiveLiveLocationMessages {
        self.inner.clone()
    }
}

impl AsRef<GetActiveLiveLocationMessages> for GetActiveLiveLocationMessages {
    fn as_ref(&self) -> &GetActiveLiveLocationMessages {
        self
    }
}

impl AsRef<GetActiveLiveLocationMessages> for GetActiveLiveLocationMessagesBuilder {
    fn as_ref(&self) -> &GetActiveLiveLocationMessages {
        &self.inner
    }
}
