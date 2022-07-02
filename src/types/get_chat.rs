use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a chat by its identifier, this is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChat {
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

impl RObject for GetChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChat {}

impl GetChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatBuilder {
        let mut inner = GetChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChat".to_string();

        GetChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatBuilder {
    inner: GetChat,
}

#[deprecated]
pub type RTDGetChatBuilder = GetChatBuilder;

impl GetChatBuilder {
    pub fn build(&self) -> GetChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChat> for GetChat {
    fn as_ref(&self) -> &GetChat {
        self
    }
}

impl AsRef<GetChat> for GetChatBuilder {
    fn as_ref(&self) -> &GetChat {
        &self.inner
    }
}
