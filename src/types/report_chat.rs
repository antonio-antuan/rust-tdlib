use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if this is a private chats with a bot, a private chat with a user sharing their location, a supergroup, or a channel, since other chats can't be checked by moderators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// The reason for reporting the chat

    #[serde(skip_serializing_if = "ChatReportReason::_is_default")]
    reason: ChatReportReason,
    /// Identifiers of reported messages, if any
    message_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportChat {}

impl ReportChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReportChatBuilder {
        let mut inner = ReportChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportChat".to_string();

        RTDReportChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn reason(&self) -> &ChatReportReason {
        &self.reason
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }
}

#[doc(hidden)]
pub struct RTDReportChatBuilder {
    inner: ReportChat,
}

impl RTDReportChatBuilder {
    pub fn build(&self) -> ReportChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn reason<T: AsRef<ChatReportReason>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().clone();
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }
}

impl AsRef<ReportChat> for ReportChat {
    fn as_ref(&self) -> &ReportChat {
        self
    }
}

impl AsRef<ReportChat> for RTDReportChatBuilder {
    fn as_ref(&self) -> &ReportChat {
        &self.inner
    }
}
