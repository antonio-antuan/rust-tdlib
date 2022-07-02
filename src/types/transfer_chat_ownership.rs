use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferChatOwnership {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the user to which transfer the ownership. The ownership can't be transferred to a bot or to a deleted user

    #[serde(default)]
    user_id: i64,
    /// The password of the current user

    #[serde(default)]
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TransferChatOwnership {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TransferChatOwnership {}

impl TransferChatOwnership {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TransferChatOwnershipBuilder {
        let mut inner = TransferChatOwnership::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "transferChatOwnership".to_string();

        TransferChatOwnershipBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct TransferChatOwnershipBuilder {
    inner: TransferChatOwnership,
}

#[deprecated]
pub type RTDTransferChatOwnershipBuilder = TransferChatOwnershipBuilder;

impl TransferChatOwnershipBuilder {
    pub fn build(&self) -> TransferChatOwnership {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<TransferChatOwnership> for TransferChatOwnership {
    fn as_ref(&self) -> &TransferChatOwnership {
        self
    }
}

impl AsRef<TransferChatOwnership> for TransferChatOwnershipBuilder {
    fn as_ref(&self) -> &TransferChatOwnership {
        &self.inner
    }
}
