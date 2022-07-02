use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns detailed statistics about a message. Can be used only if message.can_get_statistics == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Message identifier

    #[serde(default)]
    message_id: i64,
    /// Pass true if a dark theme is used by the application

    #[serde(default)]
    is_dark: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageStatistics {}

impl GetMessageStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageStatisticsBuilder {
        let mut inner = GetMessageStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageStatistics".to_string();

        GetMessageStatisticsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }
}

#[doc(hidden)]
pub struct GetMessageStatisticsBuilder {
    inner: GetMessageStatistics,
}

#[deprecated]
pub type RTDGetMessageStatisticsBuilder = GetMessageStatisticsBuilder;

impl GetMessageStatisticsBuilder {
    pub fn build(&self) -> GetMessageStatistics {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
        self.inner.is_dark = is_dark;
        self
    }
}

impl AsRef<GetMessageStatistics> for GetMessageStatistics {
    fn as_ref(&self) -> &GetMessageStatistics {
        self
    }
}

impl AsRef<GetMessageStatistics> for GetMessageStatisticsBuilder {
    fn as_ref(&self) -> &GetMessageStatistics {
        &self.inner
    }
}
