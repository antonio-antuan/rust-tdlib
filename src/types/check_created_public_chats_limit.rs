use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckCreatedPublicChatsLimit {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the public chats, for which to check the limit

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PublicChatType::_is_default")]
    type_: PublicChatType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckCreatedPublicChatsLimit {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckCreatedPublicChatsLimit {}

impl CheckCreatedPublicChatsLimit {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckCreatedPublicChatsLimitBuilder {
        let mut inner = CheckCreatedPublicChatsLimit::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkCreatedPublicChatsLimit".to_string();

        CheckCreatedPublicChatsLimitBuilder { inner }
    }

    pub fn type_(&self) -> &PublicChatType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct CheckCreatedPublicChatsLimitBuilder {
    inner: CheckCreatedPublicChatsLimit,
}

#[deprecated]
pub type RTDCheckCreatedPublicChatsLimitBuilder = CheckCreatedPublicChatsLimitBuilder;

impl CheckCreatedPublicChatsLimitBuilder {
    pub fn build(&self) -> CheckCreatedPublicChatsLimit {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PublicChatType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<CheckCreatedPublicChatsLimit> for CheckCreatedPublicChatsLimit {
    fn as_ref(&self) -> &CheckCreatedPublicChatsLimit {
        self
    }
}

impl AsRef<CheckCreatedPublicChatsLimit> for CheckCreatedPublicChatsLimitBuilder {
    fn as_ref(&self) -> &CheckCreatedPublicChatsLimit {
        &self.inner
    }
}
