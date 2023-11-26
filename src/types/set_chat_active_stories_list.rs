use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes story list in which stories from the chat are shown
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatActiveStoriesList {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted stories

    #[serde(default)]
    chat_id: i64,
    /// New list for active stories posted by the chat

    #[serde(skip_serializing_if = "StoryList::_is_default")]
    story_list: StoryList,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatActiveStoriesList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatActiveStoriesList {}

impl SetChatActiveStoriesList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatActiveStoriesListBuilder {
        let mut inner = SetChatActiveStoriesList::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatActiveStoriesList".to_string();

        SetChatActiveStoriesListBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn story_list(&self) -> &StoryList {
        &self.story_list
    }
}

#[doc(hidden)]
pub struct SetChatActiveStoriesListBuilder {
    inner: SetChatActiveStoriesList,
}

#[deprecated]
pub type RTDSetChatActiveStoriesListBuilder = SetChatActiveStoriesListBuilder;

impl SetChatActiveStoriesListBuilder {
    pub fn build(&self) -> SetChatActiveStoriesList {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn story_list<T: AsRef<StoryList>>(&mut self, story_list: T) -> &mut Self {
        self.inner.story_list = story_list.as_ref().clone();
        self
    }
}

impl AsRef<SetChatActiveStoriesList> for SetChatActiveStoriesList {
    fn as_ref(&self) -> &SetChatActiveStoriesList {
        self
    }
}

impl AsRef<SetChatActiveStoriesList> for SetChatActiveStoriesListBuilder {
    fn as_ref(&self) -> &SetChatActiveStoriesList {
        &self.inner
    }
}
