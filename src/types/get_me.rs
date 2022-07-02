use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMe {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMe {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMe {}

impl GetMe {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMeBuilder {
        let mut inner = GetMe::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMe".to_string();

        GetMeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetMeBuilder {
    inner: GetMe,
}

#[deprecated]
pub type RTDGetMeBuilder = GetMeBuilder;

impl GetMeBuilder {
    pub fn build(&self) -> GetMe {
        self.inner.clone()
    }
}

impl AsRef<GetMe> for GetMe {
    fn as_ref(&self) -> &GetMe {
        self
    }
}

impl AsRef<GetMe> for GetMeBuilder {
    fn as_ref(&self) -> &GetMe {
        &self.inner
    }
}
