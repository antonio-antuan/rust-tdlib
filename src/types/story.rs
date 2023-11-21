use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Story {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique story identifier among stories of the given sender

    #[serde(default)]
    id: i32,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    sender_chat_id: i64,
    /// Point in time (Unix timestamp) when the story was published

    #[serde(default)]
    date: i32,
    /// True, if the story is being sent by the current user

    #[serde(default)]
    is_being_sent: bool,
    /// True, if the story is being edited by the current user

    #[serde(default)]
    is_being_edited: bool,
    /// True, if the story was edited

    #[serde(default)]
    is_edited: bool,
    /// True, if the story is saved in the sender's profile and will be available there after expiration

    #[serde(default)]
    is_pinned: bool,
    /// True, if the story is visible only for the current user

    #[serde(default)]
    is_visible_only_for_self: bool,
    /// True, if the story can be deleted

    #[serde(default)]
    can_be_deleted: bool,
    /// True, if the story can be edited

    #[serde(default)]
    can_be_edited: bool,
    /// True, if the story can be forwarded as a message. Otherwise, screenshots and saving of the story content must be also forbidden

    #[serde(default)]
    can_be_forwarded: bool,
    /// True, if the story can be replied in the chat with the story sender

    #[serde(default)]
    can_be_replied: bool,
    /// True, if the story's is_pinned value can be changed

    #[serde(default)]
    can_toggle_is_pinned: bool,
    /// True, if users viewed the story can be received through getStoryViewers

    #[serde(default)]
    can_get_viewers: bool,
    /// True, if users viewed the story can't be received, because the story has expired more than getOption("story_viewers_expiration_delay") seconds ago

    #[serde(default)]
    has_expired_viewers: bool,
    /// Information about interactions with the story; may be null if the story isn't owned or there were no interactions
    interaction_info: Option<StoryInteractionInfo>,
    /// Type of the chosen reaction; may be null if none
    chosen_reaction_type: Option<ReactionType>,
    /// Privacy rules affecting story visibility; may be approximate for non-owned stories

    #[serde(skip_serializing_if = "StoryPrivacySettings::_is_default")]
    privacy_settings: StoryPrivacySettings,
    /// Content of the story

    #[serde(skip_serializing_if = "StoryContent::_is_default")]
    content: StoryContent,
    /// Clickable areas to be shown on the story content

    #[serde(default)]
    areas: Vec<StoryArea>,
    /// Caption of the story
    caption: FormattedText,
}

impl RObject for Story {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Story {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryBuilder {
        let mut inner = Story::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn sender_chat_id(&self) -> i64 {
        self.sender_chat_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn is_being_sent(&self) -> bool {
        self.is_being_sent
    }

    pub fn is_being_edited(&self) -> bool {
        self.is_being_edited
    }

    pub fn is_edited(&self) -> bool {
        self.is_edited
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn is_visible_only_for_self(&self) -> bool {
        self.is_visible_only_for_self
    }

    pub fn can_be_deleted(&self) -> bool {
        self.can_be_deleted
    }

    pub fn can_be_edited(&self) -> bool {
        self.can_be_edited
    }

    pub fn can_be_forwarded(&self) -> bool {
        self.can_be_forwarded
    }

    pub fn can_be_replied(&self) -> bool {
        self.can_be_replied
    }

    pub fn can_toggle_is_pinned(&self) -> bool {
        self.can_toggle_is_pinned
    }

    pub fn can_get_viewers(&self) -> bool {
        self.can_get_viewers
    }

    pub fn has_expired_viewers(&self) -> bool {
        self.has_expired_viewers
    }

    pub fn interaction_info(&self) -> &Option<StoryInteractionInfo> {
        &self.interaction_info
    }

    pub fn chosen_reaction_type(&self) -> &Option<ReactionType> {
        &self.chosen_reaction_type
    }

    pub fn privacy_settings(&self) -> &StoryPrivacySettings {
        &self.privacy_settings
    }

    pub fn content(&self) -> &StoryContent {
        &self.content
    }

    pub fn areas(&self) -> &Vec<StoryArea> {
        &self.areas
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct StoryBuilder {
    inner: Story,
}

#[deprecated]
pub type RTDStoryBuilder = StoryBuilder;

impl StoryBuilder {
    pub fn build(&self) -> Story {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn sender_chat_id(&mut self, sender_chat_id: i64) -> &mut Self {
        self.inner.sender_chat_id = sender_chat_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn is_being_sent(&mut self, is_being_sent: bool) -> &mut Self {
        self.inner.is_being_sent = is_being_sent;
        self
    }

    pub fn is_being_edited(&mut self, is_being_edited: bool) -> &mut Self {
        self.inner.is_being_edited = is_being_edited;
        self
    }

    pub fn is_edited(&mut self, is_edited: bool) -> &mut Self {
        self.inner.is_edited = is_edited;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }

    pub fn is_visible_only_for_self(&mut self, is_visible_only_for_self: bool) -> &mut Self {
        self.inner.is_visible_only_for_self = is_visible_only_for_self;
        self
    }

    pub fn can_be_deleted(&mut self, can_be_deleted: bool) -> &mut Self {
        self.inner.can_be_deleted = can_be_deleted;
        self
    }

    pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
        self.inner.can_be_edited = can_be_edited;
        self
    }

    pub fn can_be_forwarded(&mut self, can_be_forwarded: bool) -> &mut Self {
        self.inner.can_be_forwarded = can_be_forwarded;
        self
    }

    pub fn can_be_replied(&mut self, can_be_replied: bool) -> &mut Self {
        self.inner.can_be_replied = can_be_replied;
        self
    }

    pub fn can_toggle_is_pinned(&mut self, can_toggle_is_pinned: bool) -> &mut Self {
        self.inner.can_toggle_is_pinned = can_toggle_is_pinned;
        self
    }

    pub fn can_get_viewers(&mut self, can_get_viewers: bool) -> &mut Self {
        self.inner.can_get_viewers = can_get_viewers;
        self
    }

    pub fn has_expired_viewers(&mut self, has_expired_viewers: bool) -> &mut Self {
        self.inner.has_expired_viewers = has_expired_viewers;
        self
    }

    pub fn interaction_info<T: AsRef<StoryInteractionInfo>>(
        &mut self,
        interaction_info: T,
    ) -> &mut Self {
        self.inner.interaction_info = Some(interaction_info.as_ref().clone());
        self
    }

    pub fn chosen_reaction_type<T: AsRef<ReactionType>>(
        &mut self,
        chosen_reaction_type: T,
    ) -> &mut Self {
        self.inner.chosen_reaction_type = Some(chosen_reaction_type.as_ref().clone());
        self
    }

    pub fn privacy_settings<T: AsRef<StoryPrivacySettings>>(
        &mut self,
        privacy_settings: T,
    ) -> &mut Self {
        self.inner.privacy_settings = privacy_settings.as_ref().clone();
        self
    }

    pub fn content<T: AsRef<StoryContent>>(&mut self, content: T) -> &mut Self {
        self.inner.content = content.as_ref().clone();
        self
    }

    pub fn areas(&mut self, areas: Vec<StoryArea>) -> &mut Self {
        self.inner.areas = areas;
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<Story> for Story {
    fn as_ref(&self) -> &Story {
        self
    }
}

impl AsRef<Story> for StoryBuilder {
    fn as_ref(&self) -> &Story {
        &self.inner
    }
}
