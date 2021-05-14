use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib on a file generation progress
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetFileGenerationProgress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the generation process

    #[serde(deserialize_with = "super::_common::number_from_string")]
    generation_id: i64,
    /// Expected size of the generated file, in bytes; 0 if unknown
    expected_size: i32,
    /// The number of bytes already generated
    local_prefix_size: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetFileGenerationProgress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetFileGenerationProgress {}

impl SetFileGenerationProgress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetFileGenerationProgressBuilder {
        let mut inner = SetFileGenerationProgress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setFileGenerationProgress".to_string();

        RTDSetFileGenerationProgressBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }

    pub fn expected_size(&self) -> i32 {
        self.expected_size
    }

    pub fn local_prefix_size(&self) -> i32 {
        self.local_prefix_size
    }
}

#[doc(hidden)]
pub struct RTDSetFileGenerationProgressBuilder {
    inner: SetFileGenerationProgress,
}

impl RTDSetFileGenerationProgressBuilder {
    pub fn build(&self) -> SetFileGenerationProgress {
        self.inner.clone()
    }

    pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
        self.inner.generation_id = generation_id;
        self
    }

    pub fn expected_size(&mut self, expected_size: i32) -> &mut Self {
        self.inner.expected_size = expected_size;
        self
    }

    pub fn local_prefix_size(&mut self, local_prefix_size: i32) -> &mut Self {
        self.inner.local_prefix_size = local_prefix_size;
        self
    }
}

impl AsRef<SetFileGenerationProgress> for SetFileGenerationProgress {
    fn as_ref(&self) -> &SetFileGenerationProgress {
        self
    }
}

impl AsRef<SetFileGenerationProgress> for RTDSetFileGenerationProgressBuilder {
    fn as_ref(&self) -> &SetFileGenerationProgress {
        &self.inner
    }
}
