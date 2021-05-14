use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes a chat action bar without any other action
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveChatActionBar {
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

impl RObject for RemoveChatActionBar {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveChatActionBar {}

impl RemoveChatActionBar {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveChatActionBarBuilder {
        let mut inner = RemoveChatActionBar::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeChatActionBar".to_string();

        RTDRemoveChatActionBarBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDRemoveChatActionBarBuilder {
    inner: RemoveChatActionBar,
}

impl RTDRemoveChatActionBarBuilder {
    pub fn build(&self) -> RemoveChatActionBar {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<RemoveChatActionBar> for RemoveChatActionBar {
    fn as_ref(&self) -> &RemoveChatActionBar {
        self
    }
}

impl AsRef<RemoveChatActionBar> for RTDRemoveChatActionBarBuilder {
    fn as_ref(&self) -> &RemoveChatActionBar {
        &self.inner
    }
}
