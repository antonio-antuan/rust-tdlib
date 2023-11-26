use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes usernames assigned to a user, a supergroup, or a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usernames {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of active usernames; the first one must be shown as the primary username. The order of active usernames can be changed with reorderActiveUsernames, reorderBotActiveUsernames or reorderSupergroupActiveUsernames

    #[serde(default)]
    active_usernames: Vec<String>,
    /// List of currently disabled usernames; the username can be activated with toggleUsernameIsActive, toggleBotUsernameIsActive, or toggleSupergroupUsernameIsActive

    #[serde(default)]
    disabled_usernames: Vec<String>,
    /// The active username, which can be changed with setUsername or setSupergroupUsername

    #[serde(default)]
    editable_username: String,
}

impl RObject for Usernames {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Usernames {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UsernamesBuilder {
        let mut inner = Usernames::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UsernamesBuilder { inner }
    }

    pub fn active_usernames(&self) -> &Vec<String> {
        &self.active_usernames
    }

    pub fn disabled_usernames(&self) -> &Vec<String> {
        &self.disabled_usernames
    }

    pub fn editable_username(&self) -> &String {
        &self.editable_username
    }
}

#[doc(hidden)]
pub struct UsernamesBuilder {
    inner: Usernames,
}

#[deprecated]
pub type RTDUsernamesBuilder = UsernamesBuilder;

impl UsernamesBuilder {
    pub fn build(&self) -> Usernames {
        self.inner.clone()
    }

    pub fn active_usernames(&mut self, active_usernames: Vec<String>) -> &mut Self {
        self.inner.active_usernames = active_usernames;
        self
    }

    pub fn disabled_usernames(&mut self, disabled_usernames: Vec<String>) -> &mut Self {
        self.inner.disabled_usernames = disabled_usernames;
        self
    }

    pub fn editable_username<T: AsRef<str>>(&mut self, editable_username: T) -> &mut Self {
        self.inner.editable_username = editable_username.as_ref().to_string();
        self
    }
}

impl AsRef<Usernames> for Usernames {
    fn as_ref(&self) -> &Usernames {
        self
    }
}

impl AsRef<Usernames> for UsernamesBuilder {
    fn as_ref(&self) -> &Usernames {
        &self.inner
    }
}
