use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a user that sent a join request and waits for administrator approval
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// Point in time (Unix timestamp) when the user sent the join request

    #[serde(default)]
    date: i32,
    /// A short bio of the user

    #[serde(default)]
    bio: String,
}

impl RObject for ChatJoinRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatJoinRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatJoinRequestBuilder {
        let mut inner = ChatJoinRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatJoinRequestBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn bio(&self) -> &String {
        &self.bio
    }
}

#[doc(hidden)]
pub struct ChatJoinRequestBuilder {
    inner: ChatJoinRequest,
}

#[deprecated]
pub type RTDChatJoinRequestBuilder = ChatJoinRequestBuilder;

impl ChatJoinRequestBuilder {
    pub fn build(&self) -> ChatJoinRequest {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
        self.inner.bio = bio.as_ref().to_string();
        self
    }
}

impl AsRef<ChatJoinRequest> for ChatJoinRequest {
    fn as_ref(&self) -> &ChatJoinRequest {
        self
    }
}

impl AsRef<ChatJoinRequest> for ChatJoinRequestBuilder {
    fn as_ref(&self) -> &ChatJoinRequest {
        &self.inner
    }
}
