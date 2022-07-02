use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Ends a group call. Requires groupCall.can_be_managed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EndGroupCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EndGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EndGroupCall {}

impl EndGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EndGroupCallBuilder {
        let mut inner = EndGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "endGroupCall".to_string();

        EndGroupCallBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct EndGroupCallBuilder {
    inner: EndGroupCall,
}

#[deprecated]
pub type RTDEndGroupCallBuilder = EndGroupCallBuilder;

impl EndGroupCallBuilder {
    pub fn build(&self) -> EndGroupCall {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<EndGroupCall> for EndGroupCall {
    fn as_ref(&self) -> &EndGroupCall {
        self
    }
}

impl AsRef<EndGroupCall> for EndGroupCallBuilder {
    fn as_ref(&self) -> &EndGroupCall {
        &self.inner
    }
}
