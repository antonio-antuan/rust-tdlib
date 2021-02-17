use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Writes a part of a generated file. This method is intended to be used only if the application has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WriteGeneratedFilePart {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the generation process

    #[serde(deserialize_with = "super::_common::number_from_string")]
    generation_id: i64,
    /// The offset from which to write the data to the file
    offset: i32,
    /// The data to write
    data: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for WriteGeneratedFilePart {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for WriteGeneratedFilePart {}

impl WriteGeneratedFilePart {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDWriteGeneratedFilePartBuilder {
        let mut inner = WriteGeneratedFilePart::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "writeGeneratedFilePart".to_string();

        RTDWriteGeneratedFilePartBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDWriteGeneratedFilePartBuilder {
    inner: WriteGeneratedFilePart,
}

impl RTDWriteGeneratedFilePartBuilder {
    pub fn build(&self) -> WriteGeneratedFilePart {
        self.inner.clone()
    }

    pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
        self.inner.generation_id = generation_id;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<WriteGeneratedFilePart> for WriteGeneratedFilePart {
    fn as_ref(&self) -> &WriteGeneratedFilePart {
        self
    }
}

impl AsRef<WriteGeneratedFilePart> for RTDWriteGeneratedFilePartBuilder {
    fn as_ref(&self) -> &WriteGeneratedFilePart {
        &self.inner
    }
}
