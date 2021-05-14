use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of chat administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of chat administrators
    administrators: Vec<ChatAdministrator>,
}

impl RObject for ChatAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatAdministratorsBuilder {
        let mut inner = ChatAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatAdministratorsBuilder { inner }
    }

    pub fn administrators(&self) -> &Vec<ChatAdministrator> {
        &self.administrators
    }
}

#[doc(hidden)]
pub struct RTDChatAdministratorsBuilder {
    inner: ChatAdministrators,
}

impl RTDChatAdministratorsBuilder {
    pub fn build(&self) -> ChatAdministrators {
        self.inner.clone()
    }

    pub fn administrators(&mut self, administrators: Vec<ChatAdministrator>) -> &mut Self {
        self.inner.administrators = administrators;
        self
    }
}

impl AsRef<ChatAdministrators> for ChatAdministrators {
    fn as_ref(&self) -> &ChatAdministrators {
        self
    }
}

impl AsRef<ChatAdministrators> for RTDChatAdministratorsBuilder {
    fn as_ref(&self) -> &ChatAdministrators {
        &self.inner
    }
}
