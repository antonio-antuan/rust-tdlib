use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains 0-based positions of matched objects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundPositions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of matched objects

    #[serde(default)]
    total_count: i32,
    /// The positions of the matched objects

    #[serde(default)]
    positions: Vec<i32>,
}

impl RObject for FoundPositions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundPositions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundPositionsBuilder {
        let mut inner = FoundPositions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundPositionsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn positions(&self) -> &Vec<i32> {
        &self.positions
    }
}

#[doc(hidden)]
pub struct FoundPositionsBuilder {
    inner: FoundPositions,
}

#[deprecated]
pub type RTDFoundPositionsBuilder = FoundPositionsBuilder;

impl FoundPositionsBuilder {
    pub fn build(&self) -> FoundPositions {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn positions(&mut self, positions: Vec<i32>) -> &mut Self {
        self.inner.positions = positions;
        self
    }
}

impl AsRef<FoundPositions> for FoundPositions {
    fn as_ref(&self) -> &FoundPositions {
        self
    }
}

impl AsRef<FoundPositions> for FoundPositionsBuilder {
    fn as_ref(&self) -> &FoundPositions {
        &self.inner
    }
}
