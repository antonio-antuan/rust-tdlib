use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a new secret chat. Returns the newly created chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewSecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target user
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateNewSecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateNewSecretChat {}

impl CreateNewSecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateNewSecretChatBuilder {
        let mut inner = CreateNewSecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewSecretChat".to_string();

        RTDCreateNewSecretChatBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDCreateNewSecretChatBuilder {
    inner: CreateNewSecretChat,
}

impl RTDCreateNewSecretChatBuilder {
    pub fn build(&self) -> CreateNewSecretChat {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<CreateNewSecretChat> for CreateNewSecretChat {
    fn as_ref(&self) -> &CreateNewSecretChat {
        self
    }
}

impl AsRef<CreateNewSecretChat> for RTDCreateNewSecretChatBuilder {
    fn as_ref(&self) -> &CreateNewSecretChat {
        &self.inner
    }
}
