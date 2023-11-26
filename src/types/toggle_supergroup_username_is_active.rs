use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes active state for a username of a supergroup or channel, requires owner privileges in the supergroup or channel. The editable username can't be disabled. May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupUsernameIsActive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel

    #[serde(default)]
    supergroup_id: i64,
    /// The username to change

    #[serde(default)]
    username: String,
    /// Pass true to activate the username; pass false to disable it

    #[serde(default)]
    is_active: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupUsernameIsActive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupUsernameIsActive {}

impl ToggleSupergroupUsernameIsActive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupUsernameIsActiveBuilder {
        let mut inner = ToggleSupergroupUsernameIsActive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupUsernameIsActive".to_string();

        ToggleSupergroupUsernameIsActiveBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupUsernameIsActiveBuilder {
    inner: ToggleSupergroupUsernameIsActive,
}

#[deprecated]
pub type RTDToggleSupergroupUsernameIsActiveBuilder = ToggleSupergroupUsernameIsActiveBuilder;

impl ToggleSupergroupUsernameIsActiveBuilder {
    pub fn build(&self) -> ToggleSupergroupUsernameIsActive {
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

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.inner.is_active = is_active;
        self
    }
}

impl AsRef<ToggleSupergroupUsernameIsActive> for ToggleSupergroupUsernameIsActive {
    fn as_ref(&self) -> &ToggleSupergroupUsernameIsActive {
        self
    }
}

impl AsRef<ToggleSupergroupUsernameIsActive> for ToggleSupergroupUsernameIsActiveBuilder {
    fn as_ref(&self) -> &ToggleSupergroupUsernameIsActive {
        &self.inner
    }
}
