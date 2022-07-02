use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckAuthenticationBotToken {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The bot token

    #[serde(default)]
    token: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckAuthenticationBotToken {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckAuthenticationBotToken {}

impl CheckAuthenticationBotToken {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckAuthenticationBotTokenBuilder {
        let mut inner = CheckAuthenticationBotToken::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkAuthenticationBotToken".to_string();

        CheckAuthenticationBotTokenBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct CheckAuthenticationBotTokenBuilder {
    inner: CheckAuthenticationBotToken,
}

#[deprecated]
pub type RTDCheckAuthenticationBotTokenBuilder = CheckAuthenticationBotTokenBuilder;

impl CheckAuthenticationBotTokenBuilder {
    pub fn build(&self) -> CheckAuthenticationBotToken {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<CheckAuthenticationBotToken> for CheckAuthenticationBotToken {
    fn as_ref(&self) -> &CheckAuthenticationBotToken {
        self
    }
}

impl AsRef<CheckAuthenticationBotToken> for CheckAuthenticationBotTokenBuilder {
    fn as_ref(&self) -> &CheckAuthenticationBotToken {
        &self.inner
    }
}
