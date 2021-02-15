use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallIdBuilder {
        let mut inner = CallId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallIdBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[doc(hidden)]
pub struct RTDCallIdBuilder {
    inner: CallId,
}

impl RTDCallIdBuilder {
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

impl AsRef<CallId> for RTDCallIdBuilder {
    fn as_ref(&self) -> &CallId {
        &self.inner
    }
}
