use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBankCardInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The bank card number

    #[serde(default)]
    bank_card_number: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBankCardInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBankCardInfo {}

impl GetBankCardInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetBankCardInfoBuilder {
        let mut inner = GetBankCardInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBankCardInfo".to_string();

        GetBankCardInfoBuilder { inner }
    }

    pub fn bank_card_number(&self) -> &String {
        &self.bank_card_number
    }
}

#[doc(hidden)]
pub struct GetBankCardInfoBuilder {
    inner: GetBankCardInfo,
}

#[deprecated]
pub type RTDGetBankCardInfoBuilder = GetBankCardInfoBuilder;

impl GetBankCardInfoBuilder {
    pub fn build(&self) -> GetBankCardInfo {
        self.inner.clone()
    }

    pub fn bank_card_number<T: AsRef<str>>(&mut self, bank_card_number: T) -> &mut Self {
        self.inner.bank_card_number = bank_card_number.as_ref().to_string();
        self
    }
}

impl AsRef<GetBankCardInfo> for GetBankCardInfo {
    fn as_ref(&self) -> &GetBankCardInfo {
        self
    }
}

impl AsRef<GetBankCardInfo> for GetBankCardInfoBuilder {
    fn as_ref(&self) -> &GetBankCardInfo {
        &self.inner
    }
}
