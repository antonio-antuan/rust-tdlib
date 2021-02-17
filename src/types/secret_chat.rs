use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecretChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Secret chat identifier
    id: i32,
    /// Identifier of the chat partner
    user_id: i32,
    /// State of the secret chat

    #[serde(skip_serializing_if = "SecretChatState::_is_default")]
    state: SecretChatState,
    /// True, if the chat was created by the current user; otherwise false
    is_outbound: bool,
    /// Current message Time To Live setting (self-destruct timer) for the chat, in seconds
    ttl: i32,
    /// Hash of the currently used key for comparison with the hash of the chat partner's key. This is a string of 36 little-endian bytes, which must be split into groups of 2 bits, each denoting a pixel of one of 4 colors FFFFFF, D5E6F3, 2D5775, and 2F99C9. The pixels must be used to make a 12x12 square image filled from left to right, top to bottom. Alternatively, the first 32 bytes of the hash can be converted to the hexadecimal format and printed as 32 2-digit hex numbers
    key_hash: String,
    /// Secret chat layer; determines features supported by the chat partner's application. Video notes are supported if the layer >= 66; nested text entities and underline and strikethrough entities are supported if the layer >= 101
    layer: i32,
}

impl RObject for SecretChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SecretChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSecretChatBuilder {
        let mut inner = SecretChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSecretChatBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn state(&self) -> &SecretChatState {
        &self.state
    }

    pub fn is_outbound(&self) -> bool {
        self.is_outbound
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }

    pub fn key_hash(&self) -> &String {
        &self.key_hash
    }

    pub fn layer(&self) -> i32 {
        self.layer
    }
}

#[doc(hidden)]
pub struct RTDSecretChatBuilder {
    inner: SecretChat,
}

impl RTDSecretChatBuilder {
    pub fn build(&self) -> SecretChat {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn state<T: AsRef<SecretChatState>>(&mut self, state: T) -> &mut Self {
        self.inner.state = state.as_ref().clone();
        self
    }

    pub fn is_outbound(&mut self, is_outbound: bool) -> &mut Self {
        self.inner.is_outbound = is_outbound;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }

    pub fn key_hash<T: AsRef<str>>(&mut self, key_hash: T) -> &mut Self {
        self.inner.key_hash = key_hash.as_ref().to_string();
        self
    }

    pub fn layer(&mut self, layer: i32) -> &mut Self {
        self.inner.layer = layer;
        self
    }
}

impl AsRef<SecretChat> for SecretChat {
    fn as_ref(&self) -> &SecretChat {
        self
    }
}

impl AsRef<SecretChat> for RTDSecretChatBuilder {
    fn as_ref(&self) -> &SecretChat {
        &self.inner
    }
}
