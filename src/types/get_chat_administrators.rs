use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of administrators of the chat with their custom titles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatAdministrators {
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

impl RObject for GetChatAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatAdministrators {}

impl GetChatAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatAdministratorsBuilder {
        let mut inner = GetChatAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatAdministrators".to_string();

        RTDGetChatAdministratorsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDGetChatAdministratorsBuilder {
    inner: GetChatAdministrators,
}

impl RTDGetChatAdministratorsBuilder {
    pub fn build(&self) -> GetChatAdministrators {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatAdministrators> for GetChatAdministrators {
    fn as_ref(&self) -> &GetChatAdministrators {
        self
    }
}

impl AsRef<GetChatAdministrators> for RTDGetChatAdministratorsBuilder {
    fn as_ref(&self) -> &GetChatAdministrators {
        &self.inner
    }
}
