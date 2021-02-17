use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateChatFilterBuilder {
        let mut inner = CreateChatFilter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createChatFilter".to_string();

        RTDCreateChatFilterBuilder { inner }
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct RTDCreateChatFilterBuilder {
    inner: CreateChatFilter,
}

impl RTDCreateChatFilterBuilder {
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

impl AsRef<CreateChatFilter> for RTDCreateChatFilterBuilder {
    fn as_ref(&self) -> &CreateChatFilter {
        &self.inner
    }
}
