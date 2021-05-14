use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a new temporary password for processing payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTemporaryPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Persistent user password
    password: String,
    /// Time during which the temporary password will be valid, in seconds; should be between 60 and 86400
    valid_for: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateTemporaryPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateTemporaryPassword {}

impl CreateTemporaryPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateTemporaryPasswordBuilder {
        let mut inner = CreateTemporaryPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createTemporaryPassword".to_string();

        RTDCreateTemporaryPasswordBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn valid_for(&self) -> i32 {
        self.valid_for
    }
}

#[doc(hidden)]
pub struct RTDCreateTemporaryPasswordBuilder {
    inner: CreateTemporaryPassword,
}

impl RTDCreateTemporaryPasswordBuilder {
    pub fn build(&self) -> CreateTemporaryPassword {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn valid_for(&mut self, valid_for: i32) -> &mut Self {
        self.inner.valid_for = valid_for;
        self
    }
}

impl AsRef<CreateTemporaryPassword> for CreateTemporaryPassword {
    fn as_ref(&self) -> &CreateTemporaryPassword {
        self
    }
}

impl AsRef<CreateTemporaryPassword> for RTDCreateTemporaryPasswordBuilder {
    fn as_ref(&self) -> &CreateTemporaryPassword {
        &self.inner
    }
}
