use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Applies a Telegram Premium gift code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplyPremiumGiftCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The code to apply

    #[serde(default)]
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ApplyPremiumGiftCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ApplyPremiumGiftCode {}

impl ApplyPremiumGiftCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ApplyPremiumGiftCodeBuilder {
        let mut inner = ApplyPremiumGiftCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "applyPremiumGiftCode".to_string();

        ApplyPremiumGiftCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct ApplyPremiumGiftCodeBuilder {
    inner: ApplyPremiumGiftCode,
}

#[deprecated]
pub type RTDApplyPremiumGiftCodeBuilder = ApplyPremiumGiftCodeBuilder;

impl ApplyPremiumGiftCodeBuilder {
    pub fn build(&self) -> ApplyPremiumGiftCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<ApplyPremiumGiftCode> for ApplyPremiumGiftCode {
    fn as_ref(&self) -> &ApplyPremiumGiftCode {
        self
    }
}

impl AsRef<ApplyPremiumGiftCode> for ApplyPremiumGiftCodeBuilder {
    fn as_ref(&self) -> &ApplyPremiumGiftCode {
        &self.inner
    }
}
