use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an HTTP URL with the chat statistics. Currently this method of getting the statistics are disabled and can be deleted in the future
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatStatisticsUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Parameters from "tg://statsrefresh?params=******" link
    parameters: String,
    /// Pass true if a URL with the dark theme must be returned
    is_dark: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatStatisticsUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatStatisticsUrl {}

impl GetChatStatisticsUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatStatisticsUrlBuilder {
        let mut inner = GetChatStatisticsUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatStatisticsUrl".to_string();

        RTDGetChatStatisticsUrlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn parameters(&self) -> &String {
        &self.parameters
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }
}

#[doc(hidden)]
pub struct RTDGetChatStatisticsUrlBuilder {
    inner: GetChatStatisticsUrl,
}

impl RTDGetChatStatisticsUrlBuilder {
    pub fn build(&self) -> GetChatStatisticsUrl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn parameters<T: AsRef<str>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().to_string();
        self
    }

    pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
        self.inner.is_dark = is_dark;
        self
    }
}

impl AsRef<GetChatStatisticsUrl> for GetChatStatisticsUrl {
    fn as_ref(&self) -> &GetChatStatisticsUrl {
        self
    }
}

impl AsRef<GetChatStatisticsUrl> for RTDGetChatStatisticsUrlBuilder {
    fn as_ref(&self) -> &GetChatStatisticsUrl {
        &self.inner
    }
}
