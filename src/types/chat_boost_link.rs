use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTPS link to boost a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The link

    #[serde(default)]
    link: String,
    /// True, if the link will work for non-members of the chat

    #[serde(default)]
    is_public: bool,
}

impl RObject for ChatBoostLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoostLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostLinkBuilder {
        let mut inner = ChatBoostLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostLinkBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }
}

#[doc(hidden)]
pub struct ChatBoostLinkBuilder {
    inner: ChatBoostLink,
}

#[deprecated]
pub type RTDChatBoostLinkBuilder = ChatBoostLinkBuilder;

impl ChatBoostLinkBuilder {
    pub fn build(&self) -> ChatBoostLink {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }

    pub fn is_public(&mut self, is_public: bool) -> &mut Self {
        self.inner.is_public = is_public;
        self
    }
}

impl AsRef<ChatBoostLink> for ChatBoostLink {
    fn as_ref(&self) -> &ChatBoostLink {
        self
    }
}

impl AsRef<ChatBoostLink> for ChatBoostLinkBuilder {
    fn as_ref(&self) -> &ChatBoostLink {
        &self.inner
    }
}
