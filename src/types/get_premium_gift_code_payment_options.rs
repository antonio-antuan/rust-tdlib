use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns available options for Telegram Premium gift code or giveaway creation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPremiumGiftCodePaymentOptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the channel chat, which will be automatically boosted by receivers of the gift codes and which is administered by the user; 0 if none

    #[serde(default)]
    boosted_chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPremiumGiftCodePaymentOptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPremiumGiftCodePaymentOptions {}

impl GetPremiumGiftCodePaymentOptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPremiumGiftCodePaymentOptionsBuilder {
        let mut inner = GetPremiumGiftCodePaymentOptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPremiumGiftCodePaymentOptions".to_string();

        GetPremiumGiftCodePaymentOptionsBuilder { inner }
    }

    pub fn boosted_chat_id(&self) -> i64 {
        self.boosted_chat_id
    }
}

#[doc(hidden)]
pub struct GetPremiumGiftCodePaymentOptionsBuilder {
    inner: GetPremiumGiftCodePaymentOptions,
}

#[deprecated]
pub type RTDGetPremiumGiftCodePaymentOptionsBuilder = GetPremiumGiftCodePaymentOptionsBuilder;

impl GetPremiumGiftCodePaymentOptionsBuilder {
    pub fn build(&self) -> GetPremiumGiftCodePaymentOptions {
        self.inner.clone()
    }

    pub fn boosted_chat_id(&mut self, boosted_chat_id: i64) -> &mut Self {
        self.inner.boosted_chat_id = boosted_chat_id;
        self
    }
}

impl AsRef<GetPremiumGiftCodePaymentOptions> for GetPremiumGiftCodePaymentOptions {
    fn as_ref(&self) -> &GetPremiumGiftCodePaymentOptions {
        self
    }
}

impl AsRef<GetPremiumGiftCodePaymentOptions> for GetPremiumGiftCodePaymentOptionsBuilder {
    fn as_ref(&self) -> &GetPremiumGiftCodePaymentOptions {
        &self.inner
    }
}
