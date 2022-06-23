use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a file from a message to the list of file downloads. Download progress and completion of the download will be notified through updateFile updates. If message database is used, the list of file downloads is persistent across application restarts. The downloading is independent from download using downloadFile, i.e. it continues if downloadFile is canceled or is used to download a part of the file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddFileToDownloads {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file to download
    file_id: i32,
    /// Chat identifier of the message with the file
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// Priority of the download (1-32). The higher the priority, the earlier the file will be downloaded. If the priorities of two files are equal, then the last one for which downloadFile/addFileToDownloads was called will be downloaded first
    priority: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddFileToDownloads {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddFileToDownloads {}

impl AddFileToDownloads {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddFileToDownloadsBuilder {
        let mut inner = AddFileToDownloads::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addFileToDownloads".to_string();

        RTDAddFileToDownloadsBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }
}

#[doc(hidden)]
pub struct RTDAddFileToDownloadsBuilder {
    inner: AddFileToDownloads,
}

impl RTDAddFileToDownloadsBuilder {
    pub fn build(&self) -> AddFileToDownloads {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn priority(&mut self, priority: i32) -> &mut Self {
        self.inner.priority = priority;
        self
    }
}

impl AsRef<AddFileToDownloads> for AddFileToDownloads {
    fn as_ref(&self) -> &AddFileToDownloads {
        self
    }
}

impl AsRef<AddFileToDownloads> for RTDAddFileToDownloadsBuilder {
    fn as_ref(&self) -> &AddFileToDownloads {
        &self.inner
    }
}
