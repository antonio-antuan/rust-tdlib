use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSupergroupUsername {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel
    supergroup_id: i64,
    /// New value of the username. Use an empty string to remove the username
    username: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetSupergroupUsername {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetSupergroupUsername {}

impl SetSupergroupUsername {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetSupergroupUsernameBuilder {
        let mut inner = SetSupergroupUsername::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setSupergroupUsername".to_string();

        RTDSetSupergroupUsernameBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[doc(hidden)]
pub struct RTDSetSupergroupUsernameBuilder {
    inner: SetSupergroupUsername,
}

impl RTDSetSupergroupUsernameBuilder {
    pub fn build(&self) -> SetSupergroupUsername {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }
}

impl AsRef<SetSupergroupUsername> for SetSupergroupUsername {
    fn as_ref(&self) -> &SetSupergroupUsername {
        self
    }
}

impl AsRef<SetSupergroupUsername> for RTDSetSupergroupUsernameBuilder {
    fn as_ref(&self) -> &SetSupergroupUsername {
        &self.inner
    }
}
