use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns channel chats in which the current user has the right to post stories. The chats must be rechecked with canSendStory before actually trying to post a story there
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatsToSendStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatsToSendStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatsToSendStories {}

impl GetChatsToSendStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatsToSendStoriesBuilder {
        let mut inner = GetChatsToSendStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatsToSendStories".to_string();

        GetChatsToSendStoriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetChatsToSendStoriesBuilder {
    inner: GetChatsToSendStories,
}

#[deprecated]
pub type RTDGetChatsToSendStoriesBuilder = GetChatsToSendStoriesBuilder;

impl GetChatsToSendStoriesBuilder {
    pub fn build(&self) -> GetChatsToSendStories {
        self.inner.clone()
    }
}

impl AsRef<GetChatsToSendStories> for GetChatsToSendStories {
    fn as_ref(&self) -> &GetChatsToSendStories {
        self
    }
}

impl AsRef<GetChatsToSendStories> for GetChatsToSendStoriesBuilder {
    fn as_ref(&self) -> &GetChatsToSendStories {
        &self.inner
    }
}
