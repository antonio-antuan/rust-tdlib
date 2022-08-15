use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat photo; may be null
    photo: Option<ChatPhoto>,
    /// Contains full information about a supergroup or channel

    #[serde(default)]
    description: String,
    /// Number of members in the supergroup or channel; 0 if unknown

    #[serde(default)]
    member_count: i32,
    /// Number of privileged users in the supergroup or channel; 0 if unknown

    #[serde(default)]
    administrator_count: i32,
    /// Number of restricted users in the supergroup; 0 if unknown

    #[serde(default)]
    restricted_count: i32,
    /// Number of users banned from chat; 0 if unknown

    #[serde(default)]
    banned_count: i32,
    /// Chat identifier of a discussion group for the channel, or a channel, for which the supergroup is the designated discussion group; 0 if none or unknown

    #[serde(default)]
    linked_chat_id: i64,
    /// Delay between consecutive sent messages for non-administrator supergroup members, in seconds

    #[serde(default)]
    slow_mode_delay: i32,
    /// Time left before next message can be sent in the supergroup, in seconds. An updateSupergroupFullInfo update is not triggered when value of this field changes, but both new and old values are non-zero

    #[serde(default)]
    slow_mode_delay_expires_in: f32,
    /// True, if members of the chat can be retrieved

    #[serde(default)]
    can_get_members: bool,
    /// True, if the chat username can be changed

    #[serde(default)]
    can_set_username: bool,
    /// True, if the supergroup sticker set can be changed

    #[serde(default)]
    can_set_sticker_set: bool,
    /// True, if the supergroup location can be changed

    #[serde(default)]
    can_set_location: bool,
    /// True, if the supergroup or channel statistics are available

    #[serde(default)]
    can_get_statistics: bool,
    /// True, if new chat members will have access to old messages. In public or discussion groups and both public and private channels, old messages are always available, so this option affects only private supergroups without a linked chat. The value of this field is only available for chat administrators

    #[serde(default)]
    is_all_history_available: bool,
    /// Identifier of the supergroup sticker set; 0 if none

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sticker_set_id: i64,
    /// Location to which the supergroup is connected; may be null
    location: Option<ChatLocation>,
    /// Primary invite link for this chat; may be null. For chat administrators with can_invite_users right only
    invite_link: Option<ChatInviteLink>,
    /// List of commands of bots in the group

    #[serde(default)]
    bot_commands: Vec<BotCommands>,
    /// Identifier of the basic group from which supergroup was upgraded; 0 if none

    #[serde(default)]
    upgraded_from_basic_group_id: i64,
    /// Identifier of the last message in the basic group from which supergroup was upgraded; 0 if none

    #[serde(default)]
    upgraded_from_max_message_id: i64,
}

impl RObject for SupergroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SupergroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupFullInfoBuilder {
        let mut inner = SupergroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupFullInfoBuilder { inner }
    }

    pub fn photo(&self) -> &Option<ChatPhoto> {
        &self.photo
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn member_count(&self) -> i32 {
        self.member_count
    }

    pub fn administrator_count(&self) -> i32 {
        self.administrator_count
    }

    pub fn restricted_count(&self) -> i32 {
        self.restricted_count
    }

    pub fn banned_count(&self) -> i32 {
        self.banned_count
    }

    pub fn linked_chat_id(&self) -> i64 {
        self.linked_chat_id
    }

    pub fn slow_mode_delay(&self) -> i32 {
        self.slow_mode_delay
    }

    pub fn slow_mode_delay_expires_in(&self) -> f32 {
        self.slow_mode_delay_expires_in
    }

    pub fn can_get_members(&self) -> bool {
        self.can_get_members
    }

    pub fn can_set_username(&self) -> bool {
        self.can_set_username
    }

    pub fn can_set_sticker_set(&self) -> bool {
        self.can_set_sticker_set
    }

    pub fn can_set_location(&self) -> bool {
        self.can_set_location
    }

    pub fn can_get_statistics(&self) -> bool {
        self.can_get_statistics
    }

    pub fn is_all_history_available(&self) -> bool {
        self.is_all_history_available
    }

    pub fn sticker_set_id(&self) -> i64 {
        self.sticker_set_id
    }

    pub fn location(&self) -> &Option<ChatLocation> {
        &self.location
    }

    pub fn invite_link(&self) -> &Option<ChatInviteLink> {
        &self.invite_link
    }

    pub fn bot_commands(&self) -> &Vec<BotCommands> {
        &self.bot_commands
    }

    pub fn upgraded_from_basic_group_id(&self) -> i64 {
        self.upgraded_from_basic_group_id
    }

    pub fn upgraded_from_max_message_id(&self) -> i64 {
        self.upgraded_from_max_message_id
    }
}

