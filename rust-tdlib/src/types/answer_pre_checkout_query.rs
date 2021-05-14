use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets the result of a pre-checkout query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerPreCheckoutQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the pre-checkout query

    #[serde(deserialize_with = "super::_common::number_from_string")]
    pre_checkout_query_id: i64,
    /// An error message, empty on success
    error_message: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerPreCheckoutQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerPreCheckoutQuery {}

impl AnswerPreCheckoutQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAnswerPreCheckoutQueryBuilder {
        let mut inner = AnswerPreCheckoutQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerPreCheckoutQuery".to_string();

        RTDAnswerPreCheckoutQueryBuilder { inner }
    }

    pub fn pre_checkout_query_id(&self) -> i64 {
        self.pre_checkout_query_id
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct RTDAnswerPreCheckoutQueryBuilder {
    inner: AnswerPreCheckoutQuery,
}

impl RTDAnswerPreCheckoutQueryBuilder {
    pub fn build(&self) -> AnswerPreCheckoutQuery {
        self.inner.clone()
    }

    pub fn pre_checkout_query_id(&mut self, pre_checkout_query_id: i64) -> &mut Self {
        self.inner.pre_checkout_query_id = pre_checkout_query_id;
        self
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
        self
    }
}

impl AsRef<AnswerPreCheckoutQuery> for AnswerPreCheckoutQuery {
    fn as_ref(&self) -> &AnswerPreCheckoutQuery {
        self
    }
}

impl AsRef<AnswerPreCheckoutQuery> for RTDAnswerPreCheckoutQueryBuilder {
    fn as_ref(&self) -> &AnswerPreCheckoutQuery {
        &self.inner
    }
}
