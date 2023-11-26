use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the result of interaction with a Web App and sends corresponding message on behalf of the user to the chat from which the query originated; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerWebAppQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the Web App query

    #[serde(default)]
    web_app_query_id: String,
    /// The result of the query

    #[serde(skip_serializing_if = "InputInlineQueryResult::_is_default")]
    result: InputInlineQueryResult,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerWebAppQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerWebAppQuery {}

impl AnswerWebAppQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnswerWebAppQueryBuilder {
        let mut inner = AnswerWebAppQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerWebAppQuery".to_string();

        AnswerWebAppQueryBuilder { inner }
    }

    pub fn web_app_query_id(&self) -> &String {
        &self.web_app_query_id
    }

    pub fn result(&self) -> &InputInlineQueryResult {
        &self.result
    }
}

#[doc(hidden)]
pub struct AnswerWebAppQueryBuilder {
    inner: AnswerWebAppQuery,
}

#[deprecated]
pub type RTDAnswerWebAppQueryBuilder = AnswerWebAppQueryBuilder;

impl AnswerWebAppQueryBuilder {
    pub fn build(&self) -> AnswerWebAppQuery {
        self.inner.clone()
    }

    pub fn web_app_query_id<T: AsRef<str>>(&mut self, web_app_query_id: T) -> &mut Self {
        self.inner.web_app_query_id = web_app_query_id.as_ref().to_string();
        self
    }

    pub fn result<T: AsRef<InputInlineQueryResult>>(&mut self, result: T) -> &mut Self {
        self.inner.result = result.as_ref().clone();
        self
    }
}

impl AsRef<AnswerWebAppQuery> for AnswerWebAppQuery {
    fn as_ref(&self) -> &AnswerWebAppQuery {
        self
    }
}

impl AsRef<AnswerWebAppQuery> for AnswerWebAppQueryBuilder {
    fn as_ref(&self) -> &AnswerWebAppQuery {
        &self.inner
    }
}
