use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckDatabaseEncryptionKey {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Encryption key to check or set up

    #[serde(default)]
    encryption_key: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckDatabaseEncryptionKey {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CheckDatabaseEncryptionKey {}

impl CheckDatabaseEncryptionKey {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckDatabaseEncryptionKeyBuilder {
        let mut inner = CheckDatabaseEncryptionKey::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkDatabaseEncryptionKey".to_string();

        CheckDatabaseEncryptionKeyBuilder { inner }
    }

    pub fn encryption_key(&self) -> &String {
        &self.encryption_key
    }
}

#[doc(hidden)]
pub struct CheckDatabaseEncryptionKeyBuilder {
    inner: CheckDatabaseEncryptionKey,
}

#[deprecated]
pub type RTDCheckDatabaseEncryptionKeyBuilder = CheckDatabaseEncryptionKeyBuilder;

impl CheckDatabaseEncryptionKeyBuilder {
    pub fn build(&self) -> CheckDatabaseEncryptionKey {
        self.inner.clone()
    }

    pub fn encryption_key<T: AsRef<str>>(&mut self, encryption_key: T) -> &mut Self {
        self.inner.encryption_key = encryption_key.as_ref().to_string();
        self
    }
}

impl AsRef<CheckDatabaseEncryptionKey> for CheckDatabaseEncryptionKey {
    fn as_ref(&self) -> &CheckDatabaseEncryptionKey {
        self
    }
}

impl AsRef<CheckDatabaseEncryptionKey> for CheckDatabaseEncryptionKeyBuilder {
    fn as_ref(&self) -> &CheckDatabaseEncryptionKey {
        &self.inner
    }
}
