use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs server about a purchase through App Store. For official applications only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssignAppStoreTransaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// App Store receipt

    #[serde(default)]
    receipt: String,
    /// Transaction purpose

    #[serde(skip_serializing_if = "StorePaymentPurpose::_is_default")]
    purpose: StorePaymentPurpose,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AssignAppStoreTransaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AssignAppStoreTransaction {}

impl AssignAppStoreTransaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AssignAppStoreTransactionBuilder {
        let mut inner = AssignAppStoreTransaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "assignAppStoreTransaction".to_string();

        AssignAppStoreTransactionBuilder { inner }
    }

    pub fn receipt(&self) -> &String {
        &self.receipt
    }

    pub fn purpose(&self) -> &StorePaymentPurpose {
        &self.purpose
    }
}

#[doc(hidden)]
pub struct AssignAppStoreTransactionBuilder {
    inner: AssignAppStoreTransaction,
}

#[deprecated]
pub type RTDAssignAppStoreTransactionBuilder = AssignAppStoreTransactionBuilder;

impl AssignAppStoreTransactionBuilder {
    pub fn build(&self) -> AssignAppStoreTransaction {
        self.inner.clone()
    }

    pub fn receipt<T: AsRef<str>>(&mut self, receipt: T) -> &mut Self {
        self.inner.receipt = receipt.as_ref().to_string();
        self
    }

    pub fn purpose<T: AsRef<StorePaymentPurpose>>(&mut self, purpose: T) -> &mut Self {
        self.inner.purpose = purpose.as_ref().clone();
        self
    }
}

impl AsRef<AssignAppStoreTransaction> for AssignAppStoreTransaction {
    fn as_ref(&self) -> &AssignAppStoreTransaction {
        self
    }
}

impl AsRef<AssignAppStoreTransaction> for AssignAppStoreTransactionBuilder {
    fn as_ref(&self) -> &AssignAppStoreTransaction {
        &self.inner
    }
}
