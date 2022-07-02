use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CloseChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CloseChat {}

impl CloseChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CloseChatBuilder {
        let mut inner = CloseChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "closeChat".to_string();

        CloseChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct CloseChatBuilder {
    inner: CloseChat,
}

#[deprecated]
pub type RTDCloseChatBuilder = CloseChatBuilder;

impl CloseChatBuilder {
    pub fn build(&self) -> CloseChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<CloseChat> for CloseChat {
    fn as_ref(&self) -> &CloseChat {
        self
    }
}

impl AsRef<CloseChat> for CloseChatBuilder {
    fn as_ref(&self) -> &CloseChat {
        &self.inner
    }
}
