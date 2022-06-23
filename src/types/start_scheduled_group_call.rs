use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Starts a scheduled group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartScheduledGroupCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for StartScheduledGroupCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for StartScheduledGroupCall {}

impl StartScheduledGroupCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStartScheduledGroupCallBuilder {
        let mut inner = StartScheduledGroupCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "startScheduledGroupCall".to_string();

        RTDStartScheduledGroupCallBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct RTDStartScheduledGroupCallBuilder {
    inner: StartScheduledGroupCall,
}

impl RTDStartScheduledGroupCallBuilder {
    pub fn build(&self) -> StartScheduledGroupCall {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<StartScheduledGroupCall> for StartScheduledGroupCall {
    fn as_ref(&self) -> &StartScheduledGroupCall {
        self
    }
}

impl AsRef<StartScheduledGroupCall> for RTDStartScheduledGroupCallBuilder {
    fn as_ref(&self) -> &StartScheduledGroupCall {
        &self.inner
    }
}
