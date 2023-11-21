use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches for a given quote in a text. Returns found quote start position in UTF-16 code units. Returns a 404 error if the quote is not found. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchQuote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text in which to search for the quote
    text: FormattedText,
    /// Quote to search for
    quote: FormattedText,
    /// Approximate quote position in UTF-16 code units

    #[serde(default)]
    quote_position: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchQuote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchQuote {}

impl SearchQuote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchQuoteBuilder {
        let mut inner = SearchQuote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchQuote".to_string();

        SearchQuoteBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn quote(&self) -> &FormattedText {
        &self.quote
    }

    pub fn quote_position(&self) -> i32 {
        self.quote_position
    }
}

#[doc(hidden)]
pub struct SearchQuoteBuilder {
    inner: SearchQuote,
}

#[deprecated]
pub type RTDSearchQuoteBuilder = SearchQuoteBuilder;

impl SearchQuoteBuilder {
    pub fn build(&self) -> SearchQuote {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn quote<T: AsRef<FormattedText>>(&mut self, quote: T) -> &mut Self {
        self.inner.quote = quote.as_ref().clone();
        self
    }

    pub fn quote_position(&mut self, quote_position: i32) -> &mut Self {
        self.inner.quote_position = quote_position;
        self
    }
}

impl AsRef<SearchQuote> for SearchQuote {
    fn as_ref(&self) -> &SearchQuote {
        self
    }
}

impl AsRef<SearchQuote> for SearchQuoteBuilder {
    fn as_ref(&self) -> &SearchQuote {
        &self.inner
    }
}
