use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes active state for a username of a bot. The editable username can't be disabled. May return an error with a message "USERNAMES_ACTIVE_TOO_MUCH" if the maximum number of active usernames has been reached. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleBotUsernameIsActive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// The username to change

    #[serde(default)]
    username: String,
    /// Pass true to activate the username; pass false to disable it

    #[serde(default)]
    is_active: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleBotUsernameIsActive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleBotUsernameIsActive {}

impl ToggleBotUsernameIsActive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleBotUsernameIsActiveBuilder {
        let mut inner = ToggleBotUsernameIsActive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleBotUsernameIsActive".to_string();

        ToggleBotUsernameIsActiveBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[doc(hidden)]
pub struct ToggleBotUsernameIsActiveBuilder {
    inner: ToggleBotUsernameIsActive,
}

#[deprecated]
pub type RTDToggleBotUsernameIsActiveBuilder = ToggleBotUsernameIsActiveBuilder;

impl ToggleBotUsernameIsActiveBuilder {
    pub fn build(&self) -> ToggleBotUsernameIsActive {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
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

impl AsRef<ToggleBotUsernameIsActive> for ToggleBotUsernameIsActive {
    fn as_ref(&self) -> &ToggleBotUsernameIsActive {
        self
    }
}

impl AsRef<ToggleBotUsernameIsActive> for ToggleBotUsernameIsActiveBuilder {
    fn as_ref(&self) -> &ToggleBotUsernameIsActive {
        &self.inner
    }
}
