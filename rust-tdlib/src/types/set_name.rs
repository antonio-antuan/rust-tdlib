use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the first and last name of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new value of the first name for the user; 1-64 characters
    first_name: String,
    /// The new value of the optional last name for the user; 0-64 characters
    last_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetName {}

impl SetName {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetNameBuilder {
        let mut inner = SetName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setName".to_string();

        RTDSetNameBuilder { inner }
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }
}

#[doc(hidden)]
pub struct RTDSetNameBuilder {
    inner: SetName,
}

impl RTDSetNameBuilder {
    pub fn build(&self) -> SetName {
        self.inner.clone()
    }

    pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
        self.inner.first_name = first_name.as_ref().to_string();
        self
    }

    pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
        self.inner.last_name = last_name.as_ref().to_string();
        self
    }
}

impl AsRef<SetName> for SetName {
    fn as_ref(&self) -> &SetName {
        self
    }
}

impl AsRef<SetName> for RTDSetNameBuilder {
    fn as_ref(&self) -> &SetName {
        &self.inner
    }
}
