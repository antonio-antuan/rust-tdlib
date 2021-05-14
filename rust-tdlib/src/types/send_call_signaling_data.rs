use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends call signaling data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallSignalingData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier
    call_id: i32,
    /// The data
    data: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendCallSignalingData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendCallSignalingData {}

impl SendCallSignalingData {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendCallSignalingDataBuilder {
        let mut inner = SendCallSignalingData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCallSignalingData".to_string();

        RTDSendCallSignalingDataBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDSendCallSignalingDataBuilder {
    inner: SendCallSignalingData,
}

impl RTDSendCallSignalingDataBuilder {
    pub fn build(&self) -> SendCallSignalingData {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<SendCallSignalingData> for SendCallSignalingData {
    fn as_ref(&self) -> &SendCallSignalingData {
        self
    }
}

impl AsRef<SendCallSignalingData> for RTDSendCallSignalingDataBuilder {
    fn as_ref(&self) -> &SendCallSignalingData {
        &self.inner
    }
}
