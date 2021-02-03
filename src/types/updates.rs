use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Updates {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of updates
    updates: Vec<Update>,
}

impl RObject for Updates {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "updates"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl Updates {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpdatesBuilder {
        let mut inner = Updates::default();
        inner.td_name = "updates".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDUpdatesBuilder { inner }
    }

    pub fn updates(&self) -> &Vec<Update> {
        &self.updates
    }
}

#[doc(hidden)]
pub struct RTDUpdatesBuilder {
    inner: Updates,
}

impl RTDUpdatesBuilder {
    pub fn build(&self) -> Updates {
        self.inner.clone()
    }

    pub fn updates(&mut self, updates: Vec<Update>) -> &mut Self {
        self.inner.updates = updates;
        self
    }
}

impl AsRef<Updates> for Updates {
    fn as_ref(&self) -> &Updates {
        self
    }
}

impl AsRef<Updates> for RTDUpdatesBuilder {
    fn as_ref(&self) -> &Updates {
        &self.inner
    }
}
