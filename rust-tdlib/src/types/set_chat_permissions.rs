use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatPermissions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// New non-administrator members permissions in the chat
    permissions: ChatPermissions,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatPermissions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatPermissions {}

impl SetChatPermissions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetChatPermissionsBuilder {
        let mut inner = SetChatPermissions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatPermissions".to_string();

        RTDSetChatPermissionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn permissions(&self) -> &ChatPermissions {
        &self.permissions
    }
}

#[doc(hidden)]
pub struct RTDSetChatPermissionsBuilder {
    inner: SetChatPermissions,
}

impl RTDSetChatPermissionsBuilder {
    pub fn build(&self) -> SetChatPermissions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn permissions<T: AsRef<ChatPermissions>>(&mut self, permissions: T) -> &mut Self {
        self.inner.permissions = permissions.as_ref().clone();
        self
    }
}

impl AsRef<SetChatPermissions> for SetChatPermissions {
    fn as_ref(&self) -> &SetChatPermissions {
        self
    }
}

impl AsRef<SetChatPermissions> for RTDSetChatPermissionsBuilder {
    fn as_ref(&self) -> &SetChatPermissions {
        &self.inner
    }
}
