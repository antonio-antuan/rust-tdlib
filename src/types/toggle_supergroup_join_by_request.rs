use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether all users directly joining the supergroup need to be approved by supergroup administrators; requires can_restrict_members administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupJoinByRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel

    #[serde(default)]
    supergroup_id: i64,
    /// New value of join_by_request

    #[serde(default)]
    join_by_request: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupJoinByRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupJoinByRequest {}

impl ToggleSupergroupJoinByRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupJoinByRequestBuilder {
        let mut inner = ToggleSupergroupJoinByRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupJoinByRequest".to_string();

        ToggleSupergroupJoinByRequestBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn join_by_request(&self) -> bool {
        self.join_by_request
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupJoinByRequestBuilder {
    inner: ToggleSupergroupJoinByRequest,
}

#[deprecated]
pub type RTDToggleSupergroupJoinByRequestBuilder = ToggleSupergroupJoinByRequestBuilder;

impl ToggleSupergroupJoinByRequestBuilder {
    pub fn build(&self) -> ToggleSupergroupJoinByRequest {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn join_by_request(&mut self, join_by_request: bool) -> &mut Self {
        self.inner.join_by_request = join_by_request;
        self
    }
}

impl AsRef<ToggleSupergroupJoinByRequest> for ToggleSupergroupJoinByRequest {
    fn as_ref(&self) -> &ToggleSupergroupJoinByRequest {
        self
    }
}

impl AsRef<ToggleSupergroupJoinByRequest> for ToggleSupergroupJoinByRequestBuilder {
    fn as_ref(&self) -> &ToggleSupergroupJoinByRequest {
        &self.inner
    }
}
