use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetFileMimeType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The name of the file or path to the file

    #[serde(default)]
    file_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetFileMimeType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetFileMimeType {}

impl GetFileMimeType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetFileMimeTypeBuilder {
        let mut inner = GetFileMimeType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getFileMimeType".to_string();

        GetFileMimeTypeBuilder { inner }
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }
}

#[doc(hidden)]
pub struct GetFileMimeTypeBuilder {
    inner: GetFileMimeType,
}

#[deprecated]
pub type RTDGetFileMimeTypeBuilder = GetFileMimeTypeBuilder;

impl GetFileMimeTypeBuilder {
    pub fn build(&self) -> GetFileMimeType {
        self.inner.clone()
    }

    pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
        self.inner.file_name = file_name.as_ref().to_string();
        self
    }
}

impl AsRef<GetFileMimeType> for GetFileMimeType {
    fn as_ref(&self) -> &GetFileMimeType {
        self
    }
}

impl AsRef<GetFileMimeType> for GetFileMimeTypeBuilder {
    fn as_ref(&self) -> &GetFileMimeType {
        &self.inner
    }
}
