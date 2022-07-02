use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default icon name for a filter. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFilterDefaultIconName {
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

impl RObject for GetChatFilterDefaultIconName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFilterDefaultIconName {}

impl GetChatFilterDefaultIconName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFilterDefaultIconNameBuilder {
        let mut inner = GetChatFilterDefaultIconName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFilterDefaultIconName".to_string();

        GetChatFilterDefaultIconNameBuilder { inner }
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }
}

#[doc(hidden)]
pub struct GetChatFilterDefaultIconNameBuilder {
    inner: GetChatFilterDefaultIconName,
}

#[deprecated]
pub type RTDGetChatFilterDefaultIconNameBuilder = GetChatFilterDefaultIconNameBuilder;

impl GetChatFilterDefaultIconNameBuilder {
    pub fn build(&self) -> GetChatFilterDefaultIconName {
        self.inner.clone()
    }

    pub fn filter<T: AsRef<ChatFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }
}

impl AsRef<GetChatFilterDefaultIconName> for GetChatFilterDefaultIconName {
    fn as_ref(&self) -> &GetChatFilterDefaultIconName {
        self
    }
}

impl AsRef<GetChatFilterDefaultIconName> for GetChatFilterDefaultIconNameBuilder {
    fn as_ref(&self) -> &GetChatFilterDefaultIconName {
        &self.inner
    }
}
