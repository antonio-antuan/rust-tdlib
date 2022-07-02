use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a file with messages exported from another app
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageFileType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Beginning of the message file; up to 100 first lines

    #[serde(default)]
    message_file_head: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageFileType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDMessageFileType for GetMessageFileType {}

impl RFunction for GetMessageFileType {}

impl GetMessageFileType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageFileTypeBuilder {
        let mut inner = GetMessageFileType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageFileType".to_string();

        GetMessageFileTypeBuilder { inner }
    }

    pub fn message_file_head(&self) -> &String {
        &self.message_file_head
    }
}

#[doc(hidden)]
pub struct GetMessageFileTypeBuilder {
    inner: GetMessageFileType,
}

#[deprecated]
pub type RTDGetMessageFileTypeBuilder = GetMessageFileTypeBuilder;

impl GetMessageFileTypeBuilder {
    pub fn build(&self) -> GetMessageFileType {
        self.inner.clone()
    }

    pub fn message_file_head<T: AsRef<str>>(&mut self, message_file_head: T) -> &mut Self {
        self.inner.message_file_head = message_file_head.as_ref().to_string();
        self
    }
}

impl AsRef<GetMessageFileType> for GetMessageFileType {
    fn as_ref(&self) -> &GetMessageFileType {
        self
    }
}

impl AsRef<GetMessageFileType> for GetMessageFileTypeBuilder {
    fn as_ref(&self) -> &GetMessageFileType {
        &self.inner
    }
}
