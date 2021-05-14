use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDatabaseEncryptionKey {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New encryption key
    new_encryption_key: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetDatabaseEncryptionKey {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetDatabaseEncryptionKey {}

impl SetDatabaseEncryptionKey {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetDatabaseEncryptionKeyBuilder {
        let mut inner = SetDatabaseEncryptionKey::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setDatabaseEncryptionKey".to_string();

        RTDSetDatabaseEncryptionKeyBuilder { inner }
    }

    pub fn new_encryption_key(&self) -> &String {
        &self.new_encryption_key
    }
}

#[doc(hidden)]
pub struct RTDSetDatabaseEncryptionKeyBuilder {
    inner: SetDatabaseEncryptionKey,
}

impl RTDSetDatabaseEncryptionKeyBuilder {
    pub fn build(&self) -> SetDatabaseEncryptionKey {
        self.inner.clone()
    }

    pub fn new_encryption_key<T: AsRef<str>>(&mut self, new_encryption_key: T) -> &mut Self {
        self.inner.new_encryption_key = new_encryption_key.as_ref().to_string();
        self
    }
}

impl AsRef<SetDatabaseEncryptionKey> for SetDatabaseEncryptionKey {
    fn as_ref(&self) -> &SetDatabaseEncryptionKey {
        self
    }
}

impl AsRef<SetDatabaseEncryptionKey> for RTDSetDatabaseEncryptionKeyBuilder {
    fn as_ref(&self) -> &SetDatabaseEncryptionKey {
        &self.inner
    }
}
