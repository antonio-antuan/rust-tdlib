use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes active state for a username of the current user. The editable username can't be disabled. May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleUsernameIsActive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The username to change

    #[serde(default)]
    username: String,
    /// Pass true to activate the username; pass false to disable it

    #[serde(default)]
    is_active: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleUsernameIsActive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleUsernameIsActive {}

impl ToggleUsernameIsActive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleUsernameIsActiveBuilder {
        let mut inner = ToggleUsernameIsActive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleUsernameIsActive".to_string();

        ToggleUsernameIsActiveBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[doc(hidden)]
pub struct ToggleUsernameIsActiveBuilder {
    inner: ToggleUsernameIsActive,
}

#[deprecated]
pub type RTDToggleUsernameIsActiveBuilder = ToggleUsernameIsActiveBuilder;

impl ToggleUsernameIsActiveBuilder {
    pub fn build(&self) -> ToggleUsernameIsActive {
        self.inner.clone()
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

impl AsRef<ToggleUsernameIsActive> for ToggleUsernameIsActive {
    fn as_ref(&self) -> &ToggleUsernameIsActive {
        self
    }
}

impl AsRef<ToggleUsernameIsActive> for ToggleUsernameIsActiveBuilder {
    fn as_ref(&self) -> &ToggleUsernameIsActive {
        &self.inner
    }
}
