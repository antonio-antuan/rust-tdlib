use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileExtension {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The MIME type of the file

    #[serde(default)]
    mime_type: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetFileExtension {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetFileExtension {}

impl GetFileExtension {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetFileExtensionBuilder {
        let mut inner = GetFileExtension::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getFileExtension".to_string();

        GetFileExtensionBuilder { inner }
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }
}

#[doc(hidden)]
pub struct GetFileExtensionBuilder {
    inner: GetFileExtension,
}

#[deprecated]
pub type RTDGetFileExtensionBuilder = GetFileExtensionBuilder;

impl GetFileExtensionBuilder {
    pub fn build(&self) -> GetFileExtension {
        self.inner.clone()
    }

    pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
        self.inner.mime_type = mime_type.as_ref().to_string();
        self
    }
}

impl AsRef<GetFileExtension> for GetFileExtension {
    fn as_ref(&self) -> &GetFileExtension {
        self
    }
}

impl AsRef<GetFileExtension> for GetFileExtensionBuilder {
    fn as_ref(&self) -> &GetFileExtension {
        &self.inner
    }
}
