use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an option for buying or upgrading Telegram Premium for self
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStatePaymentOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information about the payment option
    payment_option: PremiumPaymentOption,
    /// True, if this is the currently used Telegram Premium subscription option

    #[serde(default)]
    is_current: bool,
    /// True, if the payment option can be used to upgrade the existing Telegram Premium subscription

    #[serde(default)]
    is_upgrade: bool,
    /// Identifier of the last in-store transaction for the currently used option

    #[serde(default)]
    last_transaction_id: String,
}

impl RObject for PremiumStatePaymentOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PremiumStatePaymentOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStatePaymentOptionBuilder {
        let mut inner = PremiumStatePaymentOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStatePaymentOptionBuilder { inner }
    }

    pub fn payment_option(&self) -> &PremiumPaymentOption {
        &self.payment_option
    }

    pub fn is_current(&self) -> bool {
        self.is_current
    }

    pub fn is_upgrade(&self) -> bool {
        self.is_upgrade
    }

    pub fn last_transaction_id(&self) -> &String {
        &self.last_transaction_id
    }
}

#[doc(hidden)]
pub struct PremiumStatePaymentOptionBuilder {
    inner: PremiumStatePaymentOption,
}

#[deprecated]
pub type RTDPremiumStatePaymentOptionBuilder = PremiumStatePaymentOptionBuilder;

impl PremiumStatePaymentOptionBuilder {
    pub fn build(&self) -> PremiumStatePaymentOption {
        self.inner.clone()
    }

    pub fn payment_option<T: AsRef<PremiumPaymentOption>>(
        &mut self,
        payment_option: T,
    ) -> &mut Self {
        self.inner.payment_option = payment_option.as_ref().clone();
        self
    }

    pub fn is_current(&mut self, is_current: bool) -> &mut Self {
        self.inner.is_current = is_current;
        self
    }

    pub fn is_upgrade(&mut self, is_upgrade: bool) -> &mut Self {
        self.inner.is_upgrade = is_upgrade;
        self
    }

    pub fn last_transaction_id<T: AsRef<str>>(&mut self, last_transaction_id: T) -> &mut Self {
        self.inner.last_transaction_id = last_transaction_id.as_ref().to_string();
        self
    }
}

impl AsRef<PremiumStatePaymentOption> for PremiumStatePaymentOption {
    fn as_ref(&self) -> &PremiumStatePaymentOption {
        self
    }
}

impl AsRef<PremiumStatePaymentOption> for PremiumStatePaymentOptionBuilder {
    fn as_ref(&self) -> &PremiumStatePaymentOption {
        &self.inner
    }
}
