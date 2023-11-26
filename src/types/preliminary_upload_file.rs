use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Preliminary uploads a file to the cloud before sending it in a message, which can be useful for uploading of being recorded voice and video notes. Updates updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it is sent in a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PreliminaryUploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File to upload

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    file: InputFile,
    /// File type; pass null if unknown

    #[serde(skip_serializing_if = "FileType::_is_default")]
    file_type: FileType,
    /// Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which preliminaryUploadFile was called will be uploaded first

    #[serde(default)]
    priority: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for PreliminaryUploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for PreliminaryUploadFile {}

impl PreliminaryUploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PreliminaryUploadFileBuilder {
        let mut inner = PreliminaryUploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "preliminaryUploadFile".to_string();

        PreliminaryUploadFileBuilder { inner }
    }

    pub fn file(&self) -> &InputFile {
        &self.file
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }
}

#[doc(hidden)]
pub struct PreliminaryUploadFileBuilder {
    inner: PreliminaryUploadFile,
}

#[deprecated]
pub type RTDPreliminaryUploadFileBuilder = PreliminaryUploadFileBuilder;

impl PreliminaryUploadFileBuilder {
    pub fn build(&self) -> PreliminaryUploadFile {
        self.inner.clone()
    }

    pub fn file<T: AsRef<InputFile>>(&mut self, file: T) -> &mut Self {
        self.inner.file = file.as_ref().clone();
        self
    }

    pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
        self.inner.file_type = file_type.as_ref().clone();
        self
    }

    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.inner.priority = priority;
        self
    }
}

impl AsRef<PreliminaryUploadFile> for PreliminaryUploadFile {
    fn as_ref(&self) -> &PreliminaryUploadFile {
        self
    }
}

impl AsRef<PreliminaryUploadFile> for PreliminaryUploadFileBuilder {
    fn as_ref(&self) -> &PreliminaryUploadFile {
        &self.inner
    }
}
