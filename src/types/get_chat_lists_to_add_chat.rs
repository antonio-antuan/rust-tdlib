use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns chat lists to which the chat can be added. This is an offline request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatListsToAddChat {
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

impl RObject for GetChatListsToAddChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatListsToAddChat {}

impl GetChatListsToAddChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatListsToAddChatBuilder {
        let mut inner = GetChatListsToAddChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatListsToAddChat".to_string();

        GetChatListsToAddChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatListsToAddChatBuilder {
    inner: GetChatListsToAddChat,
}

#[deprecated]
pub type RTDGetChatListsToAddChatBuilder = GetChatListsToAddChatBuilder;

impl GetChatListsToAddChatBuilder {
    pub fn build(&self) -> GetChatListsToAddChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatListsToAddChat> for GetChatListsToAddChat {
    fn as_ref(&self) -> &GetChatListsToAddChat {
        self
    }
}

impl AsRef<GetChatListsToAddChat> for GetChatListsToAddChatBuilder {
    fn as_ref(&self) -> &GetChatListsToAddChat {
        &self.inner
    }
}
