use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns storage usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStorageStatistics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The maximum number of chats with the largest storage usage for which separate statistics should be returned. All other chats will be grouped in entries with chat_id == 0. If the chat info database is not used, the chat_limit is ignored and is always set to 0
    chat_limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStorageStatistics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStorageStatistics {}

impl GetStorageStatistics {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetStorageStatisticsBuilder {
        let mut inner = GetStorageStatistics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStorageStatistics".to_string();

        RTDGetStorageStatisticsBuilder { inner }
    }

    pub fn chat_limit(&self) -> i32 {
        self.chat_limit
    }
}

#[doc(hidden)]
pub struct RTDGetStorageStatisticsBuilder {
    inner: GetStorageStatistics,
}

impl RTDGetStorageStatisticsBuilder {
    pub fn build(&self) -> GetStorageStatistics {
        self.inner.clone()
    }

    pub fn chat_limit(&mut self, chat_limit: i32) -> &mut Self {
        self.inner.chat_limit = chat_limit;
        self
    }
}

impl AsRef<GetStorageStatistics> for GetStorageStatistics {
    fn as_ref(&self) -> &GetStorageStatistics {
        self
    }
}

impl AsRef<GetStorageStatistics> for RTDGetStorageStatisticsBuilder {
    fn as_ref(&self) -> &GetStorageStatistics {
        &self.inner
    }
}
