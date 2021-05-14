use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an existing chat corresponding to a known secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSecretChat {
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

impl RObject for CreateSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateSecretChat {}

impl CreateSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateSecretChatBuilder {
        let mut inner = CreateSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createSecretChat".to_string();

        RTDCreateSecretChatBuilder { inner }
    }

    pub fn secret_chat_id(&self) -> i32 {
        self.secret_chat_id
    }
}

#[doc(hidden)]
pub struct RTDCreateSecretChatBuilder {
    inner: CreateSecretChat,
}

impl RTDCreateSecretChatBuilder {
    pub fn build(&self) -> CreateSecretChat {
        self.inner.clone()
    }

    pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
        self.inner.secret_chat_id = secret_chat_id;
        self
    }
}

impl AsRef<CreateSecretChat> for CreateSecretChat {
    fn as_ref(&self) -> &CreateSecretChat {
        self
    }
}

impl AsRef<CreateSecretChat> for RTDCreateSecretChatBuilder {
    fn as_ref(&self) -> &CreateSecretChat {
        &self.inner
    }
}
