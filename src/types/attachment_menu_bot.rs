use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a bot added to attachment menu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentMenuBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the bot added to attachment menu

    #[serde(default)]
    bot_user_id: i64,
    /// True, if the bot supports opening from attachment menu in the chat with the bot

    #[serde(default)]
    supports_self_chat: bool,
    /// True, if the bot supports opening from attachment menu in private chats with ordinary users

    #[serde(default)]
    supports_user_chats: bool,
    /// True, if the bot supports opening from attachment menu in private chats with other bots

    #[serde(default)]
    supports_bot_chats: bool,
    /// True, if the bot supports opening from attachment menu in basic group and supergroup chats

    #[serde(default)]
    supports_group_chats: bool,
    /// True, if the bot supports opening from attachment menu in channel chats

    #[serde(default)]
    supports_channel_chats: bool,
    /// True, if the bot supports "settings_button_pressed" event

    #[serde(default)]
    supports_settings: bool,
    /// Name for the bot in attachment menu

    #[serde(default)]
    name: String,
    /// Color to highlight selected name of the bot if appropriate; may be null
    name_color: Option<AttachmentMenuBotColor>,
    /// Default attachment menu icon for the bot in SVG format; may be null
    default_icon: Option<File>,
    /// Attachment menu icon for the bot in SVG format for the official iOS app; may be null
    ios_static_icon: Option<File>,
    /// Attachment menu icon for the bot in TGS format for the official iOS app; may be null
    ios_animated_icon: Option<File>,
    /// Attachment menu icon for the bot in TGS format for the official Android app; may be null
    android_icon: Option<File>,
    /// Attachment menu icon for the bot in TGS format for the official native macOS app; may be null
    macos_icon: Option<File>,
    /// Color to highlight selected icon of the bot if appropriate; may be null
    icon_color: Option<AttachmentMenuBotColor>,
    /// Default placeholder for opened Web Apps in SVG format; may be null
    web_app_placeholder: Option<File>,
}

impl RObject for AttachmentMenuBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AttachmentMenuBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAttachmentMenuBotBuilder {
        let mut inner = AttachmentMenuBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAttachmentMenuBotBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn supports_self_chat(&self) -> bool {
        self.supports_self_chat
    }

    pub fn supports_user_chats(&self) -> bool {
        self.supports_user_chats
    }

    pub fn supports_bot_chats(&self) -> bool {
        self.supports_bot_chats
    }

    pub fn supports_group_chats(&self) -> bool {
        self.supports_group_chats
    }

    pub fn supports_channel_chats(&self) -> bool {
        self.supports_channel_chats
    }

    pub fn supports_settings(&self) -> bool {
        self.supports_settings
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn name_color(&self) -> &Option<AttachmentMenuBotColor> {
        &self.name_color
    }

    pub fn default_icon(&self) -> &Option<File> {
        &self.default_icon
    }

    pub fn ios_static_icon(&self) -> &Option<File> {
        &self.ios_static_icon
    }

    pub fn ios_animated_icon(&self) -> &Option<File> {
        &self.ios_animated_icon
    }

    pub fn android_icon(&self) -> &Option<File> {
        &self.android_icon
    }

    pub fn macos_icon(&self) -> &Option<File> {
        &self.macos_icon
    }

    pub fn icon_color(&self) -> &Option<AttachmentMenuBotColor> {
        &self.icon_color
    }

    pub fn web_app_placeholder(&self) -> &Option<File> {
        &self.web_app_placeholder
    }
}

#[doc(hidden)]
pub struct RTDAttachmentMenuBotBuilder {
    inner: AttachmentMenuBot,
}

impl RTDAttachmentMenuBotBuilder {
    pub fn build(&self) -> AttachmentMenuBot {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn supports_self_chat(&mut self, supports_self_chat: bool) -> &mut Self {
        self.inner.supports_self_chat = supports_self_chat;
        self
    }

    pub fn supports_user_chats(&mut self, supports_user_chats: bool) -> &mut Self {
        self.inner.supports_user_chats = supports_user_chats;
        self
    }

    pub fn supports_bot_chats(&mut self, supports_bot_chats: bool) -> &mut Self {
        self.inner.supports_bot_chats = supports_bot_chats;
        self
    }

    pub fn supports_group_chats(&mut self, supports_group_chats: bool) -> &mut Self {
        self.inner.supports_group_chats = supports_group_chats;
        self
    }

    pub fn supports_channel_chats(&mut self, supports_channel_chats: bool) -> &mut Self {
        self.inner.supports_channel_chats = supports_channel_chats;
        self
    }

    pub fn supports_settings(&mut self, supports_settings: bool) -> &mut Self {
        self.inner.supports_settings = supports_settings;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn name_color<T: AsRef<AttachmentMenuBotColor>>(&mut self, name_color: T) -> &mut Self {
        self.inner.name_color = Some(name_color.as_ref().clone());
        self
    }

    pub fn default_icon<T: AsRef<File>>(&mut self, default_icon: T) -> &mut Self {
        self.inner.default_icon = Some(default_icon.as_ref().clone());
        self
    }

    pub fn ios_static_icon<T: AsRef<File>>(&mut self, ios_static_icon: T) -> &mut Self {
        self.inner.ios_static_icon = Some(ios_static_icon.as_ref().clone());
        self
    }

    pub fn ios_animated_icon<T: AsRef<File>>(&mut self, ios_animated_icon: T) -> &mut Self {
        self.inner.ios_animated_icon = Some(ios_animated_icon.as_ref().clone());
        self
    }

    pub fn android_icon<T: AsRef<File>>(&mut self, android_icon: T) -> &mut Self {
        self.inner.android_icon = Some(android_icon.as_ref().clone());
        self
    }

    pub fn macos_icon<T: AsRef<File>>(&mut self, macos_icon: T) -> &mut Self {
        self.inner.macos_icon = Some(macos_icon.as_ref().clone());
        self
    }

    pub fn icon_color<T: AsRef<AttachmentMenuBotColor>>(&mut self, icon_color: T) -> &mut Self {
        self.inner.icon_color = Some(icon_color.as_ref().clone());
        self
    }

    pub fn web_app_placeholder<T: AsRef<File>>(&mut self, web_app_placeholder: T) -> &mut Self {
        self.inner.web_app_placeholder = Some(web_app_placeholder.as_ref().clone());
        self
    }
}

impl AsRef<AttachmentMenuBot> for AttachmentMenuBot {
    fn as_ref(&self) -> &AttachmentMenuBot {
        self
    }
}

impl AsRef<AttachmentMenuBot> for RTDAttachmentMenuBotBuilder {
    fn as_ref(&self) -> &AttachmentMenuBot {
        &self.inner
    }
}
