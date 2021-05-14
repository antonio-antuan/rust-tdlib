use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a custom request; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCustomRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The method name
    method: String,
    /// JSON-serialized method parameters
    parameters: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendCustomRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendCustomRequest {}

impl SendCustomRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendCustomRequestBuilder {
        let mut inner = SendCustomRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCustomRequest".to_string();

        RTDSendCustomRequestBuilder { inner }
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn parameters(&self) -> &String {
        &self.parameters
    }
}

#[doc(hidden)]
pub struct RTDSendCustomRequestBuilder {
    inner: SendCustomRequest,
}

impl RTDSendCustomRequestBuilder {
    pub fn build(&self) -> SendCustomRequest {
        self.inner.clone()
    }

    pub fn method<T: AsRef<str>>(&mut self, method: T) -> &mut Self {
        self.inner.method = method.as_ref().to_string();
        self
    }

    pub fn parameters<T: AsRef<str>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().to_string();
        self
    }
}

impl AsRef<SendCustomRequest> for SendCustomRequest {
    fn as_ref(&self) -> &SendCustomRequest {
        self
    }
}

impl AsRef<SendCustomRequest> for RTDSendCustomRequestBuilder {
    fn as_ref(&self) -> &SendCustomRequest {
        &self.inner
    }
}
