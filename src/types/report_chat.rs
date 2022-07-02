use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, or if chat.can_be_reported
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifiers of reported messages, if any

    #[serde(default)]
    message_ids: Vec<i64>,
    /// The reason for reporting the chat

    #[serde(skip_serializing_if = "ChatReportReason::_is_default")]
    reason: ChatReportReason,
    /// Additional report details; 0-1024 characters

    #[serde(default)]
    text: String,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportChatBuilder {
        let mut inner = ReportChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportChat".to_string();

        ReportChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }

    pub fn reason(&self) -> &ChatReportReason {
        &self.reason
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct ReportChatBuilder {
    inner: ReportChat,
}

#[deprecated]
pub type RTDReportChatBuilder = ReportChatBuilder;

impl ReportChatBuilder {
    pub fn build(&self) -> ReportChat {
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

    pub fn reason<T: AsRef<ChatReportReason>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().clone();
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ReportChat> for ReportChat {
    fn as_ref(&self) -> &ReportChat {
        self
    }
}

impl AsRef<ReportChat> for ReportChatBuilder {
    fn as_ref(&self) -> &ReportChat {
        &self.inner
    }
}
