use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates a new invite link for a chat. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatInviteLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link name; 0-32 characters

    #[serde(default)]
    name: String,
    /// Point in time (Unix timestamp) when the link will expire; pass 0 if never

    #[serde(default)]
    expiration_date: i32,
    /// The maximum number of chat members that can join the chat via the link simultaneously; 0-99999; pass 0 if not limited

    #[serde(default)]
    member_limit: i32,
    /// True, if the link only creates join request. If true, member_limit must not be specified

    #[serde(default)]
    creates_join_request: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateChatInviteLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateChatInviteLink {}

impl CreateChatInviteLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateChatInviteLinkBuilder {
        let mut inner = CreateChatInviteLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createChatInviteLink".to_string();

        CreateChatInviteLinkBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
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
pub struct CreateChatInviteLinkBuilder {
    inner: CreateChatInviteLink,
}

#[deprecated]
pub type RTDCreateChatInviteLinkBuilder = CreateChatInviteLinkBuilder;

impl CreateChatInviteLinkBuilder {
    pub fn build(&self) -> CreateChatInviteLink {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
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

impl AsRef<CreateChatInviteLink> for CreateChatInviteLink {
    fn as_ref(&self) -> &CreateChatInviteLink {
        self
    }
}

impl AsRef<CreateChatInviteLink> for CreateChatInviteLinkBuilder {
    fn as_ref(&self) -> &CreateChatInviteLink {
        &self.inner
    }
}
