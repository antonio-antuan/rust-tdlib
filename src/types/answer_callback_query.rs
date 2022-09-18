use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the result of a callback query; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnswerCallbackQuery {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the callback query

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    callback_query_id: i64,
    /// Text of the answer

    #[serde(default)]
    text: String,
    /// If true, an alert must be shown to the user instead of a toast notification

    #[serde(default)]
    show_alert: bool,
    /// URL to be opened

    #[serde(default)]
    url: String,
    /// Time during which the result of the query can be cached, in seconds

    #[serde(default)]
    cache_time: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AnswerCallbackQuery {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AnswerCallbackQuery {}

impl AnswerCallbackQuery {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnswerCallbackQueryBuilder {
        let mut inner = AnswerCallbackQuery::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "answerCallbackQuery".to_string();

        AnswerCallbackQueryBuilder { inner }
    }

    pub fn callback_query_id(&self) -> i64 {
        self.callback_query_id
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn show_alert(&self) -> bool {
        self.show_alert
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn cache_time(&self) -> i32 {
        self.cache_time
    }
}

#[doc(hidden)]
pub struct AnswerCallbackQueryBuilder {
    inner: AnswerCallbackQuery,
}

#[deprecated]
pub type RTDAnswerCallbackQueryBuilder = AnswerCallbackQueryBuilder;

impl AnswerCallbackQueryBuilder {
    pub fn build(&self) -> AnswerCallbackQuery {
        self.inner.clone()
    }

    pub fn callback_query_id(&mut self, callback_query_id: i64) -> &mut Self {
        self.inner.callback_query_id = callback_query_id;
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn show_alert(&mut self, show_alert: bool) -> &mut Self {
        self.inner.show_alert = show_alert;
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn cache_time(&mut self, cache_time: i32) -> &mut Self {
        self.inner.cache_time = cache_time;
        self
    }
}

impl AsRef<AnswerCallbackQuery> for AnswerCallbackQuery {
    fn as_ref(&self) -> &AnswerCallbackQuery {
        self
    }
}

impl AsRef<AnswerCallbackQuery> for AnswerCallbackQueryBuilder {
    fn as_ref(&self) -> &AnswerCallbackQuery {
        &self.inner
    }
}
