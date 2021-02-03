use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about a bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Title of the bank card description
    title: String,
    /// Actions that can be done with the bank card number
    actions: Vec<BankCardActionOpenUrl>,
}

impl RObject for BankCardInfo {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "bankCardInfo"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl BankCardInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBankCardInfoBuilder {
        let mut inner = BankCardInfo::default();
        inner.td_name = "bankCardInfo".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDBankCardInfoBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn actions(&self) -> &Vec<BankCardActionOpenUrl> {
        &self.actions
    }
}

#[doc(hidden)]
pub struct RTDBankCardInfoBuilder {
    inner: BankCardInfo,
}

impl RTDBankCardInfoBuilder {
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

impl AsRef<BankCardInfo> for RTDBankCardInfoBuilder {
    fn as_ref(&self) -> &BankCardInfo {
        &self.inner
    }
}
