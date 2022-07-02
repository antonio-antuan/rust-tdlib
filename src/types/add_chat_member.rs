use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a new member to a chat. Members can't be added to private or secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddChatMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the user

    #[serde(default)]
    user_id: i64,
    /// The number of earlier messages from the chat to be forwarded to the new member; up to 100. Ignored for supergroups and channels, or if the added user is a bot

    #[serde(default)]
    forward_limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddChatMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddChatMember {}

impl AddChatMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddChatMemberBuilder {
        let mut inner = AddChatMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addChatMember".to_string();

        AddChatMemberBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn forward_limit(&self) -> i32 {
        self.forward_limit
    }
}

#[doc(hidden)]
pub struct AddChatMemberBuilder {
    inner: AddChatMember,
}

#[deprecated]
pub type RTDAddChatMemberBuilder = AddChatMemberBuilder;

impl AddChatMemberBuilder {
    pub fn build(&self) -> AddChatMember {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn forward_limit(&mut self, forward_limit: i32) -> &mut Self {
        self.inner.forward_limit = forward_limit;
        self
    }
}

impl AsRef<AddChatMember> for AddChatMember {
    fn as_ref(&self) -> &AddChatMember {
        self
    }
}

impl AsRef<AddChatMember> for AddChatMemberBuilder {
    fn as_ref(&self) -> &AddChatMember {
        &self.inner
    }
}
