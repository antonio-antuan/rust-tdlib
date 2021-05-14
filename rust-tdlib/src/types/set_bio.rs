use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the bio of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new value of the user bio; 0-70 characters without line feeds
    bio: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBio {}

impl SetBio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetBioBuilder {
        let mut inner = SetBio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBio".to_string();

        RTDSetBioBuilder { inner }
    }

    pub fn bio(&self) -> &String {
        &self.bio
    }
}

#[doc(hidden)]
pub struct RTDSetBioBuilder {
    inner: SetBio,
}

impl RTDSetBioBuilder {
    pub fn build(&self) -> SetBio {
        self.inner.clone()
    }

    pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
        self.inner.bio = bio.as_ref().to_string();
        self
    }
}

impl AsRef<SetBio> for SetBio {
    fn as_ref(&self) -> &SetBio {
        self
    }
}

impl AsRef<SetBio> for RTDSetBioBuilder {
    fn as_ref(&self) -> &SetBio {
        &self.inner
    }
}
