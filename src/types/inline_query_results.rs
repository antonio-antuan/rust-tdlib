use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResults {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the inline query

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    inline_query_id: i64,
    /// Button to be shown above inline query results; may be null
    button: Option<InlineQueryResultsButton>,
    /// Results of the query

    #[serde(default)]
    results: Vec<InlineQueryResult>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for InlineQueryResults {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InlineQueryResults {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineQueryResultsBuilder {
        let mut inner = InlineQueryResults::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineQueryResultsBuilder { inner }
    }

    pub fn inline_query_id(&self) -> i64 {
        self.inline_query_id
    }

    pub fn button(&self) -> &Option<InlineQueryResultsButton> {
        &self.button
    }

    pub fn results(&self) -> &Vec<InlineQueryResult> {
        &self.results
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct InlineQueryResultsBuilder {
    inner: InlineQueryResults,
}

#[deprecated]
pub type RTDInlineQueryResultsBuilder = InlineQueryResultsBuilder;

impl InlineQueryResultsBuilder {
    pub fn build(&self) -> InlineQueryResults {
        self.inner.clone()
    }

    pub fn inline_query_id(&mut self, inline_query_id: i64) -> &mut Self {
        self.inner.inline_query_id = inline_query_id;
        self
    }

    pub fn button<T: AsRef<InlineQueryResultsButton>>(&mut self, button: T) -> &mut Self {
        self.inner.button = Some(button.as_ref().clone());
        self
    }

    pub fn results(&mut self, results: Vec<InlineQueryResult>) -> &mut Self {
        self.inner.results = results;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<InlineQueryResults> for InlineQueryResults {
    fn as_ref(&self) -> &InlineQueryResults {
        self
    }
}

impl AsRef<InlineQueryResults> for InlineQueryResultsBuilder {
    fn as_ref(&self) -> &InlineQueryResults {
        &self.inner
    }
}
