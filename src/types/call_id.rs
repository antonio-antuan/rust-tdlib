use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the call identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier

    #[serde(default)]
    id: i32,
}

impl RObject for CallId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl CallId {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallIdBuilder {
        let mut inner = CallId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallIdBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[doc(hidden)]
pub struct CallIdBuilder {
    inner: CallId,
}

#[deprecated]
pub type RTDCallIdBuilder = CallIdBuilder;

impl CallIdBuilder {
    pub fn build(&self) -> CallId {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }
}

impl AsRef<CallId> for CallId {
    fn as_ref(&self) -> &CallId {
        self
    }
}

impl AsRef<CallId> for CallIdBuilder {
    fn as_ref(&self) -> &CallId {
        &self.inner
    }
}
