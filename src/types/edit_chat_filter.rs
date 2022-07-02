use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Edits existing chat filter. Returns information about the edited chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditChatFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat filter identifier

    #[serde(default)]
    chat_filter_id: i32,
    /// The edited chat filter
    filter: ChatFilter,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EditChatFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EditChatFilter {}

impl EditChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EditChatFilterBuilder {
        let mut inner = EditChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "editChatFilter".to_string();

        EditChatFilterBuilder { inner }
    }

    pub fn chat_filter_id(&self) -> i32 {
        self.chat_filter_id
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct EditChatFilterBuilder {
    inner: EditChatFilter,
}

#[deprecated]
pub type RTDEditChatFilterBuilder = EditChatFilterBuilder;

impl EditChatFilterBuilder {
    pub fn build(&self) -> EditChatFilter {
        self.inner.clone()
    }

    pub fn chat_filter_id(&mut self, chat_filter_id: i32) -> &mut Self {
        self.inner.chat_filter_id = chat_filter_id;
        self
    }

    pub fn filter<T: AsRef<ChatFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }
}

impl AsRef<EditChatFilter> for EditChatFilter {
    fn as_ref(&self) -> &EditChatFilter {
        self
    }
}

impl AsRef<EditChatFilter> for EditChatFilterBuilder {
    fn as_ref(&self) -> &EditChatFilter {
        &self.inner
    }
}
