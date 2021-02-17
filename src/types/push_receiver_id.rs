use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushReceiverId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The globally unique identifier of push notification subscription

    #[serde(deserialize_with = "super::_common::number_from_string")]
    id: i64,
}

impl RObject for PushReceiverId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PushReceiverId {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushReceiverIdBuilder {
        let mut inner = PushReceiverId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushReceiverIdBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

#[doc(hidden)]
pub struct RTDPushReceiverIdBuilder {
    inner: PushReceiverId,
}

impl RTDPushReceiverIdBuilder {
    pub fn build(&self) -> PushReceiverId {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }
}

impl AsRef<PushReceiverId> for PushReceiverId {
    fn as_ref(&self) -> &PushReceiverId {
        self
    }
}

impl AsRef<PushReceiverId> for RTDPushReceiverIdBuilder {
    fn as_ref(&self) -> &PushReceiverId {
        &self.inner
    }
}
