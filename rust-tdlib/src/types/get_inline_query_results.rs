use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInlineQueryResults {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the target bot
    bot_user_id: i32,
    /// Identifier of the chat where the query was sent
    chat_id: i64,
    /// Location of the user, only if needed
    user_location: Location,
    /// Text of the query
    query: String,
    /// Offset of the first entry to return
    offset: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInlineQueryResults {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInlineQueryResults {}

impl GetInlineQueryResults {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetInlineQueryResultsBuilder {
        let mut inner = GetInlineQueryResults::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInlineQueryResults".to_string();

        RTDGetInlineQueryResultsBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i32 {
        self.bot_user_id
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn user_location(&self) -> &Location {
        &self.user_location
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }
}

#[doc(hidden)]
pub struct RTDGetInlineQueryResultsBuilder {
    inner: GetInlineQueryResults,
}

impl RTDGetInlineQueryResultsBuilder {
    pub fn build(&self) -> GetInlineQueryResults {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn user_location<T: AsRef<Location>>(&mut self, user_location: T) -> &mut Self {
        self.inner.user_location = user_location.as_ref().clone();
        self
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }
}

impl AsRef<GetInlineQueryResults> for GetInlineQueryResults {
    fn as_ref(&self) -> &GetInlineQueryResults {
        self
    }
}

impl AsRef<GetInlineQueryResults> for RTDGetInlineQueryResultsBuilder {
    fn as_ref(&self) -> &GetInlineQueryResults {
        &self.inner
    }
}
