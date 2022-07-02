use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPushReceiverId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded push notification payload

    #[serde(default)]
    payload: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPushReceiverId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPushReceiverId {}

impl GetPushReceiverId {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPushReceiverIdBuilder {
        let mut inner = GetPushReceiverId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPushReceiverId".to_string();

        GetPushReceiverIdBuilder { inner }
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }
}

#[doc(hidden)]
pub struct GetPushReceiverIdBuilder {
    inner: GetPushReceiverId,
}

#[deprecated]
pub type RTDGetPushReceiverIdBuilder = GetPushReceiverIdBuilder;

impl GetPushReceiverIdBuilder {
    pub fn build(&self) -> GetPushReceiverId {
        self.inner.clone()
    }

    pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().to_string();
        self
    }
}

impl AsRef<GetPushReceiverId> for GetPushReceiverId {
    fn as_ref(&self) -> &GetPushReceiverId {
        self
    }
}

impl AsRef<GetPushReceiverId> for GetPushReceiverIdBuilder {
    fn as_ref(&self) -> &GetPushReceiverId {
        &self.inner
    }
}
