use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Accepts an incoming call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier

    #[serde(default)]
    call_id: i32,
    /// The call protocols supported by the application
    protocol: CallProtocol,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AcceptCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AcceptCall {}

impl AcceptCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AcceptCallBuilder {
        let mut inner = AcceptCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "acceptCall".to_string();

        AcceptCallBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn protocol(&self) -> &CallProtocol {
        &self.protocol
    }
}

#[doc(hidden)]
pub struct AcceptCallBuilder {
    inner: AcceptCall,
}

#[deprecated]
pub type RTDAcceptCallBuilder = AcceptCallBuilder;

impl AcceptCallBuilder {
    pub fn build(&self) -> AcceptCall {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
        self.inner.protocol = protocol.as_ref().clone();
        self
    }
}

impl AsRef<AcceptCall> for AcceptCall {
    fn as_ref(&self) -> &AcceptCall {
        self
    }
}

impl AsRef<AcceptCall> for AcceptCallBuilder {
    fn as_ref(&self) -> &AcceptCall {
        &self.inner
    }
}
