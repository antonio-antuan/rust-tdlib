use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes content and caption of a story. Can be called only if story.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// Identifier of the story to edit

    #[serde(default)]
    story_id: i32,
    /// New content of the story; pass null to keep the current content

    #[serde(skip_serializing_if = "InputStoryContent::_is_default")]
    content: InputStoryContent,
    /// New clickable rectangle areas to be shown on the story media; pass null to keep the current areas. Areas can't be edited if story content isn't changed
    areas: InputStoryAreas,
    /// New story caption; pass null to keep the current caption
    caption: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditStory {}

impl EditStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditStoryBuilder {
        let mut inner = EditStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editStory".to_string();

        EditStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn content(&self) -> &InputStoryContent {
        &self.content
    }

    pub fn areas(&self) -> &InputStoryAreas {
        &self.areas
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct EditStoryBuilder {
    inner: EditStory,
}

#[deprecated]
pub type RTDEditStoryBuilder = EditStoryBuilder;

impl EditStoryBuilder {
    pub fn build(&self) -> EditStory {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn content<T: AsRef<InputStoryContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }

    pub fn areas<T: AsRef<InputStoryAreas>>(&mut self, areas: T) -> &mut Self {
        self.inner.areas = areas.as_ref().clone();
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<EditStory> for EditStory {
    fn as_ref(&self) -> &EditStory {
        self
    }
}

impl AsRef<EditStory> for EditStoryBuilder {
    fn as_ref(&self) -> &EditStory {
        &self.inner
    }
}
