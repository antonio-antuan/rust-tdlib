use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns invite links for a chat created by specified administrator. Requires administrator privileges and can_invite_users right in the chat to get own links and owner privileges to get other links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatInviteLinks {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// User identifier of a chat administrator. Must be an identifier of the current user for non-owner

    #[serde(default)]
    creator_user_id: i64,
    /// Pass true if revoked links needs to be returned instead of active or expired

    #[serde(default)]
    is_revoked: bool,
    /// Creation date of an invite link starting after which to return invite links; use 0 to get results from the beginning

    #[serde(default)]
    offset_date: i32,
    /// Invite link starting after which to return invite links; use empty string to get results from the beginning

    #[serde(default)]
    offset_invite_link: String,
    /// The maximum number of invite links to return; up to 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatInviteLinks {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatInviteLinks {}

impl GetChatInviteLinks {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatInviteLinksBuilder {
        let mut inner = GetChatInviteLinks::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatInviteLinks".to_string();

        GetChatInviteLinksBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn creator_user_id(&self) -> i64 {
        self.creator_user_id
    }

    pub fn is_revoked(&self) -> bool {
        self.is_revoked
    }

    pub fn offset_date(&self) -> i32 {
        self.offset_date
    }

    pub fn offset_invite_link(&self) -> &String {
        &self.offset_invite_link
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatInviteLinksBuilder {
    inner: GetChatInviteLinks,
}

#[deprecated]
pub type RTDGetChatInviteLinksBuilder = GetChatInviteLinksBuilder;

impl GetChatInviteLinksBuilder {
    pub fn build(&self) -> GetChatInviteLinks {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn creator_user_id(&mut self, creator_user_id: i64) -> &mut Self {
        self.inner.creator_user_id = creator_user_id;
        self
    }

    pub fn is_revoked(&mut self, is_revoked: bool) -> &mut Self {
        self.inner.is_revoked = is_revoked;
        self
    }

    pub fn offset_date(&mut self, offset_date: i32) -> &mut Self {
        self.inner.offset_date = offset_date;
        self
    }

    pub fn offset_invite_link<T: AsRef<str>>(&mut self, offset_invite_link: T) -> &mut Self {
        self.inner.offset_invite_link = offset_invite_link.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatInviteLinks> for GetChatInviteLinks {
    fn as_ref(&self) -> &GetChatInviteLinks {
        self
    }
}

impl AsRef<GetChatInviteLinks> for GetChatInviteLinksBuilder {
    fn as_ref(&self) -> &GetChatInviteLinks {
        &self.inner
    }
}
