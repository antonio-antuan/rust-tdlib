use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Creates new chat filter. Returns information about the created chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateChatFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat filter
    filter: ChatFilter,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateChatFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateChatFilter {}

impl CreateChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateChatFilterBuilder {
        let mut inner = CreateChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createChatFilter".to_string();

        CreateChatFilterBuilder { inner }
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct CreateChatFilterBuilder {
    inner: CreateChatFilter,
}

#[deprecated]
pub type RTDCreateChatFilterBuilder = CreateChatFilterBuilder;

impl CreateChatFilterBuilder {
    pub fn build(&self) -> CreateChatFilter {
        self.inner.clone()
    }

    pub fn filter<T: AsRef<ChatFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }
}

impl AsRef<CreateChatFilter> for CreateChatFilter {
    fn as_ref(&self) -> &CreateChatFilter {
        self
    }
}

impl AsRef<CreateChatFilter> for CreateChatFilterBuilder {
    fn as_ref(&self) -> &CreateChatFilter {
        &self.inner
    }
}
