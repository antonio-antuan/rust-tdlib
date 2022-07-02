use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupCall {
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

impl RObject for GetGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGroupCall {}

impl GetGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetGroupCallBuilder {
        let mut inner = GetGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGroupCall".to_string();

        GetGroupCallBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct GetGroupCallBuilder {
    inner: GetGroupCall,
}

#[deprecated]
pub type RTDGetGroupCallBuilder = GetGroupCallBuilder;

impl GetGroupCallBuilder {
    pub fn build(&self) -> GetGroupCall {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<GetGroupCall> for GetGroupCall {
    fn as_ref(&self) -> &GetGroupCall {
        self
    }
}

impl AsRef<GetGroupCall> for GetGroupCallBuilder {
    fn as_ref(&self) -> &GetGroupCall {
        &self.inner
    }
}
