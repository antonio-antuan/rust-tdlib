use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether the current user can send a story on behalf of a chat; requires can_post_stories rights for channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStory {
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

impl RObject for CanSendStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStory {}

impl RFunction for CanSendStory {}

impl CanSendStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryBuilder {
        let mut inner = CanSendStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "canSendStory".to_string();

        CanSendStoryBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct CanSendStoryBuilder {
    inner: CanSendStory,
}

#[deprecated]
pub type RTDCanSendStoryBuilder = CanSendStoryBuilder;

impl CanSendStoryBuilder {
    pub fn build(&self) -> CanSendStory {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<CanSendStory> for CanSendStory {
    fn as_ref(&self) -> &CanSendStory {
        self
    }
}

impl AsRef<CanSendStory> for CanSendStoryBuilder {
    fn as_ref(&self) -> &CanSendStory {
        &self.inner
    }
}
