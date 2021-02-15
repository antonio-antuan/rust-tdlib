use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an action associated with a bank card number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BankCardActionOpenUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Action text
    text: String,
    /// The URL to be opened
    url: String,
}

impl RObject for BankCardActionOpenUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BankCardActionOpenUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBankCardActionOpenUrlBuilder {
        let mut inner = BankCardActionOpenUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBankCardActionOpenUrlBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDBankCardActionOpenUrlBuilder {
    inner: BankCardActionOpenUrl,
}

impl RTDBankCardActionOpenUrlBuilder {
    pub fn build(&self) -> BankCardActionOpenUrl {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<BankCardActionOpenUrl> for BankCardActionOpenUrl {
    fn as_ref(&self) -> &BankCardActionOpenUrl {
        self
    }
}

impl AsRef<BankCardActionOpenUrl> for RTDBankCardActionOpenUrlBuilder {
    fn as_ref(&self) -> &BankCardActionOpenUrl {
        &self.inner
    }
}
