use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Clears draft messages in all chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearAllDraftMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, local draft messages in secret chats will not be cleared

    #[serde(default)]
    exclude_secret_chats: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearAllDraftMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearAllDraftMessages {}

impl ClearAllDraftMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClearAllDraftMessagesBuilder {
        let mut inner = ClearAllDraftMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearAllDraftMessages".to_string();

        ClearAllDraftMessagesBuilder { inner }
    }

    pub fn exclude_secret_chats(&self) -> bool {
        self.exclude_secret_chats
    }
}

#[doc(hidden)]
pub struct ClearAllDraftMessagesBuilder {
    inner: ClearAllDraftMessages,
}

#[deprecated]
pub type RTDClearAllDraftMessagesBuilder = ClearAllDraftMessagesBuilder;

impl ClearAllDraftMessagesBuilder {
    pub fn build(&self) -> ClearAllDraftMessages {
        self.inner.clone()
    }

    pub fn exclude_secret_chats(&mut self, exclude_secret_chats: bool) -> &mut Self {
        self.inner.exclude_secret_chats = exclude_secret_chats;
        self
    }
}

impl AsRef<ClearAllDraftMessages> for ClearAllDraftMessages {
    fn as_ref(&self) -> &ClearAllDraftMessages {
        self
    }
}

impl AsRef<ClearAllDraftMessages> for ClearAllDraftMessagesBuilder {
    fn as_ref(&self) -> &ClearAllDraftMessages {
        &self.inner
    }
}
