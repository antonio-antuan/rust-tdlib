use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for a chat or a forum topic
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, mute_for is ignored and the value for the relevant type of chat or the forum chat is used instead

    #[serde(default)]
    use_default_mute_for: bool,
    /// Time left before notifications will be unmuted, in seconds

    #[serde(default)]
    mute_for: i32,
    /// If true, the value for the relevant type of chat or the forum chat is used instead of sound_id

    #[serde(default)]
    use_default_sound: bool,
    /// Identifier of the notification sound to be played for messages; 0 if sound is disabled

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sound_id: i64,
    /// If true, show_preview is ignored and the value for the relevant type of chat or the forum chat is used instead

    #[serde(default)]
    use_default_show_preview: bool,
    /// True, if message content must be displayed in notifications

    #[serde(default)]
    show_preview: bool,
    /// If true, mute_stories is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_mute_stories: bool,
    /// True, if story notifications are disabled for the chat

    #[serde(default)]
    mute_stories: bool,
    /// If true, the value for the relevant type of chat is used instead of story_sound_id

    #[serde(default)]
    use_default_story_sound: bool,
    /// Identifier of the notification sound to be played for stories; 0 if sound is disabled

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    story_sound_id: i64,
    /// If true, show_story_sender is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_show_story_sender: bool,
    /// True, if the sender of stories must be displayed in notifications

    #[serde(default)]
    show_story_sender: bool,
    /// If true, disable_pinned_message_notifications is ignored and the value for the relevant type of chat or the forum chat is used instead

    #[serde(default)]
    use_default_disable_pinned_message_notifications: bool,
    /// If true, notifications for incoming pinned messages will be created as for an ordinary unread message

    #[serde(default)]
    disable_pinned_message_notifications: bool,
    /// If true, disable_mention_notifications is ignored and the value for the relevant type of chat or the forum chat is used instead

    #[serde(default)]
    use_default_disable_mention_notifications: bool,
    /// If true, notifications for messages with mentions will be created as for an ordinary unread message

    #[serde(default)]
    disable_mention_notifications: bool,
}

impl RObject for ChatNotificationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatNotificationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatNotificationSettingsBuilder {
        let mut inner = ChatNotificationSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatNotificationSettingsBuilder { inner }
    }

    pub fn use_default_mute_for(&self) -> bool {
        self.use_default_mute_for
    }

    pub fn mute_for(&self) -> i32 {
        self.mute_for
    }

    pub fn use_default_sound(&self) -> bool {
        self.use_default_sound
    }

    pub fn sound_id(&self) -> i64 {
        self.sound_id
    }

    pub fn use_default_show_preview(&self) -> bool {
        self.use_default_show_preview
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

    pub fn use_default_story_sound(&self) -> bool {
        self.use_default_story_sound
    }

    pub fn story_sound_id(&self) -> i64 {
        self.story_sound_id
    }

    pub fn use_default_show_story_sender(&self) -> bool {
        self.use_default_show_story_sender
    }

    pub fn show_story_sender(&self) -> bool {
        self.show_story_sender
    }

    pub fn use_default_disable_pinned_message_notifications(&self) -> bool {
        self.use_default_disable_pinned_message_notifications
    }

    pub fn disable_pinned_message_notifications(&self) -> bool {
        self.disable_pinned_message_notifications
    }

    pub fn use_default_disable_mention_notifications(&self) -> bool {
        self.use_default_disable_mention_notifications
    }

    pub fn disable_mention_notifications(&self) -> bool {
        self.disable_mention_notifications
    }
}

#[doc(hidden)]
pub struct ChatNotificationSettingsBuilder {
    inner: ChatNotificationSettings,
}

#[deprecated]
pub type RTDChatNotificationSettingsBuilder = ChatNotificationSettingsBuilder;

impl ChatNotificationSettingsBuilder {
    pub fn build(&self) -> ChatNotificationSettings {
        self.inner.clone()
    }

    pub fn use_default_mute_for(&mut self, use_default_mute_for: bool) -> &mut Self {
        self.inner.use_default_mute_for = use_default_mute_for;
        self
    }

    pub fn mute_for(&mut self, mute_for: i32) -> &mut Self {
        self.inner.mute_for = mute_for;
        self
    }

    pub fn use_default_sound(&mut self, use_default_sound: bool) -> &mut Self {
        self.inner.use_default_sound = use_default_sound;
        self
    }

    pub fn sound_id(&mut self, sound_id: i64) -> &mut Self {
        self.inner.sound_id = sound_id;
        self
    }

    pub fn use_default_show_preview(&mut self, use_default_show_preview: bool) -> &mut Self {
        self.inner.use_default_show_preview = use_default_show_preview;
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

    pub fn use_default_story_sound(&mut self, use_default_story_sound: bool) -> &mut Self {
        self.inner.use_default_story_sound = use_default_story_sound;
        self
    }

    pub fn story_sound_id(&mut self, story_sound_id: i64) -> &mut Self {
        self.inner.story_sound_id = story_sound_id;
        self
    }

    pub fn use_default_show_story_sender(
        &mut self,
        use_default_show_story_sender: bool,
    ) -> &mut Self {
        self.inner.use_default_show_story_sender = use_default_show_story_sender;
        self
    }

    pub fn show_story_sender(&mut self, show_story_sender: bool) -> &mut Self {
        self.inner.show_story_sender = show_story_sender;
        self
    }

    pub fn use_default_disable_pinned_message_notifications(
        &mut self,
        use_default_disable_pinned_message_notifications: bool,
    ) -> &mut Self {
        self.inner.use_default_disable_pinned_message_notifications =
            use_default_disable_pinned_message_notifications;
        self
    }

    pub fn disable_pinned_message_notifications(
        &mut self,
        disable_pinned_message_notifications: bool,
    ) -> &mut Self {
        self.inner.disable_pinned_message_notifications = disable_pinned_message_notifications;
        self
    }

    pub fn use_default_disable_mention_notifications(
        &mut self,
        use_default_disable_mention_notifications: bool,
    ) -> &mut Self {
        self.inner.use_default_disable_mention_notifications =
            use_default_disable_mention_notifications;
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

impl AsRef<ChatNotificationSettings> for ChatNotificationSettings {
    fn as_ref(&self) -> &ChatNotificationSettings {
        self
    }
}

impl AsRef<ChatNotificationSettings> for ChatNotificationSettingsBuilder {
    fn as_ref(&self) -> &ChatNotificationSettings {
        &self.inner
    }
}
