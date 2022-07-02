use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches a public chat by its username. Currently, only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPublicChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username to be resolved

    #[serde(default)]
    username: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchPublicChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchPublicChat {}

impl SearchPublicChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchPublicChatBuilder {
        let mut inner = SearchPublicChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchPublicChat".to_string();

        SearchPublicChatBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[doc(hidden)]
pub struct SearchPublicChatBuilder {
    inner: SearchPublicChat,
}

#[deprecated]
pub type RTDSearchPublicChatBuilder = SearchPublicChatBuilder;

impl SearchPublicChatBuilder {
    pub fn build(&self) -> SearchPublicChat {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }
}

impl AsRef<SearchPublicChat> for SearchPublicChat {
    fn as_ref(&self) -> &SearchPublicChat {
        self
    }
}

impl AsRef<SearchPublicChat> for SearchPublicChatBuilder {
    fn as_ref(&self) -> &SearchPublicChat {
        &self.inner
    }
}
