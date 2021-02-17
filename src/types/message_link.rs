use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTPS link to a message in a supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message link
    link: String,
    /// True, if the link will work for non-members of the chat
    is_public: bool,
}

impl RObject for MessageLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDMessageLinkBuilder {
        let mut inner = MessageLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDMessageLinkBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }
}

#[doc(hidden)]
pub struct RTDMessageLinkBuilder {
    inner: MessageLink,
}

impl RTDMessageLinkBuilder {
    pub fn build(&self) -> MessageLink {
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

impl AsRef<MessageLink> for MessageLink {
    fn as_ref(&self) -> &MessageLink {
        self
    }
}

impl AsRef<MessageLink> for RTDMessageLinkBuilder {
    fn as_ref(&self) -> &MessageLink {
        &self.inner
    }
}
