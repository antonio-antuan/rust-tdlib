use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Edits a non-primary invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Invite link to be edited
    invite_link: String,
    /// Invite link name; 0-32 characters
    name: String,
    /// Point in time (Unix timestamp) when the link will expire; pass 0 if never
    expiration_date: i32,
    /// The maximum number of chat members that can join the chat via the link simultaneously; 0-99999; pass 0 if not limited
    member_limit: i32,
    /// Pass true if users joining the chat via the link need to be approved by chat administrators. In this case, member_limit must be 0
    creates_join_request: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditChatInviteLink {}

impl EditChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEditChatInviteLinkBuilder {
        let mut inner = EditChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editChatInviteLink".to_string();

        RTDEditChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn expiration_date(&self) -> i32 {
        self.expiration_date
    }

    pub fn member_limit(&self) -> i32 {
        self.member_limit
    }

    pub fn creates_join_request(&self) -> bool {
        self.creates_join_request
    }
}

#[doc(hidden)]
pub struct RTDEditChatInviteLinkBuilder {
    inner: EditChatInviteLink,
}

impl RTDEditChatInviteLinkBuilder {
    pub fn build(&self) -> EditChatInviteLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn invite_link<T: AsRef<str>>(&mut self, invite_link: T) -> &mut Self {
        self.inner.invite_link = invite_link.as_ref().to_string();
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
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

    pub fn creates_join_request(&mut self, creates_join_request: bool) -> &mut Self {
        self.inner.creates_join_request = creates_join_request;
        self
    }
}

impl AsRef<EditChatInviteLink> for EditChatInviteLink {
    fn as_ref(&self) -> &EditChatInviteLink {
        self
    }
}

impl AsRef<EditChatInviteLink> for RTDEditChatInviteLinkBuilder {
    fn as_ref(&self) -> &EditChatInviteLink {
        &self.inner
    }
}
