use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Return information about a Telegram Premium gift code
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckPremiumGiftCode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The code to check

    #[serde(default)]
    code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckPremiumGiftCode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckPremiumGiftCode {}

impl CheckPremiumGiftCode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckPremiumGiftCodeBuilder {
        let mut inner = CheckPremiumGiftCode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkPremiumGiftCode".to_string();

        CheckPremiumGiftCodeBuilder { inner }
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}

#[doc(hidden)]
pub struct CheckPremiumGiftCodeBuilder {
    inner: CheckPremiumGiftCode,
}

#[deprecated]
pub type RTDCheckPremiumGiftCodeBuilder = CheckPremiumGiftCodeBuilder;

impl CheckPremiumGiftCodeBuilder {
    pub fn build(&self) -> CheckPremiumGiftCode {
        self.inner.clone()
    }

    pub fn code<T: AsRef<str>>(&mut self, code: T) -> &mut Self {
        self.inner.code = code.as_ref().to_string();
        self
    }
}

impl AsRef<CheckPremiumGiftCode> for CheckPremiumGiftCode {
    fn as_ref(&self) -> &CheckPremiumGiftCode {
        self
    }
}

impl AsRef<CheckPremiumGiftCode> for CheckPremiumGiftCodeBuilder {
    fn as_ref(&self) -> &CheckPremiumGiftCode {
        &self.inner
    }
}
