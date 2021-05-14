use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches for a background by its name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The name of the background
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchBackground {}

impl SearchBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchBackgroundBuilder {
        let mut inner = SearchBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchBackground".to_string();

        RTDSearchBackgroundBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct RTDSearchBackgroundBuilder {
    inner: SearchBackground,
}

impl RTDSearchBackgroundBuilder {
    pub fn build(&self) -> SearchBackground {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<SearchBackground> for SearchBackground {
    fn as_ref(&self) -> &SearchBackground {
        self
    }
}

impl AsRef<SearchBackground> for RTDSearchBackgroundBuilder {
    fn as_ref(&self) -> &SearchBackground {
        &self.inner
    }
}
