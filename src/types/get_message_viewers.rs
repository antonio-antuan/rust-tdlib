use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns viewers of a recent outgoing message in a basic group or a supergroup chat. For video notes and voice notes only users, opened content of the message, are returned. The method can be called if message.can_get_viewers == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageViewers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageViewers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageViewers {}

impl GetMessageViewers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageViewersBuilder {
        let mut inner = GetMessageViewers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageViewers".to_string();

        GetMessageViewersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct GetMessageViewersBuilder {
    inner: GetMessageViewers,
}

#[deprecated]
pub type RTDGetMessageViewersBuilder = GetMessageViewersBuilder;

impl GetMessageViewersBuilder {
    pub fn build(&self) -> GetMessageViewers {
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
}

impl AsRef<GetMessageViewers> for GetMessageViewers {
    fn as_ref(&self) -> &GetMessageViewers {
        self
    }
}

impl AsRef<GetMessageViewers> for GetMessageViewersBuilder {
    fn as_ref(&self) -> &GetMessageViewers {
        &self.inner
    }
}
