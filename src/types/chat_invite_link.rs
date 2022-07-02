use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat invite link

    #[serde(default)]
    invite_link: String,
    /// Name of the link

    #[serde(default)]
    name: String,
    /// User identifier of an administrator created the link

    #[serde(default)]
    creator_user_id: i64,
    /// Point in time (Unix timestamp) when the link was created

    #[serde(default)]
    date: i32,
    /// Point in time (Unix timestamp) when the link was last edited; 0 if never or unknown

    #[serde(default)]
    edit_date: i32,
    /// Point in time (Unix timestamp) when the link will expire; 0 if never

    #[serde(default)]
    expiration_date: i32,
    /// The maximum number of members, which can join the chat using the link simultaneously; 0 if not limited. Always 0 if the link requires approval

    #[serde(default)]
    member_limit: i32,
    /// Number of chat members, which joined the chat using the link

    #[serde(default)]
    member_count: i32,
    /// Number of pending join requests created using this link

    #[serde(default)]
    pending_join_request_count: i32,
    /// True, if the link only creates join request. If true, total number of joining members will be unlimited

    #[serde(default)]
    creates_join_request: bool,
    /// True, if the link is primary. Primary invite link can't have name, expiration date, or usage limit. There is exactly one primary invite link for each administrator with can_invite_users right at a given time

    #[serde(default)]
    is_primary: bool,
    /// True, if the link was revoked

    #[serde(default)]
    is_revoked: bool,
}

impl RObject for ChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatInviteLinkBuilder {
        let mut inner = ChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatInviteLinkBuilder { inner }
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn creator_user_id(&self) -> i64 {
        self.creator_user_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn edit_date(&self) -> i32 {
        self.edit_date
    }

    pub fn expiration_date(&self) -> i32 {
        self.expiration_date
    }

    pub fn member_limit(&self) -> i32 {
        self.member_limit
    }

    pub fn member_count(&self) -> i32 {
        self.member_count
    }

    pub fn pending_join_request_count(&self) -> i32 {
        self.pending_join_request_count
    }

    pub fn creates_join_request(&self) -> bool {
        self.creates_join_request
    }

    pub fn is_primary(&self) -> bool {
        self.is_primary
    }

    pub fn is_revoked(&self) -> bool {
        self.is_revoked
    }
}

#[doc(hidden)]
pub struct ChatInviteLinkBuilder {
    inner: ChatInviteLink,
}

#[deprecated]
pub type RTDChatInviteLinkBuilder = ChatInviteLinkBuilder;

impl ChatInviteLinkBuilder {
    pub fn build(&self) -> ChatInviteLink {
        self.inner.clone()
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
        self.inner.creator_user_id = creator_user_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn edit_date(&mut self, edit_date: i32) -> &mut Self {
        self.inner.edit_date = edit_date;
        self
    }

    pub fn expiration_date(&mut self, expiration_date: i32) -> &mut Self {
        self.inner.expiration_date = expiration_date;
        self
    }

    pub fn member_limit(&mut self, member_limit: i32) -> &mut Self {
        self.inner.member_limit = member_limit;
        self
    }

    pub fn member_count(&mut self, member_count: i32) -> &mut Self {
        self.inner.member_count = member_count;
        self
    }

    pub fn pending_join_request_count(&mut self, pending_join_request_count: i32) -> &mut Self {
        self.inner.pending_join_request_count = pending_join_request_count;
        self
    }

    pub fn creates_join_request(&mut self, creates_join_request: bool) -> &mut Self {
        self.inner.creates_join_request = creates_join_request;
        self
    }

    pub fn is_primary(&mut self, is_primary: bool) -> &mut Self {
        self.inner.is_primary = is_primary;
        self
    }

    pub fn is_revoked(&mut self, is_revoked: bool) -> &mut Self {
        self.inner.is_revoked = is_revoked;
        self
    }
}

impl AsRef<ChatInviteLink> for ChatInviteLink {
    fn as_ref(&self) -> &ChatInviteLink {
        self
    }
}

impl AsRef<ChatInviteLink> for ChatInviteLinkBuilder {
    fn as_ref(&self) -> &ChatInviteLink {
        &self.inner
    }
}
