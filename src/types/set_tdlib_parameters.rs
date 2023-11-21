use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetTdlibParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to use Telegram test environment instead of the production environment

    #[serde(default)]
    use_test_dc: bool,
    /// The path to the directory for the persistent database; if empty, the current working directory will be used

    #[serde(default)]
    database_directory: String,
    /// The path to the directory for storing files; if empty, database_directory will be used

    #[serde(default)]
    files_directory: String,
    /// Encryption key for the database. If the encryption key is invalid, then an error with code 401 will be returned

    #[serde(default)]
    database_encryption_key: String,
    /// Pass true to keep information about downloaded and uploaded files between application restarts

    #[serde(default)]
    use_file_database: bool,
    /// Pass true to keep cache of users, basic groups, supergroups, channels and secret chats between restarts. Implies use_file_database

    #[serde(default)]
    use_chat_info_database: bool,
    /// Pass true to keep cache of chats and messages between restarts. Implies use_chat_info_database

    #[serde(default)]
    use_message_database: bool,
    /// Pass true to enable support for secret chats

    #[serde(default)]
    use_secret_chats: bool,
    /// Application identifier for Telegram API access, which can be obtained at https://my.telegram.org

    #[serde(default)]
    api_id: i32,
    /// Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org

    #[serde(default)]
    api_hash: String,
    /// IETF language tag of the user's operating system language; must be non-empty

    #[serde(default)]
    system_language_code: String,
    /// Model of the device the application is being run on; must be non-empty

    #[serde(default)]
    device_model: String,
    /// Version of the operating system the application is being run on. If empty, the version is automatically detected by TDLib

    #[serde(default)]
    system_version: String,
    /// Application version; must be non-empty

    #[serde(default)]
    application_version: String,
    /// Pass true to automatically delete old files in background

    #[serde(default)]
    enable_storage_optimizer: bool,
    /// Pass true to ignore original file names for downloaded files. Otherwise, downloaded files are saved under names as close as possible to the original name

    #[serde(default)]
    ignore_file_names: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetTdlibParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetTdlibParameters {}

impl SetTdlibParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetTdlibParametersBuilder {
        let mut inner = SetTdlibParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setTdlibParameters".to_string();

        SetTdlibParametersBuilder { inner }
    }

    pub fn use_test_dc(&self) -> bool {
        self.use_test_dc
    }

    pub fn database_directory(&self) -> &String {
        &self.database_directory
    }

    pub fn files_directory(&self) -> &String {
        &self.files_directory
    }

    pub fn database_encryption_key(&self) -> &String {
        &self.database_encryption_key
    }

    pub fn use_file_database(&self) -> bool {
        self.use_file_database
    }

    pub fn use_chat_info_database(&self) -> bool {
        self.use_chat_info_database
    }

    pub fn use_message_database(&self) -> bool {
        self.use_message_database
    }

    pub fn use_secret_chats(&self) -> bool {
        self.use_secret_chats
    }

    pub fn api_id(&self) -> i32 {
        self.api_id
    }

    pub fn api_hash(&self) -> &String {
        &self.api_hash
    }

    pub fn system_language_code(&self) -> &String {
        &self.system_language_code
    }

    pub fn device_model(&self) -> &String {
        &self.device_model
    }

    pub fn system_version(&self) -> &String {
        &self.system_version
    }

    pub fn application_version(&self) -> &String {
        &self.application_version
    }

    pub fn enable_storage_optimizer(&self) -> bool {
        self.enable_storage_optimizer
    }

    pub fn ignore_file_names(&self) -> bool {
        self.ignore_file_names
    }
}

#[doc(hidden)]
pub struct SetTdlibParametersBuilder {
    inner: SetTdlibParameters,
}

#[deprecated]
pub type RTDSetTdlibParametersBuilder = SetTdlibParametersBuilder;

impl SetTdlibParametersBuilder {
    pub fn build(&self) -> SetTdlibParameters {
        self.inner.clone()
    }

    pub fn use_test_dc(&mut self, use_test_dc: bool) -> &mut Self {
        self.inner.use_test_dc = use_test_dc;
        self
    }

    pub fn database_directory<T: AsRef<str>>(&mut self, database_directory: T) -> &mut Self {
        self.inner.database_directory = database_directory.as_ref().to_string();
        self
    }

    pub fn files_directory<T: AsRef<str>>(&mut self, files_directory: T) -> &mut Self {
        self.inner.files_directory = files_directory.as_ref().to_string();
        self
    }

    pub fn database_encryption_key<T: AsRef<str>>(
        &mut self,
        database_encryption_key: T,
    ) -> &mut Self {
        self.inner.database_encryption_key = database_encryption_key.as_ref().to_string();
        self
    }

    pub fn use_file_database(&mut self, use_file_database: bool) -> &mut Self {
        self.inner.use_file_database = use_file_database;
        self
    }

    pub fn use_chat_info_database(&mut self, use_chat_info_database: bool) -> &mut Self {
        self.inner.use_chat_info_database = use_chat_info_database;
        self
    }

    pub fn use_message_database(&mut self, use_message_database: bool) -> &mut Self {
        self.inner.use_message_database = use_message_database;
        self
    }

    pub fn use_secret_chats(&mut self, use_secret_chats: bool) -> &mut Self {
        self.inner.use_secret_chats = use_secret_chats;
        self
    }

    pub fn api_id(&mut self, api_id: i32) -> &mut Self {
        self.inner.api_id = api_id;
        self
    }

    pub fn api_hash<T: AsRef<str>>(&mut self, api_hash: T) -> &mut Self {
        self.inner.api_hash = api_hash.as_ref().to_string();
        self
    }

    pub fn system_language_code<T: AsRef<str>>(&mut self, system_language_code: T) -> &mut Self {
        self.inner.system_language_code = system_language_code.as_ref().to_string();
        self
    }

    pub fn device_model<T: AsRef<str>>(&mut self, device_model: T) -> &mut Self {
        self.inner.device_model = device_model.as_ref().to_string();
        self
    }

    pub fn system_version<T: AsRef<str>>(&mut self, system_version: T) -> &mut Self {
        self.inner.system_version = system_version.as_ref().to_string();
        self
    }

    pub fn application_version<T: AsRef<str>>(&mut self, application_version: T) -> &mut Self {
        self.inner.application_version = application_version.as_ref().to_string();
        self
    }

    pub fn enable_storage_optimizer(&mut self, enable_storage_optimizer: bool) -> &mut Self {
        self.inner.enable_storage_optimizer = enable_storage_optimizer;
        self
    }

    pub fn ignore_file_names(&mut self, ignore_file_names: bool) -> &mut Self {
        self.inner.ignore_file_names = ignore_file_names;
        self
    }
}

impl AsRef<SetTdlibParameters> for SetTdlibParameters {
    fn as_ref(&self) -> &SetTdlibParameters {
        self
    }
}

impl AsRef<SetTdlibParameters> for SetTdlibParametersBuilder {
    fn as_ref(&self) -> &SetTdlibParameters {
        &self.inner
    }
}
