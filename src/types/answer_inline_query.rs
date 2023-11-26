use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the result of an inline query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the inline query

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    inline_query_id: i64,
    /// Pass true if results may be cached and returned only for the user that sent the query. By default, results may be returned to any user who sends the same query

    #[serde(default)]
    is_personal: bool,
    /// Button to be shown above inline query results; pass null if none
    button: InlineQueryResultsButton,
    /// The results of the query

    #[serde(default)]
    results: Vec<InputInlineQueryResult>,
    /// Allowed time to cache the results of the query, in seconds

    #[serde(default)]
    cache_time: i32,
    /// Offset for the next inline query; pass an empty string if there are no more results

    #[serde(default)]
    next_offset: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerInlineQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerInlineQuery {}

impl AnswerInlineQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnswerInlineQueryBuilder {
        let mut inner = AnswerInlineQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerInlineQuery".to_string();

        AnswerInlineQueryBuilder { inner }
    }

    pub fn inline_query_id(&self) -> i64 {
        self.inline_query_id
    }

    pub fn is_personal(&self) -> bool {
        self.is_personal
    }

    pub fn button(&self) -> &InlineQueryResultsButton {
        &self.button
    }

    pub fn results(&self) -> &Vec<InputInlineQueryResult> {
        &self.results
    }

    pub fn cache_time(&self) -> i32 {
        self.cache_time
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct AnswerInlineQueryBuilder {
    inner: AnswerInlineQuery,
}

#[deprecated]
pub type RTDAnswerInlineQueryBuilder = AnswerInlineQueryBuilder;

impl AnswerInlineQueryBuilder {
    pub fn build(&self) -> AnswerInlineQuery {
        self.inner.clone()
    }

    pub fn inline_query_id(&mut self, inline_query_id: i64) -> &mut Self {
        self.inner.inline_query_id = inline_query_id;
        self
    }

    pub fn is_personal(&mut self, is_personal: bool) -> &mut Self {
        self.inner.is_personal = is_personal;
        self
    }

    pub fn button<T: AsRef<InlineQueryResultsButton>>(&mut self, button: T) -> &mut Self {
        self.inner.button = button.as_ref().clone();
        self
    }

    pub fn results(&mut self, results: Vec<InputInlineQueryResult>) -> &mut Self {
        self.inner.results = results;
        self
    }

    pub fn cache_time(&mut self, cache_time: i32) -> &mut Self {
        self.inner.cache_time = cache_time;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<AnswerInlineQuery> for AnswerInlineQuery {
    fn as_ref(&self) -> &AnswerInlineQuery {
        self
    }
}

impl AsRef<AnswerInlineQuery> for AnswerInlineQueryBuilder {
    fn as_ref(&self) -> &AnswerInlineQuery {
        &self.inner
    }
}
