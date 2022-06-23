use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns suggested name for saving a file in a given directory
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSuggestedFileName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the file
    file_id: i32,
    /// Directory in which the file is supposed to be saved
    directory: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSuggestedFileName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSuggestedFileName {}

impl GetSuggestedFileName {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSuggestedFileNameBuilder {
        let mut inner = GetSuggestedFileName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSuggestedFileName".to_string();

        RTDGetSuggestedFileNameBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn directory(&self) -> &String {
        &self.directory
    }
}

#[doc(hidden)]
pub struct RTDGetSuggestedFileNameBuilder {
    inner: GetSuggestedFileName,
}

impl RTDGetSuggestedFileNameBuilder {
    pub fn build(&self) -> GetSuggestedFileName {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn directory<T: AsRef<str>>(&mut self, directory: T) -> &mut Self {
        self.inner.directory = directory.as_ref().to_string();
        self
    }
}

impl AsRef<GetSuggestedFileName> for GetSuggestedFileName {
    fn as_ref(&self) -> &GetSuggestedFileName {
        self
    }
}

impl AsRef<GetSuggestedFileName> for RTDGetSuggestedFileNameBuilder {
    fn as_ref(&self) -> &GetSuggestedFileName {
        &self.inner
    }
}
