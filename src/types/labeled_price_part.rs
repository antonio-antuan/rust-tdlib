use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Portion of the price of a product (e.g., "delivery cost", "tax amount")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LabeledPricePart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Label for this portion of the product price

    #[serde(default)]
    label: String,
    /// Currency amount in the smallest units of the currency

    #[serde(default)]
    amount: i64,
}

impl RObject for LabeledPricePart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LabeledPricePart {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LabeledPricePartBuilder {
        let mut inner = LabeledPricePart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LabeledPricePartBuilder { inner }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn amount(&self) -> i64 {
        self.amount
    }
}

#[doc(hidden)]
pub struct LabeledPricePartBuilder {
    inner: LabeledPricePart,
}

#[deprecated]
pub type RTDLabeledPricePartBuilder = LabeledPricePartBuilder;

impl LabeledPricePartBuilder {
    pub fn build(&self) -> LabeledPricePart {
        self.inner.clone()
    }

    pub fn label<T: AsRef<str>>(&mut self, label: T) -> &mut Self {
        self.inner.label = label.as_ref().to_string();
        self
    }

    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner.amount = amount;
        self
    }
}

impl AsRef<LabeledPricePart> for LabeledPricePart {
    fn as_ref(&self) -> &LabeledPricePart {
        self
    }
}

impl AsRef<LabeledPricePart> for LabeledPricePartBuilder {
    fn as_ref(&self) -> &LabeledPricePart {
        &self.inner
    }
}
