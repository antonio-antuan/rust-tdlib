use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Disables all active non-editable usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisableAllSupergroupUsernames {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel

    #[serde(default)]
    supergroup_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DisableAllSupergroupUsernames {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DisableAllSupergroupUsernames {}

impl DisableAllSupergroupUsernames {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DisableAllSupergroupUsernamesBuilder {
        let mut inner = DisableAllSupergroupUsernames::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "disableAllSupergroupUsernames".to_string();

        DisableAllSupergroupUsernamesBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct DisableAllSupergroupUsernamesBuilder {
    inner: DisableAllSupergroupUsernames,
}

#[deprecated]
pub type RTDDisableAllSupergroupUsernamesBuilder = DisableAllSupergroupUsernamesBuilder;

impl DisableAllSupergroupUsernamesBuilder {
    pub fn build(&self) -> DisableAllSupergroupUsernames {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<DisableAllSupergroupUsernames> for DisableAllSupergroupUsernames {
    fn as_ref(&self) -> &DisableAllSupergroupUsernames {
        self
    }
}

impl AsRef<DisableAllSupergroupUsernames> for DisableAllSupergroupUsernamesBuilder {
    fn as_ref(&self) -> &DisableAllSupergroupUsernames {
        &self.inner
    }
}
