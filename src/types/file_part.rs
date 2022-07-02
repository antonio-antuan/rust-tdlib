use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a part of a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilePart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File bytes

    #[serde(default)]
    data: String,
}

impl RObject for FilePart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FilePart {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FilePartBuilder {
        let mut inner = FilePart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FilePartBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct FilePartBuilder {
    inner: FilePart,
}

#[deprecated]
pub type RTDFilePartBuilder = FilePartBuilder;

impl FilePartBuilder {
    pub fn build(&self) -> FilePart {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<FilePart> for FilePart {
    fn as_ref(&self) -> &FilePart {
        self
    }
}

impl AsRef<FilePart> for FilePartBuilder {
    fn as_ref(&self) -> &FilePart {
        &self.inner
    }
}
