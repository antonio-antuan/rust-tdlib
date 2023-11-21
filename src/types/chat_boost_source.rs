use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes source of a chat boost
pub trait TDChatBoostSource: Debug + RObject {}

/// Describes source of a chat boost
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ChatBoostSource {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The chat created a Telegram Premium gift code for a user
    #[serde(rename = "chatBoostSourceGiftCode")]
    GiftCode(ChatBoostSourceGiftCode),
    /// The chat created a Telegram Premium giveaway
    #[serde(rename = "chatBoostSourceGiveaway")]
    Giveaway(ChatBoostSourceGiveaway),
    /// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
    #[serde(rename = "chatBoostSourcePremium")]
    Premium(ChatBoostSourcePremium),
}

impl RObject for ChatBoostSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatBoostSource::GiftCode(t) => t.extra(),
            ChatBoostSource::Giveaway(t) => t.extra(),
            ChatBoostSource::Premium(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatBoostSource::GiftCode(t) => t.client_id(),
            ChatBoostSource::Giveaway(t) => t.client_id(),
            ChatBoostSource::Premium(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatBoostSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatBoostSource::_Default)
    }
}

impl AsRef<ChatBoostSource> for ChatBoostSource {
    fn as_ref(&self) -> &ChatBoostSource {
        self
    }
}

/// The chat created a Telegram Premium gift code for a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a user, for which the gift code was created

    #[serde(default)]
    user_id: i64,
    /// The created Telegram Premium gift code, which is known only if this is a gift code for the current user, or it has already been claimed

    #[serde(default)]
    gift_code: String,
}

impl RObject for ChatBoostSourceGiftCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatBoostSource for ChatBoostSourceGiftCode {}

impl ChatBoostSourceGiftCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostSourceGiftCodeBuilder {
        let mut inner = ChatBoostSourceGiftCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostSourceGiftCodeBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn gift_code(&self) -> &String {
        &self.gift_code
    }
}

#[doc(hidden)]
pub struct ChatBoostSourceGiftCodeBuilder {
    inner: ChatBoostSourceGiftCode,
}

#[deprecated]
pub type RTDChatBoostSourceGiftCodeBuilder = ChatBoostSourceGiftCodeBuilder;

impl ChatBoostSourceGiftCodeBuilder {
    pub fn build(&self) -> ChatBoostSourceGiftCode {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
        self.inner.gift_code = gift_code.as_ref().to_string();
        self
    }
}

impl AsRef<ChatBoostSourceGiftCode> for ChatBoostSourceGiftCode {
    fn as_ref(&self) -> &ChatBoostSourceGiftCode {
        self
    }
}

impl AsRef<ChatBoostSourceGiftCode> for ChatBoostSourceGiftCodeBuilder {
    fn as_ref(&self) -> &ChatBoostSourceGiftCode {
        &self.inner
    }
}

/// The chat created a Telegram Premium giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a user that won in the giveaway; 0 if none

    #[serde(default)]
    user_id: i64,
    /// The created Telegram Premium gift code if it was used by the user or can be claimed by the current user; an empty string otherwise

    #[serde(default)]
    gift_code: String,
    /// Identifier of the corresponding giveaway message; can be an identifier of a deleted message

    #[serde(default)]
    giveaway_message_id: i64,
    /// True, if the winner for the corresponding Telegram Premium subscription wasn't chosen, because there were not enough participants

    #[serde(default)]
    is_unclaimed: bool,
}

impl RObject for ChatBoostSourceGiveaway {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatBoostSource for ChatBoostSourceGiveaway {}

impl ChatBoostSourceGiveaway {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostSourceGiveawayBuilder {
        let mut inner = ChatBoostSourceGiveaway::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostSourceGiveawayBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn gift_code(&self) -> &String {
        &self.gift_code
    }

    pub fn giveaway_message_id(&self) -> i64 {
        self.giveaway_message_id
    }

    pub fn is_unclaimed(&self) -> bool {
        self.is_unclaimed
    }
}

#[doc(hidden)]
pub struct ChatBoostSourceGiveawayBuilder {
    inner: ChatBoostSourceGiveaway,
}

#[deprecated]
pub type RTDChatBoostSourceGiveawayBuilder = ChatBoostSourceGiveawayBuilder;

impl ChatBoostSourceGiveawayBuilder {
    pub fn build(&self) -> ChatBoostSourceGiveaway {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn gift_code<T: AsRef<str>>(&mut self, gift_code: T) -> &mut Self {
        self.inner.gift_code = gift_code.as_ref().to_string();
        self
    }

    pub fn giveaway_message_id(&mut self, giveaway_message_id: i64) -> &mut Self {
        self.inner.giveaway_message_id = giveaway_message_id;
        self
    }

    pub fn is_unclaimed(&mut self, is_unclaimed: bool) -> &mut Self {
        self.inner.is_unclaimed = is_unclaimed;
        self
    }
}

impl AsRef<ChatBoostSourceGiveaway> for ChatBoostSourceGiveaway {
    fn as_ref(&self) -> &ChatBoostSourceGiveaway {
        self
    }
}

impl AsRef<ChatBoostSourceGiveaway> for ChatBoostSourceGiveawayBuilder {
    fn as_ref(&self) -> &ChatBoostSourceGiveaway {
        &self.inner
    }
}

/// A user with Telegram Premium subscription or gifted Telegram Premium boosted the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user

    #[serde(default)]
    user_id: i64,
}

impl RObject for ChatBoostSourcePremium {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatBoostSource for ChatBoostSourcePremium {}

impl ChatBoostSourcePremium {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostSourcePremiumBuilder {
        let mut inner = ChatBoostSourcePremium::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostSourcePremiumBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct ChatBoostSourcePremiumBuilder {
    inner: ChatBoostSourcePremium,
}

#[deprecated]
pub type RTDChatBoostSourcePremiumBuilder = ChatBoostSourcePremiumBuilder;

impl ChatBoostSourcePremiumBuilder {
    pub fn build(&self) -> ChatBoostSourcePremium {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<ChatBoostSourcePremium> for ChatBoostSourcePremium {
    fn as_ref(&self) -> &ChatBoostSourcePremium {
        self
    }
}

impl AsRef<ChatBoostSourcePremium> for ChatBoostSourcePremiumBuilder {
    fn as_ref(&self) -> &ChatBoostSourcePremium {
        &self.inner
    }
}
