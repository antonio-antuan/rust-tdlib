use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenChat {
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

impl RObject for OpenChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for OpenChat {}

impl OpenChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OpenChatBuilder {
        let mut inner = OpenChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "openChat".to_string();

        OpenChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct OpenChatBuilder {
    inner: OpenChat,
}

#[deprecated]
pub type RTDOpenChatBuilder = OpenChatBuilder;

impl OpenChatBuilder {
    pub fn build(&self) -> OpenChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<OpenChat> for OpenChat {
    fn as_ref(&self) -> &OpenChat {
        self
    }
}

impl AsRef<OpenChat> for OpenChatBuilder {
    fn as_ref(&self) -> &OpenChat {
        &self.inner
    }
}
