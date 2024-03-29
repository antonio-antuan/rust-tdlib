use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about saved card credentials
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedCredentials {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the saved credentials

    #[serde(default)]
    id: String,
    /// Title of the saved credentials

    #[serde(default)]
    title: String,
}

impl RObject for SavedCredentials {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl SavedCredentials {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SavedCredentialsBuilder {
        let mut inner = SavedCredentials::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SavedCredentialsBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct SavedCredentialsBuilder {
    inner: SavedCredentials,
}

#[deprecated]
pub type RTDSavedCredentialsBuilder = SavedCredentialsBuilder;

impl SavedCredentialsBuilder {
    pub fn build(&self) -> SavedCredentials {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<SavedCredentials> for SavedCredentials {
    fn as_ref(&self) -> &SavedCredentials {
        self
    }
}

impl AsRef<SavedCredentials> for SavedCredentialsBuilder {
    fn as_ref(&self) -> &SavedCredentials {
        &self.inner
    }
}
