use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a chat administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAdministrator {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the administrator

    #[serde(default)]
    user_id: i64,
    /// Custom title of the administrator

    #[serde(default)]
    custom_title: String,
    /// True, if the user is the owner of the chat

    #[serde(default)]
    is_owner: bool,
}

impl RObject for ChatAdministrator {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatAdministrator {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatAdministratorBuilder {
        let mut inner = ChatAdministrator::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatAdministratorBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn custom_title(&self) -> &String {
        &self.custom_title
    }

    pub fn is_owner(&self) -> bool {
        self.is_owner
    }
}

#[doc(hidden)]
pub struct ChatAdministratorBuilder {
    inner: ChatAdministrator,
}

#[deprecated]
pub type RTDChatAdministratorBuilder = ChatAdministratorBuilder;

impl ChatAdministratorBuilder {
    pub fn build(&self) -> ChatAdministrator {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn custom_title<T: AsRef<str>>(&mut self, custom_title: T) -> &mut Self {
        self.inner.custom_title = custom_title.as_ref().to_string();
        self
    }

    pub fn is_owner(&mut self, is_owner: bool) -> &mut Self {
        self.inner.is_owner = is_owner;
        self
    }
}

impl AsRef<ChatAdministrator> for ChatAdministrator {
    fn as_ref(&self) -> &ChatAdministrator {
        self
    }
}

impl AsRef<ChatAdministrator> for ChatAdministratorBuilder {
    fn as_ref(&self) -> &ChatAdministrator {
        &self.inner
    }
}
