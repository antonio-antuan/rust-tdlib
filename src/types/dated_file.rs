use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// File with the date it was uploaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatedFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The file
    file: File,
    /// Point in time (Unix timestamp) when the file was uploaded

    #[serde(default)]
    date: i32,
}

impl RObject for DatedFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DatedFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DatedFileBuilder {
        let mut inner = DatedFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DatedFileBuilder { inner }
    }

    pub fn file(&self) -> &File {
        &self.file
    }

    pub fn date(&self) -> i32 {
        self.date
    }
}

#[doc(hidden)]
pub struct DatedFileBuilder {
    inner: DatedFile,
}

#[deprecated]
pub type RTDDatedFileBuilder = DatedFileBuilder;

impl DatedFileBuilder {
    pub fn build(&self) -> DatedFile {
        self.inner.clone()
    }

    pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
        self.inner.file = file.as_ref().clone();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }
}

impl AsRef<DatedFile> for DatedFile {
    fn as_ref(&self) -> &DatedFile {
        self
    }
}

impl AsRef<DatedFile> for DatedFileBuilder {
    fn as_ref(&self) -> &DatedFile {
        &self.inner
    }
}
