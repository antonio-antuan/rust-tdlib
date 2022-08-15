use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Finishes the file generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FinishFileGeneration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the generation process

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    generation_id: i64,
    /// If passed, the file generation has failed and must be terminated; pass null if the file generation succeeded
    error: Error,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for FinishFileGeneration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for FinishFileGeneration {}

impl FinishFileGeneration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FinishFileGenerationBuilder {
        let mut inner = FinishFileGeneration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "finishFileGeneration".to_string();

        FinishFileGenerationBuilder { inner }
    }

    pub fn generation_id(&self) -> i64 {
        self.generation_id
    }

    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[doc(hidden)]
pub struct FinishFileGenerationBuilder {
    inner: FinishFileGeneration,
}

#[deprecated]
pub type RTDFinishFileGenerationBuilder = FinishFileGenerationBuilder;

impl FinishFileGenerationBuilder {
    pub fn build(&self) -> FinishFileGeneration {
        self.inner.clone()
    }

    pub fn generation_id(&mut self, generation_id: i64) -> &mut Self {
        self.inner.generation_id = generation_id;
        self
    }

    pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
        self.inner.error = error.as_ref().clone();
        self
    }
}

impl AsRef<FinishFileGeneration> for FinishFileGeneration {
    fn as_ref(&self) -> &FinishFileGeneration {
        self
    }
}

impl AsRef<FinishFileGeneration> for FinishFileGenerationBuilder {
    fn as_ref(&self) -> &FinishFileGeneration {
        &self.inner
    }
}
