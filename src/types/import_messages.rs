use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Imports messages exported from another app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info administrator right

    #[serde(default)]
    chat_id: i64,
    /// File with messages to import. Only inputFileLocal and inputFileGenerated are supported. The file must not be previously uploaded

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    message_file: InputFile,
    /// Files used in the imported messages. Only inputFileLocal and inputFileGenerated are supported. The files must not be previously uploaded

    #[serde(default)]
    attached_files: Vec<InputFile>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ImportMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ImportMessages {}

impl ImportMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ImportMessagesBuilder {
        let mut inner = ImportMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "importMessages".to_string();

        ImportMessagesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_file(&self) -> &InputFile {
        &self.message_file
    }

    pub fn attached_files(&self) -> &Vec<InputFile> {
        &self.attached_files
    }
}

#[doc(hidden)]
pub struct ImportMessagesBuilder {
    inner: ImportMessages,
}

#[deprecated]
pub type RTDImportMessagesBuilder = ImportMessagesBuilder;

impl ImportMessagesBuilder {
    pub fn build(&self) -> ImportMessages {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_file<T: AsRef<InputFile>>(&mut self, message_file: T) -> &mut Self {
        self.inner.message_file = message_file.as_ref().clone();
        self
    }

    pub fn attached_files(&mut self, attached_files: Vec<InputFile>) -> &mut Self {
        self.inner.attached_files = attached_files;
        self
    }
}

impl AsRef<ImportMessages> for ImportMessages {
    fn as_ref(&self) -> &ImportMessages {
        self
    }
}

impl AsRef<ImportMessages> for ImportMessagesBuilder {
    fn as_ref(&self) -> &ImportMessages {
        &self.inner
    }
}
