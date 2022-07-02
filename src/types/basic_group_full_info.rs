use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicGroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat photo; may be null
    photo: Option<ChatPhoto>,
    /// Contains full information about a basic group

    #[serde(default)]
    description: String,
    /// User identifier of the creator of the group; 0 if unknown

    #[serde(default)]
    creator_user_id: i64,
    /// Group members

    #[serde(default)]
    members: Vec<ChatMember>,
    /// Primary invite link for this group; may be null. For chat administrators with can_invite_users right only. Updated only after the basic group is opened
    invite_link: Option<ChatInviteLink>,
    /// List of commands of bots in the group

    #[serde(default)]
    bot_commands: Vec<BotCommands>,
}

impl RObject for BasicGroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BasicGroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BasicGroupFullInfoBuilder {
        let mut inner = BasicGroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BasicGroupFullInfoBuilder { inner }
    }

    pub fn photo(&self) -> &Option<ChatPhoto> {
        &self.photo
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn creator_user_id(&self) -> i64 {
        self.creator_user_id
    }

    pub fn members(&self) -> &Vec<ChatMember> {
        &self.members
    }

    pub fn invite_link(&self) -> &Option<ChatInviteLink> {
        &self.invite_link
    }

    pub fn bot_commands(&self) -> &Vec<BotCommands> {
        &self.bot_commands
    }
}

#[doc(hidden)]
pub struct BasicGroupFullInfoBuilder {
    inner: BasicGroupFullInfo,
}

#[deprecated]
pub type RTDBasicGroupFullInfoBuilder = BasicGroupFullInfoBuilder;

impl BasicGroupFullInfoBuilder {
    pub fn build(&self) -> BasicGroupFullInfo {
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

    pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
        self.inner.creator_user_id = creator_user_id;
        self
    }

    pub fn members(&mut self, members: Vec<ChatMember>) -> &mut Self {
        self.inner.members = members;
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
}

impl AsRef<BasicGroupFullInfo> for BasicGroupFullInfo {
    fn as_ref(&self) -> &BasicGroupFullInfo {
        self
    }
}

impl AsRef<BasicGroupFullInfo> for BasicGroupFullInfoBuilder {
    fn as_ref(&self) -> &BasicGroupFullInfo {
        &self.inner
    }
}
