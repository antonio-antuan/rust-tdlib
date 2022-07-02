use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Information about a bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the bank card description

    #[serde(default)]
    title: String,
    /// Actions that can be done with the bank card number

    #[serde(default)]
    actions: Vec<BankCardActionOpenUrl>,
}

impl RObject for BankCardInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BankCardInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BankCardInfoBuilder {
        let mut inner = BankCardInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BankCardInfoBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn actions(&self) -> &Vec<BankCardActionOpenUrl> {
        &self.actions
    }
}

#[doc(hidden)]
pub struct BankCardInfoBuilder {
    inner: BankCardInfo,
}

#[deprecated]
pub type RTDBankCardInfoBuilder = BankCardInfoBuilder;

impl BankCardInfoBuilder {
    pub fn build(&self) -> BankCardInfo {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn actions(&mut self, actions: Vec<BankCardActionOpenUrl>) -> &mut Self {
        self.inner.actions = actions;
        self
    }
}

impl AsRef<BankCardInfo> for BankCardInfo {
    fn as_ref(&self) -> &BankCardInfo {
        self
    }
}

impl AsRef<BankCardInfo> for BankCardInfoBuilder {
    fn as_ref(&self) -> &BankCardInfo {
        &self.inner
    }
}
