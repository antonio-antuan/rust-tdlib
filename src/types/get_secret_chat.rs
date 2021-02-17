use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a secret chat by its identifier. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSecretChat {
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

impl RObject for GetSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSecretChat {}

impl GetSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSecretChatBuilder {
        let mut inner = GetSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSecretChat".to_string();

        RTDGetSecretChatBuilder { inner }
    }

    pub fn secret_chat_id(&self) -> i32 {
        self.secret_chat_id
    }
}

#[doc(hidden)]
pub struct RTDGetSecretChatBuilder {
    inner: GetSecretChat,
}

impl RTDGetSecretChatBuilder {
    pub fn build(&self) -> GetSecretChat {
        self.inner.clone()
    }

    pub fn secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self {
        self.inner.secret_chat_id = secret_chat_id;
        self
    }
}

impl AsRef<GetSecretChat> for GetSecretChat {
    fn as_ref(&self) -> &GetSecretChat {
        self
    }
}

impl AsRef<GetSecretChat> for RTDGetSecretChatBuilder {
    fn as_ref(&self) -> &GetSecretChat {
        &self.inner
    }
}
