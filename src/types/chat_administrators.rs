use crate::errors::Result;
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatAdministratorsBuilder {
        let mut inner = ChatAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatAdministratorsBuilder { inner }
    }

    pub fn administrators(&self) -> &Vec<ChatAdministrator> {
        &self.administrators
    }
}

#[doc(hidden)]
pub struct ChatAdministratorsBuilder {
    inner: ChatAdministrators,
}

#[deprecated]
pub type RTDChatAdministratorsBuilder = ChatAdministratorsBuilder;

impl ChatAdministratorsBuilder {
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

impl AsRef<ChatAdministrators> for ChatAdministratorsBuilder {
    fn as_ref(&self) -> &ChatAdministrators {
        &self.inner
    }
}
