use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the result of a shipping query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerShippingQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the shipping query

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    shipping_query_id: i64,
    /// Available shipping options

    #[serde(default)]
    shipping_options: Vec<ShippingOption>,
    /// An error message, empty on success

    #[serde(default)]
    error_message: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerShippingQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerShippingQuery {}

impl AnswerShippingQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnswerShippingQueryBuilder {
        let mut inner = AnswerShippingQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerShippingQuery".to_string();

        AnswerShippingQueryBuilder { inner }
    }

    pub fn shipping_query_id(&self) -> i64 {
        self.shipping_query_id
    }

    pub fn shipping_options(&self) -> &Vec<ShippingOption> {
        &self.shipping_options
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct AnswerShippingQueryBuilder {
    inner: AnswerShippingQuery,
}

#[deprecated]
pub type RTDAnswerShippingQueryBuilder = AnswerShippingQueryBuilder;

impl AnswerShippingQueryBuilder {
    pub fn build(&self) -> AnswerShippingQuery {
        self.inner.clone()
    }

    pub fn shipping_query_id(&mut self, shipping_query_id: i64) -> &mut Self {
        self.inner.shipping_query_id = shipping_query_id;
        self
    }

    pub fn shipping_options(&mut self, shipping_options: Vec<ShippingOption>) -> &mut Self {
        self.inner.shipping_options = shipping_options;
        self
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
        self
    }
}

impl AsRef<AnswerShippingQuery> for AnswerShippingQuery {
    fn as_ref(&self) -> &AnswerShippingQuery {
        self
    }
}

impl AsRef<AnswerShippingQuery> for AnswerShippingQueryBuilder {
    fn as_ref(&self) -> &AnswerShippingQuery {
        &self.inner
    }
}
