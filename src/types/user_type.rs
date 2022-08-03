use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the type of a user. The following types are possible: regular users, deleted users and bots
pub trait TDUserType: Debug + RObject {}

/// Represents the type of a user. The following types are possible: regular users, deleted users and bots
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserType {
    #[doc(hidden)]
    _Default,
    /// A bot (see https://core.telegram.org/bots)
    #[serde(rename = "userTypeBot")]
    Bot(UserTypeBot),
    /// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
    #[serde(rename = "userTypeDeleted")]
    Deleted(UserTypeDeleted),
    /// A regular user
    #[serde(rename = "userTypeRegular")]
    Regular(UserTypeRegular),
    /// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
    #[serde(rename = "userTypeUnknown")]
    Unknown(UserTypeUnknown),
}

impl Default for UserType {
    fn default() -> Self {
        UserType::_Default
    }
}

impl RObject for UserType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            UserType::Bot(t) => t.extra(),
            UserType::Deleted(t) => t.extra(),
            UserType::Regular(t) => t.extra(),
            UserType::Unknown(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            UserType::Bot(t) => t.client_id(),
            UserType::Deleted(t) => t.client_id(),
            UserType::Regular(t) => t.client_id(),
            UserType::Unknown(t) => t.client_id(),

            _ => None,
        }
    }
}

impl UserType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, UserType::_Default)
    }
}

impl AsRef<UserType> for UserType {
    fn as_ref(&self) -> &UserType {
        self
    }
}

/// A bot (see https://core.telegram.org/bots)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the bot can be invited to basic group and supergroup chats

    #[serde(default)]
    can_join_groups: bool,
    /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages

    #[serde(default)]
    can_read_all_group_messages: bool,
    /// True, if the bot supports inline queries

    #[serde(default)]
    is_inline: bool,
    /// Placeholder for inline queries (displayed on the application input field)

    #[serde(default)]
    inline_query_placeholder: String,
    /// True, if the location of the user is expected to be sent with every inline query to this bot

    #[serde(default)]
    need_location: bool,
}

impl RObject for UserTypeBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserType for UserTypeBot {}

impl UserTypeBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserTypeBotBuilder {
        let mut inner = UserTypeBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserTypeBotBuilder { inner }
    }

    pub fn can_join_groups(&self) -> bool {
        self.can_join_groups
    }

    pub fn can_read_all_group_messages(&self) -> bool {
        self.can_read_all_group_messages
    }

    pub fn is_inline(&self) -> bool {
        self.is_inline
    }

    pub fn inline_query_placeholder(&self) -> &String {
        &self.inline_query_placeholder
    }

    pub fn need_location(&self) -> bool {
        self.need_location
    }
}

#[doc(hidden)]
pub struct UserTypeBotBuilder {
    inner: UserTypeBot,
}

#[deprecated]
pub type RTDUserTypeBotBuilder = UserTypeBotBuilder;

impl UserTypeBotBuilder {
    pub fn build(&self) -> UserTypeBot {
        self.inner.clone()
    }

    pub fn can_join_groups(&mut self, can_join_groups: bool) -> &mut Self {
        self.inner.can_join_groups = can_join_groups;
        self
    }

    pub fn can_read_all_group_messages(&mut self, can_read_all_group_messages: bool) -> &mut Self {
        self.inner.can_read_all_group_messages = can_read_all_group_messages;
        self
    }

    pub fn is_inline(&mut self, is_inline: bool) -> &mut Self {
        self.inner.is_inline = is_inline;
        self
    }

    pub fn inline_query_placeholder<T: AsRef<str>>(
        &mut self,
        inline_query_placeholder: T,
    ) -> &mut Self {
        self.inner.inline_query_placeholder = inline_query_placeholder.as_ref().to_string();
        self
    }

    pub fn need_location(&mut self, need_location: bool) -> &mut Self {
        self.inner.need_location = need_location;
        self
    }
}

impl AsRef<UserTypeBot> for UserTypeBot {
    fn as_ref(&self) -> &UserTypeBot {
        self
    }
}

impl AsRef<UserTypeBot> for UserTypeBotBuilder {
    fn as_ref(&self) -> &UserTypeBot {
        &self.inner
    }
}

/// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeDeleted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserTypeDeleted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserType for UserTypeDeleted {}

impl UserTypeDeleted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserTypeDeletedBuilder {
        let mut inner = UserTypeDeleted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserTypeDeletedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserTypeDeletedBuilder {
    inner: UserTypeDeleted,
}

#[deprecated]
pub type RTDUserTypeDeletedBuilder = UserTypeDeletedBuilder;

impl UserTypeDeletedBuilder {
    pub fn build(&self) -> UserTypeDeleted {
        self.inner.clone()
    }
}

impl AsRef<UserTypeDeleted> for UserTypeDeleted {
    fn as_ref(&self) -> &UserTypeDeleted {
        self
    }
}

impl AsRef<UserTypeDeleted> for UserTypeDeletedBuilder {
    fn as_ref(&self) -> &UserTypeDeleted {
        &self.inner
    }
}

/// A regular user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeRegular {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserTypeRegular {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserType for UserTypeRegular {}

impl UserTypeRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserTypeRegularBuilder {
        let mut inner = UserTypeRegular::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserTypeRegularBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserTypeRegularBuilder {
    inner: UserTypeRegular,
}

#[deprecated]
pub type RTDUserTypeRegularBuilder = UserTypeRegularBuilder;

impl UserTypeRegularBuilder {
    pub fn build(&self) -> UserTypeRegular {
        self.inner.clone()
    }
}

impl AsRef<UserTypeRegular> for UserTypeRegular {
    fn as_ref(&self) -> &UserTypeRegular {
        self
    }
}

impl AsRef<UserTypeRegular> for UserTypeRegularBuilder {
    fn as_ref(&self) -> &UserTypeRegular {
        &self.inner
    }
}

/// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeUnknown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserTypeUnknown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserType for UserTypeUnknown {}

impl UserTypeUnknown {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserTypeUnknownBuilder {
        let mut inner = UserTypeUnknown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserTypeUnknownBuilder { inner }
    }
}

#[doc(hidden)]
pub struct UserTypeUnknownBuilder {
    inner: UserTypeUnknown,
}

#[deprecated]
pub type RTDUserTypeUnknownBuilder = UserTypeUnknownBuilder;

impl UserTypeUnknownBuilder {
    pub fn build(&self) -> UserTypeUnknown {
        self.inner.clone()
    }
}

impl AsRef<UserTypeUnknown> for UserTypeUnknown {
    fn as_ref(&self) -> &UserTypeUnknown {
        self
    }
}

impl AsRef<UserTypeUnknown> for UserTypeUnknownBuilder {
    fn as_ref(&self) -> &UserTypeUnknown {
        &self.inner
    }
}
