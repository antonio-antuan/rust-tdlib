use crate::errors::Result;
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

    #[serde(default)]
    call_id: i32,
    /// The data

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendCallSignalingDataBuilder {
        let mut inner = SendCallSignalingData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCallSignalingData".to_string();

        SendCallSignalingDataBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct SendCallSignalingDataBuilder {
    inner: SendCallSignalingData,
}

#[deprecated]
pub type RTDSendCallSignalingDataBuilder = SendCallSignalingDataBuilder;

impl SendCallSignalingDataBuilder {
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

impl AsRef<SendCallSignalingData> for SendCallSignalingDataBuilder {
    fn as_ref(&self) -> &SendCallSignalingData {
        &self.inner
    }
}
