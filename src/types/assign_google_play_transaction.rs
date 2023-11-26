use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs server about a purchase through Google Play. For official applications only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssignGooglePlayTransaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Application package name

    #[serde(default)]
    package_name: String,
    /// Identifier of the purchased store product

    #[serde(default)]
    store_product_id: String,
    /// Google Play purchase token

    #[serde(default)]
    purchase_token: String,
    /// Transaction purpose

    #[serde(skip_serializing_if = "StorePaymentPurpose::_is_default")]
    purpose: StorePaymentPurpose,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AssignGooglePlayTransaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AssignGooglePlayTransaction {}

impl AssignGooglePlayTransaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AssignGooglePlayTransactionBuilder {
        let mut inner = AssignGooglePlayTransaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "assignGooglePlayTransaction".to_string();

        AssignGooglePlayTransactionBuilder { inner }
    }

    pub fn package_name(&self) -> &String {
        &self.package_name
    }

    pub fn store_product_id(&self) -> &String {
        &self.store_product_id
    }

    pub fn purchase_token(&self) -> &String {
        &self.purchase_token
    }

    pub fn purpose(&self) -> &StorePaymentPurpose {
        &self.purpose
    }
}

#[doc(hidden)]
pub struct AssignGooglePlayTransactionBuilder {
    inner: AssignGooglePlayTransaction,
}

#[deprecated]
pub type RTDAssignGooglePlayTransactionBuilder = AssignGooglePlayTransactionBuilder;

impl AssignGooglePlayTransactionBuilder {
    pub fn build(&self) -> AssignGooglePlayTransaction {
        self.inner.clone()
    }

    pub fn package_name<T: AsRef<str>>(&mut self, package_name: T) -> &mut Self {
        self.inner.package_name = package_name.as_ref().to_string();
        self
    }

    pub fn store_product_id<T: AsRef<str>>(&mut self, store_product_id: T) -> &mut Self {
        self.inner.store_product_id = store_product_id.as_ref().to_string();
        self
    }

    pub fn purchase_token<T: AsRef<str>>(&mut self, purchase_token: T) -> &mut Self {
        self.inner.purchase_token = purchase_token.as_ref().to_string();
        self
    }

    pub fn purpose<T: AsRef<StorePaymentPurpose>>(&mut self, purpose: T) -> &mut Self {
        self.inner.purpose = purpose.as_ref().clone();
        self
    }
}

impl AsRef<AssignGooglePlayTransaction> for AssignGooglePlayTransaction {
    fn as_ref(&self) -> &AssignGooglePlayTransaction {
        self
    }
}

impl AsRef<AssignGooglePlayTransaction> for AssignGooglePlayTransactionBuilder {
    fn as_ref(&self) -> &AssignGooglePlayTransaction {
        &self.inner
    }
}
