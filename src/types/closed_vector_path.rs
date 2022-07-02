use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a closed vector path. The path begins at the end point of the last command
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClosedVectorPath {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of vector path commands

    #[serde(default)]
    commands: Vec<VectorPathCommand>,
}

impl RObject for ClosedVectorPath {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ClosedVectorPath {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClosedVectorPathBuilder {
        let mut inner = ClosedVectorPath::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ClosedVectorPathBuilder { inner }
    }

    pub fn commands(&self) -> &Vec<VectorPathCommand> {
        &self.commands
    }
}

#[doc(hidden)]
pub struct ClosedVectorPathBuilder {
    inner: ClosedVectorPath,
}

#[deprecated]
pub type RTDClosedVectorPathBuilder = ClosedVectorPathBuilder;

impl ClosedVectorPathBuilder {
    pub fn build(&self) -> ClosedVectorPath {
        self.inner.clone()
    }

    pub fn commands(&mut self, commands: Vec<VectorPathCommand>) -> &mut Self {
        self.inner.commands = commands;
        self
    }
}

impl AsRef<ClosedVectorPath> for ClosedVectorPath {
    fn as_ref(&self) -> &ClosedVectorPath {
        self
    }
}

impl AsRef<ClosedVectorPath> for ClosedVectorPathBuilder {
    fn as_ref(&self) -> &ClosedVectorPath {
        &self.inner
    }
}
