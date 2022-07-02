use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CleanFileName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// File name or path to the file

    #[serde(default)]
    file_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CleanFileName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CleanFileName {}

impl CleanFileName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CleanFileNameBuilder {
        let mut inner = CleanFileName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cleanFileName".to_string();

        CleanFileNameBuilder { inner }
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }
}

#[doc(hidden)]
pub struct CleanFileNameBuilder {
    inner: CleanFileName,
}

#[deprecated]
pub type RTDCleanFileNameBuilder = CleanFileNameBuilder;

impl CleanFileNameBuilder {
    pub fn build(&self) -> CleanFileName {
        self.inner.clone()
    }

    pub fn file_name<T: AsRef<str>>(&mut self, file_name: T) -> &mut Self {
        self.inner.file_name = file_name.as_ref().to_string();
        self
    }
}

impl AsRef<CleanFileName> for CleanFileName {
    fn as_ref(&self) -> &CleanFileName {
        self
    }
}

impl AsRef<CleanFileName> for CleanFileNameBuilder {
    fn as_ref(&self) -> &CleanFileName {
        &self.inner
    }
}
