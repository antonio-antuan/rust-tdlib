use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes current user from chat members. Private and secret chats can't be left using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LeaveChat {
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

impl RObject for LeaveChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LeaveChat {}

impl LeaveChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLeaveChatBuilder {
        let mut inner = LeaveChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "leaveChat".to_string();

        RTDLeaveChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDLeaveChatBuilder {
    inner: LeaveChat,
}

impl RTDLeaveChatBuilder {
    pub fn build(&self) -> LeaveChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<LeaveChat> for LeaveChat {
    fn as_ref(&self) -> &LeaveChat {
        self
    }
}

impl AsRef<LeaveChat> for RTDLeaveChatBuilder {
    fn as_ref(&self) -> &LeaveChat {
        &self.inner
    }
}
