use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The first name of the user; 1-64 characters
    first_name: String,
    /// The last name of the user; 0-64 characters
    last_name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RegisterUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RegisterUser {}

impl RegisterUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRegisterUserBuilder {
        let mut inner = RegisterUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "registerUser".to_string();

        RTDRegisterUserBuilder { inner }
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }
}

#[doc(hidden)]
pub struct RTDRegisterUserBuilder {
    inner: RegisterUser,
}

impl RTDRegisterUserBuilder {
    pub fn build(&self) -> RegisterUser {
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

impl AsRef<RegisterUser> for RegisterUser {
    fn as_ref(&self) -> &RegisterUser {
        self
    }
}

impl AsRef<RegisterUser> for RTDRegisterUserBuilder {
    fn as_ref(&self) -> &RegisterUser {
        &self.inner
    }
}
