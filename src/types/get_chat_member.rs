use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a single member of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Member identifier

    #[serde(skip_serializing_if = "MessageSender::_is_default")]
    member_id: MessageSender,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatMember {}

impl GetChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatMemberBuilder {
        let mut inner = GetChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatMember".to_string();

        RTDGetChatMemberBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn member_id(&self) -> &MessageSender {
        &self.member_id
    }
}

#[doc(hidden)]
pub struct RTDGetChatMemberBuilder {
    inner: GetChatMember,
}

impl RTDGetChatMemberBuilder {
    pub fn build(&self) -> GetChatMember {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn member_id<T: AsRef<MessageSender>>(&mut self, member_id: T) -> &mut Self {
        self.inner.member_id = member_id.as_ref().clone();
        self
    }
}

impl AsRef<GetChatMember> for GetChatMember {
    fn as_ref(&self) -> &GetChatMember {
        self
    }
}

impl AsRef<GetChatMember> for RTDGetChatMemberBuilder {
    fn as_ref(&self) -> &GetChatMember {
        &self.inner
    }
}
