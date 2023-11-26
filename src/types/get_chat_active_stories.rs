use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the list of active stories posted by the given chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatActiveStories {
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

impl RObject for GetChatActiveStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatActiveStories {}

impl GetChatActiveStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatActiveStoriesBuilder {
        let mut inner = GetChatActiveStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatActiveStories".to_string();

        GetChatActiveStoriesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetChatActiveStoriesBuilder {
    inner: GetChatActiveStories,
}

#[deprecated]
pub type RTDGetChatActiveStoriesBuilder = GetChatActiveStoriesBuilder;

impl GetChatActiveStoriesBuilder {
    pub fn build(&self) -> GetChatActiveStories {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetChatActiveStories> for GetChatActiveStories {
    fn as_ref(&self) -> &GetChatActiveStories {
        self
    }
}

impl AsRef<GetChatActiveStories> for GetChatActiveStoriesBuilder {
    fn as_ref(&self) -> &GetChatActiveStories {
        &self.inner
    }
}
