use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for several chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before notifications will be unmuted, in seconds

    #[serde(default)]
    mute_for: i32,
    /// Identifier of the notification sound to be played; 0 if sound is disabled

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sound_id: i64,
    /// True, if message content must be displayed in notifications

    #[serde(default)]
    show_preview: bool,
    /// If true, mute_stories is ignored and story notifications are received only for the first 5 chats from topChatCategoryUsers

    #[serde(default)]
    use_default_mute_stories: bool,
    /// True, if story notifications are disabled for the chat

    #[serde(default)]
    mute_stories: bool,
    /// Identifier of the notification sound to be played for stories; 0 if sound is disabled

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    story_sound_id: i64,
    /// True, if the sender of stories must be displayed in notifications

    #[serde(default)]
    show_story_sender: bool,
    /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message

    #[serde(default)]
    disable_pinned_message_notifications: bool,
    /// True, if notifications for messages with mentions will be created as for an ordinary unread message

    #[serde(default)]
    disable_mention_notifications: bool,
}

impl RObject for ScopeNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ScopeNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ScopeNotificationSettingsBuilder {
        let mut inner = ScopeNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ScopeNotificationSettingsBuilder { inner }
    }

    pub fn mute_for(&self) -> i32 {
        self.mute_for
    }

    pub fn sound_id(&self) -> i64 {
        self.sound_id
    }

    pub fn show_preview(&self) -> bool {
        self.show_preview
    }

    pub fn use_default_mute_stories(&self) -> bool {
        self.use_default_mute_stories
    }

    pub fn mute_stories(&self) -> bool {
        self.mute_stories
    }

    pub fn story_sound_id(&self) -> i64 {
        self.story_sound_id
    }

    pub fn show_story_sender(&self) -> bool {
        self.show_story_sender
    }

    pub fn disable_pinned_message_notifications(&self) -> bool {
        self.disable_pinned_message_notifications
    }

    pub fn disable_mention_notifications(&self) -> bool {
        self.disable_mention_notifications
    }
}

#[doc(hidden)]
pub struct ScopeNotificationSettingsBuilder {
    inner: ScopeNotificationSettings,
}

#[deprecated]
pub type RTDScopeNotificationSettingsBuilder = ScopeNotificationSettingsBuilder;

impl ScopeNotificationSettingsBuilder {
    pub fn build(&self) -> ScopeNotificationSettings {
        self.inner.clone()
    }

    pub fn mute_for(&mut self, mute_for: i32) -> &mut Self {
        self.inner.mute_for = mute_for;
        self
    }

    pub fn sound_id(&mut self, sound_id: i64) -> &mut Self {
        self.inner.sound_id = sound_id;
        self
    }

    pub fn show_preview(&mut self, show_preview: bool) -> &mut Self {
        self.inner.show_preview = show_preview;
        self
    }

    pub fn use_default_mute_stories(&mut self, use_default_mute_stories: bool) -> &mut Self {
        self.inner.use_default_mute_stories = use_default_mute_stories;
        self
    }

    pub fn mute_stories(&mut self, mute_stories: bool) -> &mut Self {
        self.inner.mute_stories = mute_stories;
        self
    }

    pub fn story_sound_id(&mut self, story_sound_id: i64) -> &mut Self {
        self.inner.story_sound_id = story_sound_id;
        self
    }

    pub fn show_story_sender(&mut self, show_story_sender: bool) -> &mut Self {
        self.inner.show_story_sender = show_story_sender;
        self
    }

    pub fn disable_pinned_message_notifications(
        &mut self,
        disable_pinned_message_notifications: bool,
    ) -> &mut Self {
        self.inner.disable_pinned_message_notifications = disable_pinned_message_notifications;
        self
    }

    pub fn disable_mention_notifications(
        &mut self,
        disable_mention_notifications: bool,
    ) -> &mut Self {
        self.inner.disable_mention_notifications = disable_mention_notifications;
        self
    }
}

impl AsRef<ScopeNotificationSettings> for ScopeNotificationSettings {
    fn as_ref(&self) -> &ScopeNotificationSettings {
        self
    }
}

impl AsRef<ScopeNotificationSettings> for ScopeNotificationSettingsBuilder {
    fn as_ref(&self) -> &ScopeNotificationSettings {
        &self.inner
    }
}
