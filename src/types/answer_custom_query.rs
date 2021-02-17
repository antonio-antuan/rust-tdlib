use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Answers a custom query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerCustomQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a custom query

    #[serde(deserialize_with = "super::_common::number_from_string")]
    custom_query_id: i64,
    /// JSON-serialized answer to the query
    data: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerCustomQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerCustomQuery {}

impl AnswerCustomQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAnswerCustomQueryBuilder {
        let mut inner = AnswerCustomQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerCustomQuery".to_string();

        RTDAnswerCustomQueryBuilder { inner }
    }

    pub fn custom_query_id(&self) -> i64 {
        self.custom_query_id
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDAnswerCustomQueryBuilder {
    inner: AnswerCustomQuery,
}

impl RTDAnswerCustomQueryBuilder {
    pub fn build(&self) -> AnswerCustomQuery {
        self.inner.clone()
    }

    pub fn custom_query_id(&mut self, custom_query_id: i64) -> &mut Self {
        self.inner.custom_query_id = custom_query_id;
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<AnswerCustomQuery> for AnswerCustomQuery {
    fn as_ref(&self) -> &AnswerCustomQuery {
        self
    }
}

impl AsRef<AnswerCustomQuery> for RTDAnswerCustomQueryBuilder {
    fn as_ref(&self) -> &AnswerCustomQuery {
        &self.inner
    }
}
