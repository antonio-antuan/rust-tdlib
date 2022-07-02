use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Leaves a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveGroupCall {
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

impl RObject for LeaveGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LeaveGroupCall {}

impl LeaveGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LeaveGroupCallBuilder {
        let mut inner = LeaveGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "leaveGroupCall".to_string();

        LeaveGroupCallBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct LeaveGroupCallBuilder {
    inner: LeaveGroupCall,
}

#[deprecated]
pub type RTDLeaveGroupCallBuilder = LeaveGroupCallBuilder;

impl LeaveGroupCallBuilder {
    pub fn build(&self) -> LeaveGroupCall {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<LeaveGroupCall> for LeaveGroupCall {
    fn as_ref(&self) -> &LeaveGroupCall {
        self
    }
}

impl AsRef<LeaveGroupCall> for LeaveGroupCallBuilder {
    fn as_ref(&self) -> &LeaveGroupCall {
        &self.inner
    }
}
