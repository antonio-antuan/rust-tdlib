use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds current user as a new member to a chat. Private and secret chats can't be joined using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JoinChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for JoinChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for JoinChat {}

impl JoinChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJoinChatBuilder {
        let mut inner = JoinChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "joinChat".to_string();

        RTDJoinChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDJoinChatBuilder {
    inner: JoinChat,
}

impl RTDJoinChatBuilder {
    pub fn build(&self) -> JoinChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<JoinChat> for JoinChat {
    fn as_ref(&self) -> &JoinChat {
        self
    }
}

impl AsRef<JoinChat> for RTDJoinChatBuilder {
    fn as_ref(&self) -> &JoinChat {
        &self.inner
    }
}
