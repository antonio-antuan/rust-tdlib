use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text that is shown on the bot's profile page and is sent together with the link when users share the bot

    #[serde(default)]
    short_description: String,
    /// Contains information about a bot

    #[serde(default)]
    description: String,
    /// Photo shown in the chat with the bot if the chat is empty; may be null
    photo: Option<Photo>,
    /// Animation shown in the chat with the bot if the chat is empty; may be null
    animation: Option<Animation>,
    /// Information about a button to show instead of the bot commands menu button; may be null if ordinary bot commands menu must be shown
    menu_button: Option<BotMenuButton>,
    /// List of the bot commands

    #[serde(default)]
    commands: Vec<BotCommand>,
    /// Default administrator rights for adding the bot to basic group and supergroup chats; may be null
    default_group_administrator_rights: Option<ChatAdministratorRights>,
    /// Default administrator rights for adding the bot to channels; may be null
    default_channel_administrator_rights: Option<ChatAdministratorRights>,
    /// The internal link, which can be used to edit bot commands; may be null
    edit_commands_link: Option<InternalLinkType>,
    /// The internal link, which can be used to edit bot description; may be null
    edit_description_link: Option<InternalLinkType>,
    /// The internal link, which can be used to edit the photo or animation shown in the chat with the bot if the chat is empty; may be null
    edit_description_media_link: Option<InternalLinkType>,
    /// The internal link, which can be used to edit bot settings; may be null
    edit_settings_link: Option<InternalLinkType>,
}

impl RObject for BotInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BotInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotInfoBuilder {
        let mut inner = BotInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotInfoBuilder { inner }
    }

    pub fn short_description(&self) -> &String {
        &self.short_description
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn photo(&self) -> &Option<Photo> {
        &self.photo
    }

    pub fn animation(&self) -> &Option<Animation> {
        &self.animation
    }

    pub fn menu_button(&self) -> &Option<BotMenuButton> {
        &self.menu_button
    }

    pub fn commands(&self) -> &Vec<BotCommand> {
        &self.commands
    }

    pub fn default_group_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.default_group_administrator_rights
    }

    pub fn default_channel_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.default_channel_administrator_rights
    }

    pub fn edit_commands_link(&self) -> &Option<InternalLinkType> {
        &self.edit_commands_link
    }

    pub fn edit_description_link(&self) -> &Option<InternalLinkType> {
        &self.edit_description_link
    }

    pub fn edit_description_media_link(&self) -> &Option<InternalLinkType> {
        &self.edit_description_media_link
    }

    pub fn edit_settings_link(&self) -> &Option<InternalLinkType> {
        &self.edit_settings_link
    }
}

#[doc(hidden)]
pub struct BotInfoBuilder {
    inner: BotInfo,
}

#[deprecated]
pub type RTDBotInfoBuilder = BotInfoBuilder;

impl BotInfoBuilder {
    pub fn build(&self) -> BotInfo {
        self.inner.clone()
    }

    pub fn short_description<T: AsRef<str>>(&mut self, short_description: T) -> &mut Self {
        self.inner.short_description = short_description.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = Some(animation.as_ref().clone());
        self
    }

    pub fn menu_button<T: AsRef<BotMenuButton>>(&mut self, menu_button: T) -> &mut Self {
        self.inner.menu_button = Some(menu_button.as_ref().clone());
        self
    }

    pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
        self.inner.commands = commands;
        self
    }

    pub fn default_group_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        default_group_administrator_rights: T,
    ) -> &mut Self {
        self.inner.default_group_administrator_rights =
            Some(default_group_administrator_rights.as_ref().clone());
        self
    }

    pub fn default_channel_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        default_channel_administrator_rights: T,
    ) -> &mut Self {
        self.inner.default_channel_administrator_rights =
            Some(default_channel_administrator_rights.as_ref().clone());
        self
    }

    pub fn edit_commands_link<T: AsRef<InternalLinkType>>(
        &mut self,
        edit_commands_link: T,
    ) -> &mut Self {
        self.inner.edit_commands_link = Some(edit_commands_link.as_ref().clone());
        self
    }

    pub fn edit_description_link<T: AsRef<InternalLinkType>>(
        &mut self,
        edit_description_link: T,
    ) -> &mut Self {
        self.inner.edit_description_link = Some(edit_description_link.as_ref().clone());
        self
    }

    pub fn edit_description_media_link<T: AsRef<InternalLinkType>>(
        &mut self,
        edit_description_media_link: T,
    ) -> &mut Self {
        self.inner.edit_description_media_link = Some(edit_description_media_link.as_ref().clone());
        self
    }

    pub fn edit_settings_link<T: AsRef<InternalLinkType>>(
        &mut self,
        edit_settings_link: T,
    ) -> &mut Self {
        self.inner.edit_settings_link = Some(edit_settings_link.as_ref().clone());
        self
    }
}

impl AsRef<BotInfo> for BotInfo {
    fn as_ref(&self) -> &BotInfo {
        self
    }
}

impl AsRef<BotInfo> for BotInfoBuilder {
    fn as_ref(&self) -> &BotInfo {
        &self.inner
    }
}
