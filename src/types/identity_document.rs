use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// An identity document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Document number; 1-24 characters

    #[serde(default)]
    number: String,
    /// Document expiry date; may be null if not applicable
    expiry_date: Option<Date>,
    /// Front side of the document
    front_side: DatedFile,
    /// Reverse side of the document; only for driver license and identity card; may be null
    reverse_side: Option<DatedFile>,
    /// Selfie with the document; may be null
    selfie: Option<DatedFile>,
    /// List of files containing a certified English translation of the document

    #[serde(default)]
    translation: Vec<DatedFile>,
}

impl RObject for IdentityDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl IdentityDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> IdentityDocumentBuilder {
        let mut inner = IdentityDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        IdentityDocumentBuilder { inner }
    }

    pub fn number(&self) -> &String {
        &self.number
    }

    pub fn expiry_date(&self) -> &Option<Date> {
        &self.expiry_date
    }

    pub fn front_side(&self) -> &DatedFile {
        &self.front_side
    }

    pub fn reverse_side(&self) -> &Option<DatedFile> {
        &self.reverse_side
    }

    pub fn selfie(&self) -> &Option<DatedFile> {
        &self.selfie
    }

    pub fn translation(&self) -> &Vec<DatedFile> {
        &self.translation
    }
}

#[doc(hidden)]
pub struct IdentityDocumentBuilder {
    inner: IdentityDocument,
}

#[deprecated]
pub type RTDIdentityDocumentBuilder = IdentityDocumentBuilder;

impl IdentityDocumentBuilder {
    pub fn build(&self) -> IdentityDocument {
        self.inner.clone()
    }

    pub fn number<T: AsRef<str>>(&mut self, number: T) -> &mut Self {
        self.inner.number = number.as_ref().to_string();
        self
    }

    pub fn expiry_date<T: AsRef<Date>>(&mut self, expiry_date: T) -> &mut Self {
        self.inner.expiry_date = Some(expiry_date.as_ref().clone());
        self
    }

    pub fn front_side<T: AsRef<DatedFile>>(&mut self, front_side: T) -> &mut Self {
        self.inner.front_side = front_side.as_ref().clone();
        self
    }

    pub fn reverse_side<T: AsRef<DatedFile>>(&mut self, reverse_side: T) -> &mut Self {
        self.inner.reverse_side = Some(reverse_side.as_ref().clone());
        self
    }

    pub fn selfie<T: AsRef<DatedFile>>(&mut self, selfie: T) -> &mut Self {
        self.inner.selfie = Some(selfie.as_ref().clone());
        self
    }

    pub fn translation(&mut self, translation: Vec<DatedFile>) -> &mut Self {
        self.inner.translation = translation;
        self
    }
}

impl AsRef<IdentityDocument> for IdentityDocument {
    fn as_ref(&self) -> &IdentityDocument {
        self
    }
}

impl AsRef<IdentityDocument> for IdentityDocumentBuilder {
    fn as_ref(&self) -> &IdentityDocument {
        &self.inner
    }
}
