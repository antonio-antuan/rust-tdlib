use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports reactions set on a message to the Telegram moderators. Reactions on a message can be reported only if message.can_report_reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportMessageReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Identifier of the sender, which added the reaction

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    sender_id: MessageSender,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportMessageReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportMessageReactions {}

impl ReportMessageReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportMessageReactionsBuilder {
        let mut inner = ReportMessageReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportMessageReactions".to_string();

        ReportMessageReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn sender_id(&self) -> &MessageSender {
        &self.sender_id
    }
}

#[doc(hidden)]
pub struct ReportMessageReactionsBuilder {
    inner: ReportMessageReactions,
}

#[deprecated]
pub type RTDReportMessageReactionsBuilder = ReportMessageReactionsBuilder;

impl ReportMessageReactionsBuilder {
    pub fn build(&self) -> ReportMessageReactions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
        self.inner.sender_id = sender_id.as_ref().clone();
        self
    }
}

impl AsRef<ReportMessageReactions> for ReportMessageReactions {
    fn as_ref(&self) -> &ReportMessageReactions {
        self
    }
}

impl AsRef<ReportMessageReactions> for ReportMessageReactionsBuilder {
    fn as_ref(&self) -> &ReportMessageReactions {
        &self.inner
    }
}
