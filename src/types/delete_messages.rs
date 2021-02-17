use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Identifiers of the messages to be deleted
    message_ids: Vec<i64>,
    /// Pass true to try to delete messages for all chat members. Always true for supergroups, channels and secret chats
    revoke: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteMessages {}

impl DeleteMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteMessagesBuilder {
        let mut inner = DeleteMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteMessages".to_string();

        RTDDeleteMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn revoke(&self) -> bool {
        self.revoke
    }
}

#[doc(hidden)]
pub struct RTDDeleteMessagesBuilder {
    inner: DeleteMessages,
}

impl RTDDeleteMessagesBuilder {
    pub fn build(&self) -> DeleteMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }

    pub fn revoke(&mut self, revoke: bool) -> &mut Self {
        self.inner.revoke = revoke;
        self
    }
}

impl AsRef<DeleteMessages> for DeleteMessages {
    fn as_ref(&self) -> &DeleteMessages {
        self
    }
}

impl AsRef<DeleteMessages> for RTDDeleteMessagesBuilder {
    fn as_ref(&self) -> &DeleteMessages {
        &self.inner
    }
}
