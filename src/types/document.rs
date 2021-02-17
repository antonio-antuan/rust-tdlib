use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a document of any type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Document {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Original name of the file; as defined by the sender
    file_name: String,
    /// MIME type of the file; as defined by the sender
    mime_type: String,
    /// Document minithumbnail; may be null
    minithumbnail: Option<Minithumbnail>,
    /// Document thumbnail in JPEG or PNG format (PNG will be used only for background patterns); as defined by the sender; may be null
    thumbnail: Option<Thumbnail>,
    /// File containing the document
    document: File,
}

impl RObject for Document {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Document {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDocumentBuilder {
        let mut inner = Document::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDDocumentBuilder { inner }
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
        &self.minithumbnail
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn document(&self) -> &File {
        &self.document
    }
}

#[doc(hidden)]
pub struct RTDDocumentBuilder {
    inner: Document,
}

impl RTDDocumentBuilder {
    pub fn build(&self) -> Document {
        self.inner.clone()
    }

    pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
        self.inner.file_name = file_name.as_ref().to_string();
        self
    }

    pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
        self.inner.mime_type = mime_type.as_ref().to_string();
        self
    }

    pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
        self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn document<T: AsRef<File>>(&mut self, document: T) -> &mut Self {
        self.inner.document = document.as_ref().clone();
        self
    }
}

impl AsRef<Document> for Document {
    fn as_ref(&self) -> &Document {
        self
    }
}

impl AsRef<Document> for RTDDocumentBuilder {
    fn as_ref(&self) -> &Document {
        &self.inner
    }
}
