use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes background from the list of installed backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The background identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    background_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveBackground {}

impl RemoveBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveBackgroundBuilder {
        let mut inner = RemoveBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeBackground".to_string();

        RemoveBackgroundBuilder { inner }
    }

    pub fn background_id(&self) -> i64 {
        self.background_id
    }
}

#[doc(hidden)]
pub struct RemoveBackgroundBuilder {
    inner: RemoveBackground,
}

#[deprecated]
pub type RTDRemoveBackgroundBuilder = RemoveBackgroundBuilder;

impl RemoveBackgroundBuilder {
    pub fn build(&self) -> RemoveBackground {
        self.inner.clone()
    }

    pub fn background_id(&mut self, background_id: i64) -> &mut Self {
        self.inner.background_id = background_id;
        self
    }
}

impl AsRef<RemoveBackground> for RemoveBackground {
    fn as_ref(&self) -> &RemoveBackground {
        self
    }
}

impl AsRef<RemoveBackground> for RemoveBackgroundBuilder {
    fn as_ref(&self) -> &RemoveBackground {
        &self.inner
    }
}
