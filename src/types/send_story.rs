use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a new story to a chat; requires can_post_stories rights for channel chats. Returns a temporary story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that will post the story

    #[serde(default)]
    chat_id: i64,
    /// Content of the story

    #[serde(skip_serializing_if = "InputStoryContent::_is_default")]
    content: InputStoryContent,
    /// Clickable rectangle areas to be shown on the story media; pass null if none
    areas: InputStoryAreas,
    /// Story caption; pass null to use an empty caption; 0-getOption("story_caption_length_max") characters
    caption: FormattedText,
    /// The privacy settings for the story

    #[serde(skip_serializing_if = "StoryPrivacySettings::_is_default")]
    privacy_settings: StoryPrivacySettings,
    /// Period after which the story is moved to archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400 for Telegram Premium users, and 86400 otherwise

    #[serde(default)]
    active_period: i32,
    /// Pass true to keep the story accessible after expiration

    #[serde(default)]
    is_pinned: bool,
    /// Pass true if the content of the story must be protected from forwarding and screenshotting

    #[serde(default)]
    protect_content: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendStory {}

impl SendStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendStoryBuilder {
        let mut inner = SendStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendStory".to_string();

        SendStoryBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
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

    pub fn privacy_settings(&self) -> &StoryPrivacySettings {
        &self.privacy_settings
    }

    pub fn active_period(&self) -> i32 {
        self.active_period
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn protect_content(&self) -> bool {
        self.protect_content
    }
}

#[doc(hidden)]
pub struct SendStoryBuilder {
    inner: SendStory,
}

#[deprecated]
pub type RTDSendStoryBuilder = SendStoryBuilder;

impl SendStoryBuilder {
    pub fn build(&self) -> SendStory {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
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

    pub fn privacy_settings<T: AsRef<StoryPrivacySettings>>(
        &mut self,
        privacy_settings: T,
    ) -> &mut Self {
        self.inner.privacy_settings = privacy_settings.as_ref().clone();
        self
    }

    pub fn active_period(&mut self, active_period: i32) -> &mut Self {
        self.inner.active_period = active_period;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }

    pub fn protect_content(&mut self, protect_content: bool) -> &mut Self {
        self.inner.protect_content = protect_content;
        self
    }
}

impl AsRef<SendStory> for SendStory {
    fn as_ref(&self) -> &SendStory {
        self
    }
}

impl AsRef<SendStory> for SendStoryBuilder {
    fn as_ref(&self) -> &SendStory {
        &self.inner
    }
}
