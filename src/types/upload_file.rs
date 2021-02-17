use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File to upload

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    file: InputFile,
    /// File type

    #[serde(skip_serializing_if = "FileType::_is_default")]
    file_type: FileType,
    /// Priority of the upload (1-32). The higher the priority, the earlier the file will be uploaded. If the priorities of two files are equal, then the first one for which uploadFile was called will be uploaded first
    priority: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UploadFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UploadFile {}

impl UploadFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUploadFileBuilder {
        let mut inner = UploadFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "uploadFile".to_string();

        RTDUploadFileBuilder { inner }
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
pub struct RTDUploadFileBuilder {
    inner: UploadFile,
}

impl RTDUploadFileBuilder {
    pub fn build(&self) -> UploadFile {
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

impl AsRef<UploadFile> for UploadFile {
    fn as_ref(&self) -> &UploadFile {
        self
    }
}

impl AsRef<UploadFile> for RTDUploadFileBuilder {
    fn as_ref(&self) -> &UploadFile {
        &self.inner
    }
}
