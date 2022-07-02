use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a list of public chats of the specified type, owned by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCreatedPublicChats {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the public chats to return

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PublicChatType::_is_default")]
    type_: PublicChatType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCreatedPublicChats {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCreatedPublicChats {}

impl GetCreatedPublicChats {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCreatedPublicChatsBuilder {
        let mut inner = GetCreatedPublicChats::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCreatedPublicChats".to_string();

        GetCreatedPublicChatsBuilder { inner }
    }

    pub fn type_(&self) -> &PublicChatType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct GetCreatedPublicChatsBuilder {
    inner: GetCreatedPublicChats,
}

#[deprecated]
pub type RTDGetCreatedPublicChatsBuilder = GetCreatedPublicChatsBuilder;

impl GetCreatedPublicChatsBuilder {
    pub fn build(&self) -> GetCreatedPublicChats {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PublicChatType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<GetCreatedPublicChats> for GetCreatedPublicChats {
    fn as_ref(&self) -> &GetCreatedPublicChats {
        self
    }
}

impl AsRef<GetCreatedPublicChats> for GetCreatedPublicChatsBuilder {
    fn as_ref(&self) -> &GetCreatedPublicChats {
        &self.inner
    }
}
