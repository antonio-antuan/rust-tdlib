use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of sponsored messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SponsoredMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of sponsored messages

    #[serde(default)]
    messages: Vec<SponsoredMessage>,
    /// The minimum number of messages between shown sponsored messages, or 0 if only one sponsored message must be shown after all ordinary messages

    #[serde(default)]
    messages_between: i32,
}

impl RObject for SponsoredMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SponsoredMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SponsoredMessagesBuilder {
        let mut inner = SponsoredMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SponsoredMessagesBuilder { inner }
    }

    pub fn messages(&self) -> &Vec<SponsoredMessage> {
        &self.messages
    }

    pub fn messages_between(&self) -> i32 {
        self.messages_between
    }
}

#[doc(hidden)]
pub struct SponsoredMessagesBuilder {
    inner: SponsoredMessages,
}

#[deprecated]
pub type RTDSponsoredMessagesBuilder = SponsoredMessagesBuilder;

impl SponsoredMessagesBuilder {
    pub fn build(&self) -> SponsoredMessages {
        self.inner.clone()
    }

    pub fn messages(&mut self, messages: Vec<SponsoredMessage>) -> &mut Self {
        self.inner.messages = messages;
        self
    }

    pub fn messages_between(&mut self, messages_between: i32) -> &mut Self {
        self.inner.messages_between = messages_between;
        self
    }
}

impl AsRef<SponsoredMessages> for SponsoredMessages {
    fn as_ref(&self) -> &SponsoredMessages {
        self
    }
}

impl AsRef<SponsoredMessages> for SponsoredMessagesBuilder {
    fn as_ref(&self) -> &SponsoredMessages {
        &self.inner
    }
}
