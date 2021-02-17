use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a user that can be contacted to get support
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupportUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSupportUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSupportUser {}

impl GetSupportUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSupportUserBuilder {
        let mut inner = GetSupportUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupportUser".to_string();

        RTDGetSupportUserBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetSupportUserBuilder {
    inner: GetSupportUser,
}

impl RTDGetSupportUserBuilder {
    pub fn build(&self) -> GetSupportUser {
        self.inner.clone()
    }
}

impl AsRef<GetSupportUser> for GetSupportUser {
    fn as_ref(&self) -> &GetSupportUser {
        self
    }
}

impl AsRef<GetSupportUser> for RTDGetSupportUserBuilder {
    fn as_ref(&self) -> &GetSupportUser {
        &self.inner
    }
}
