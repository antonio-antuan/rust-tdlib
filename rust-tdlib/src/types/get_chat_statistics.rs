use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns detailed statistics about a chat. Currently this method can be used only for supergroups and channels. Can be used only if SupergroupFullInfo.can_get_statistics == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Pass true if a dark theme is used by the application
    is_dark: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatStatistics for GetChatStatistics {}

impl RFunction for GetChatStatistics {}

impl GetChatStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetChatStatisticsBuilder {
        let mut inner = GetChatStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatStatistics".to_string();

        RTDGetChatStatisticsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }
}

#[doc(hidden)]
pub struct RTDGetChatStatisticsBuilder {
    inner: GetChatStatistics,
}

impl RTDGetChatStatisticsBuilder {
    pub fn build(&self) -> GetChatStatistics {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
        self.inner.is_dark = is_dark;
        self
    }
}

impl AsRef<GetChatStatistics> for GetChatStatistics {
    fn as_ref(&self) -> &GetChatStatistics {
        self
    }
}

impl AsRef<GetChatStatistics> for RTDGetChatStatisticsBuilder {
    fn as_ref(&self) -> &GetChatStatistics {
        &self.inner
    }
}
