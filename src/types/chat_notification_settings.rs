use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about notification settings for a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatNotificationSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, mute_for is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_mute_for: bool,
    /// Time left before notifications will be unmuted, in seconds

    #[serde(default)]
    mute_for: i32,
    /// If true, sound is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_sound: bool,
    /// The name of an audio file to be used for notification sounds; only applies to iOS applications

    #[serde(default)]
    sound: String,
    /// If true, show_preview is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_show_preview: bool,
    /// True, if message content must be displayed in notifications

    #[serde(default)]
    show_preview: bool,
    /// If true, disable_pinned_message_notifications is ignored and the value for the relevant type of chat is used instead

    #[serde(default)]
    use_default_disable_pinned_message_notifications: bool,
    /// If true, notifications for incoming pinned messages will be created as for an ordinary unread message

    #[serde(default)]
    disable_pinned_message_notifications: bool,
    /// If true, disable_mention_notifications is ignored and the value for the relevant type of chat is used instead

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

    pub fn sound(&self) -> &String {
        &self.sound
    }

    pub fn use_default_show_preview(&self) -> bool {
        self.use_default_show_preview
    }

    pub fn show_preview(&self) -> bool {
        self.show_preview
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

    pub fn sound<T: AsRef<str>>(&mut self, sound: T) -> &mut Self {
        self.inner.sound = sound.as_ref().to_string();
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
