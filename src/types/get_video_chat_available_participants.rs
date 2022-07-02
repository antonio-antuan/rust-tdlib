use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of participant identifiers, on whose behalf a video chat in the chat can be joined
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetVideoChatAvailableParticipants {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetVideoChatAvailableParticipants {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetVideoChatAvailableParticipants {}

impl GetVideoChatAvailableParticipants {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetVideoChatAvailableParticipantsBuilder {
        let mut inner = GetVideoChatAvailableParticipants::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getVideoChatAvailableParticipants".to_string();

        GetVideoChatAvailableParticipantsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetVideoChatAvailableParticipantsBuilder {
    inner: GetVideoChatAvailableParticipants,
}

#[deprecated]
pub type RTDGetVideoChatAvailableParticipantsBuilder = GetVideoChatAvailableParticipantsBuilder;

impl GetVideoChatAvailableParticipantsBuilder {
    pub fn build(&self) -> GetVideoChatAvailableParticipants {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetVideoChatAvailableParticipants> for GetVideoChatAvailableParticipants {
    fn as_ref(&self) -> &GetVideoChatAvailableParticipants {
        self
    }
}

impl AsRef<GetVideoChatAvailableParticipants> for GetVideoChatAvailableParticipantsBuilder {
    fn as_ref(&self) -> &GetVideoChatAvailableParticipants {
        &self.inner
    }
}
