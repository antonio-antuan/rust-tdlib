use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes order of active usernames of a supergroup or channel, requires owner privileges in the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderSupergroupActiveUsernames {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel

    #[serde(default)]
    supergroup_id: i64,
    /// The new order of active usernames. All currently active usernames must be specified

    #[serde(default)]
    usernames: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReorderSupergroupActiveUsernames {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReorderSupergroupActiveUsernames {}

impl ReorderSupergroupActiveUsernames {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReorderSupergroupActiveUsernamesBuilder {
        let mut inner = ReorderSupergroupActiveUsernames::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderSupergroupActiveUsernames".to_string();

        ReorderSupergroupActiveUsernamesBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn usernames(&self) -> &Vec<String> {
        &self.usernames
    }
}

#[doc(hidden)]
pub struct ReorderSupergroupActiveUsernamesBuilder {
    inner: ReorderSupergroupActiveUsernames,
}

#[deprecated]
pub type RTDReorderSupergroupActiveUsernamesBuilder = ReorderSupergroupActiveUsernamesBuilder;

impl ReorderSupergroupActiveUsernamesBuilder {
    pub fn build(&self) -> ReorderSupergroupActiveUsernames {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn usernames(&mut self, usernames: Vec<String>) -> &mut Self {
        self.inner.usernames = usernames;
        self
    }
}

impl AsRef<ReorderSupergroupActiveUsernames> for ReorderSupergroupActiveUsernames {
    fn as_ref(&self) -> &ReorderSupergroupActiveUsernames {
        self
    }
}

impl AsRef<ReorderSupergroupActiveUsernames> for ReorderSupergroupActiveUsernamesBuilder {
    fn as_ref(&self) -> &ReorderSupergroupActiveUsernames {
        &self.inner
    }
}
