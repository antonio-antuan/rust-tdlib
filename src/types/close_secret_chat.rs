use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Closes a secret chat, effectively transferring its state to secretChatStateClosed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseSecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Secret chat identifier
    secret_chat_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CloseSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CloseSecretChat {}

impl CloseSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCloseSecretChatBuilder {
        let mut inner = CloseSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "closeSecretChat".to_string();

        RTDCloseSecretChatBuilder { inner }
    }

    pub fn secret_chat_id(&self) -> i32 {
        self.secret_chat_id
    }
}

#[doc(hidden)]
pub struct RTDCloseSecretChatBuilder {
    inner: CloseSecretChat,
}

impl RTDCloseSecretChatBuilder {
    pub fn build(&self) -> CloseSecretChat {
        self.inner.clone()
    }

    pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
        self.inner.secret_chat_id = secret_chat_id;
        self
    }
}

impl AsRef<CloseSecretChat> for CloseSecretChat {
    fn as_ref(&self) -> &CloseSecretChat {
        self
    }
}

impl AsRef<CloseSecretChat> for RTDCloseSecretChatBuilder {
    fn as_ref(&self) -> &CloseSecretChat {
        &self.inner
    }
}
