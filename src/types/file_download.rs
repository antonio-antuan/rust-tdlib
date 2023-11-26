use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a file added to file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileDownload {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File identifier

    #[serde(default)]
    file_id: i32,
    /// The message with the file
    message: Message,
    /// Point in time (Unix timestamp) when the file was added to the download list

    #[serde(default)]
    add_date: i32,
    /// Point in time (Unix timestamp) when the file downloading was completed; 0 if the file downloading isn't completed

    #[serde(default)]
    complete_date: i32,
    /// True, if downloading of the file is paused

    #[serde(default)]
    is_paused: bool,
}

impl RObject for FileDownload {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FileDownload {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileDownloadBuilder {
        let mut inner = FileDownload::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileDownloadBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn message(&self) -> &Message {
        &self.message
    }

    pub fn add_date(&self) -> i32 {
        self.add_date
    }

    pub fn complete_date(&self) -> i32 {
        self.complete_date
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}

#[doc(hidden)]
pub struct FileDownloadBuilder {
    inner: FileDownload,
}

#[deprecated]
pub type RTDFileDownloadBuilder = FileDownloadBuilder;

impl FileDownloadBuilder {
    pub fn build(&self) -> FileDownload {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }

    pub fn add_date(&mut self, add_date: i32) -> &mut Self {
        self.inner.add_date = add_date;
        self
    }

    pub fn complete_date(&mut self, complete_date: i32) -> &mut Self {
        self.inner.complete_date = complete_date;
        self
    }

    pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
        self.inner.is_paused = is_paused;
        self
    }
}

impl AsRef<FileDownload> for FileDownload {
    fn as_ref(&self) -> &FileDownload {
        self
    }
}

impl AsRef<FileDownload> for FileDownloadBuilder {
    fn as_ref(&self) -> &FileDownload {
        &self.inner
    }
}
