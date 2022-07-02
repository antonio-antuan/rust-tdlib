use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns pending join requests in a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatJoinRequests {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Invite link for which to return join requests. If empty, all join requests will be returned. Requires administrator privileges and can_invite_users right in the chat for own links and owner privileges for other links

    #[serde(default)]
    invite_link: String,
    /// A query to search for in the first names, last names and usernames of the users to return

    #[serde(default)]
    query: String,
    /// A chat join request from which to return next requests; pass null to get results from the beginning
    offset_request: ChatJoinRequest,
    /// The maximum number of requests to join the chat to return

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatJoinRequests {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatJoinRequests {}

impl GetChatJoinRequests {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatJoinRequestsBuilder {
        let mut inner = GetChatJoinRequests::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatJoinRequests".to_string();

        GetChatJoinRequestsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn invite_link(&self) -> &String {
        &self.invite_link
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset_request(&self) -> &ChatJoinRequest {
        &self.offset_request
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatJoinRequestsBuilder {
    inner: GetChatJoinRequests,
}

#[deprecated]
pub type RTDGetChatJoinRequestsBuilder = GetChatJoinRequestsBuilder;

impl GetChatJoinRequestsBuilder {
    pub fn build(&self) -> GetChatJoinRequests {
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

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn offset_request<T: AsRef<ChatJoinRequest>>(&mut self, offset_request: T) -> &mut Self {
        self.inner.offset_request = offset_request.as_ref().clone();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatJoinRequests> for GetChatJoinRequests {
    fn as_ref(&self) -> &GetChatJoinRequests {
        self
    }
}

impl AsRef<GetChatJoinRequests> for GetChatJoinRequestsBuilder {
    fn as_ref(&self) -> &GetChatJoinRequests {
        &self.inner
    }
}
