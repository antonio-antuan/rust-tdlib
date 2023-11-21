use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains 0-based match position
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The position of the match

    #[serde(default)]
    position: i32,
}

impl RObject for FoundPosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundPositionBuilder {
        let mut inner = FoundPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundPositionBuilder { inner }
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}

#[doc(hidden)]
pub struct FoundPositionBuilder {
    inner: FoundPosition,
}

#[deprecated]
pub type RTDFoundPositionBuilder = FoundPositionBuilder;

impl FoundPositionBuilder {
    pub fn build(&self) -> FoundPosition {
        self.inner.clone()
    }

    pub fn position(&mut self, position: i32) -> &mut Self {
        self.inner.position = position;
        self
    }
}

impl AsRef<FoundPosition> for FoundPosition {
    fn as_ref(&self) -> &FoundPosition {
        self
    }
}

impl AsRef<FoundPosition> for FoundPositionBuilder {
    fn as_ref(&self) -> &FoundPosition {
        &self.inner
    }
}
