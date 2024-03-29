use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A personal document, containing some information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PersonalDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of files containing the pages of the document

    #[serde(default)]
    files: Vec<DatedFile>,
    /// List of files containing a certified English translation of the document

    #[serde(default)]
    translation: Vec<DatedFile>,
}

impl RObject for PersonalDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PersonalDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PersonalDocumentBuilder {
        let mut inner = PersonalDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PersonalDocumentBuilder { inner }
    }

    pub fn files(&self) -> &Vec<DatedFile> {
        &self.files
    }

    pub fn translation(&self) -> &Vec<DatedFile> {
        &self.translation
    }
}

#[doc(hidden)]
pub struct PersonalDocumentBuilder {
    inner: PersonalDocument,
}

#[deprecated]
pub type RTDPersonalDocumentBuilder = PersonalDocumentBuilder;

impl PersonalDocumentBuilder {
    pub fn build(&self) -> PersonalDocument {
        self.inner.clone()
    }

    pub fn files(&mut self, files: Vec<DatedFile>) -> &mut Self {
        self.inner.files = files;
        self
    }

    pub fn translation(&mut self, translation: Vec<DatedFile>) -> &mut Self {
        self.inner.translation = translation;
        self
    }
}

impl AsRef<PersonalDocument> for PersonalDocument {
    fn as_ref(&self) -> &PersonalDocument {
        self
    }
}

impl AsRef<PersonalDocument> for PersonalDocumentBuilder {
    fn as_ref(&self) -> &PersonalDocument {
        &self.inner
    }
}