#[doc(hidden)]
pub struct SupergroupFullInfoBuilder {
    inner: SupergroupFullInfo,
}

#[deprecated]
pub type RTDSupergroupFullInfoBuilder = SupergroupFullInfoBuilder;

impl SupergroupFullInfoBuilder {
    pub fn build(&self) -> SupergroupFullInfo {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn member_count(&mut self, member_count: i32) -> &mut Self {
        self.inner.member_count = member_count;
        self
    }

    pub fn administrator_count(&mut self, administrator_count: i32) -> &mut Self {
        self.inner.administrator_count = administrator_count;
        self
    }

    pub fn restricted_count(&mut self, restricted_count: i32) -> &mut Self {
        self.inner.restricted_count = restricted_count;
        self
    }

    pub fn banned_count(&mut self, banned_count: i32) -> &mut Self {
        self.inner.banned_count = banned_count;
        self
    }

    pub fn linked_chat_id(&mut self, linked_chat_id: i64) -> &mut Self {
        self.inner.linked_chat_id = linked_chat_id;
        self
    }

    pub fn slow_mode_delay(&mut self, slow_mode_delay: i32) -> &mut Self {
        self.inner.slow_mode_delay = slow_mode_delay;
        self
    }

    pub fn slow_mode_delay_expires_in(&mut self, slow_mode_delay_expires_in: f32) -> &mut Self {
        self.inner.slow_mode_delay_expires_in = slow_mode_delay_expires_in;
        self
    }

    pub fn can_get_members(&mut self, can_get_members: bool) -> &mut Self {
        self.inner.can_get_members = can_get_members;
        self
    }

    pub fn can_set_username(&mut self, can_set_username: bool) -> &mut Self {
        self.inner.can_set_username = can_set_username;
        self
    }

    pub fn can_set_sticker_set(&mut self, can_set_sticker_set: bool) -> &mut Self {
        self.inner.can_set_sticker_set = can_set_sticker_set;
        self
    }

    pub fn can_set_location(&mut self, can_set_location: bool) -> &mut Self {
        self.inner.can_set_location = can_set_location;
        self
    }

    pub fn can_get_statistics(&mut self, can_get_statistics: bool) -> &mut Self {
        self.inner.can_get_statistics = can_get_statistics;
        self
    }

    pub fn is_all_history_available(&mut self, is_all_history_available: bool) -> &mut Self {
        self.inner.is_all_history_available = is_all_history_available;
        self
    }

    pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self {
        self.inner.sticker_set_id = sticker_set_id;
        self
    }

    pub fn location<T: AsRef<ChatLocation>>(&mut self, location: T) -> &mut Self {
        self.inner.location = Some(location.as_ref().clone());
        self
    }

    pub fn invite_link<T: AsRef<ChatInviteLink>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = Some(invite_link.as_ref().clone());
        self
    }

    pub fn bot_commands(&mut self, bot_commands: Vec<BotCommands>) -> &mut Self {
        self.inner.bot_commands = bot_commands;
        self
    }

    pub fn upgraded_from_basic_group_id(&mut self, upgraded_from_basic_group_id: i64) -> &mut Self {
        self.inner.upgraded_from_basic_group_id = upgraded_from_basic_group_id;
        self
    }

    pub fn upgraded_from_max_message_id(&mut self, upgraded_from_max_message_id: i64) -> &mut Self {
        self.inner.upgraded_from_max_message_id = upgraded_from_max_message_id;
        self
    }
}

impl AsRef<SupergroupFullInfo> for SupergroupFullInfo {
    fn as_ref(&self) -> &SupergroupFullInfo {
        self
    }
}

impl AsRef<SupergroupFullInfo> for SupergroupFullInfoBuilder {
    fn as_ref(&self) -> &SupergroupFullInfo {
        &self.inner
    }
}
