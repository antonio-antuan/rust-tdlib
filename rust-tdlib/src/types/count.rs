use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a counter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Count {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Count
    count: i32,
}

impl RObject for Count {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Count {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCountBuilder {
        let mut inner = Count::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCountBuilder { inner }
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[doc(hidden)]
pub struct RTDCountBuilder {
    inner: Count,
}

impl RTDCountBuilder {
    pub fn build(&self) -> Count {
        self.inner.clone()
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }
}

impl AsRef<Count> for Count {
    fn as_ref(&self) -> &Count {
        self
    }
}

impl AsRef<Count> for RTDCountBuilder {
    fn as_ref(&self) -> &Count {
        &self.inner
    }
}
