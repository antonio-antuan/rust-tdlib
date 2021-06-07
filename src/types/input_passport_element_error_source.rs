use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains the description of an error in a Telegram Passport element; for bots only
pub trait TDInputPassportElementErrorSource: Debug + RObject {}

/// Contains the description of an error in a Telegram Passport element; for bots only
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElementErrorSource {
    #[doc(hidden)]
    _Default,
    /// A data field contains an error. The error is considered resolved when the field's value changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceDataField",
        deserialize = "inputPassportElementErrorSourceDataField"
    ))]
    DataField(InputPassportElementErrorSourceDataField),
    /// The file contains an error. The error is considered resolved when the file changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceFile",
        deserialize = "inputPassportElementErrorSourceFile"
    ))]
    File(InputPassportElementErrorSourceFile),
    /// The list of attached files contains an error. The error is considered resolved when the file list changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceFiles",
        deserialize = "inputPassportElementErrorSourceFiles"
    ))]
    Files(InputPassportElementErrorSourceFiles),
    /// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceFrontSide",
        deserialize = "inputPassportElementErrorSourceFrontSide"
    ))]
    FrontSide(InputPassportElementErrorSourceFrontSide),
    /// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceReverseSide",
        deserialize = "inputPassportElementErrorSourceReverseSide"
    ))]
    ReverseSide(InputPassportElementErrorSourceReverseSide),
    /// The selfie contains an error. The error is considered resolved when the file with the selfie changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceSelfie",
        deserialize = "inputPassportElementErrorSourceSelfie"
    ))]
    Selfie(InputPassportElementErrorSourceSelfie),
    /// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceTranslationFile",
        deserialize = "inputPassportElementErrorSourceTranslationFile"
    ))]
    TranslationFile(InputPassportElementErrorSourceTranslationFile),
    /// The translation of the document contains an error. The error is considered resolved when the list of files changes
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceTranslationFiles",
        deserialize = "inputPassportElementErrorSourceTranslationFiles"
    ))]
    TranslationFiles(InputPassportElementErrorSourceTranslationFiles),
    /// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
    #[serde(rename(
        serialize = "inputPassportElementErrorSourceUnspecified",
        deserialize = "inputPassportElementErrorSourceUnspecified"
    ))]
    Unspecified(InputPassportElementErrorSourceUnspecified),
}

impl Default for InputPassportElementErrorSource {
    fn default() -> Self {
        InputPassportElementErrorSource::_Default
    }
}

impl RObject for InputPassportElementErrorSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputPassportElementErrorSource::DataField(t) => t.extra(),
            InputPassportElementErrorSource::File(t) => t.extra(),
            InputPassportElementErrorSource::Files(t) => t.extra(),
            InputPassportElementErrorSource::FrontSide(t) => t.extra(),
            InputPassportElementErrorSource::ReverseSide(t) => t.extra(),
            InputPassportElementErrorSource::Selfie(t) => t.extra(),
            InputPassportElementErrorSource::TranslationFile(t) => t.extra(),
            InputPassportElementErrorSource::TranslationFiles(t) => t.extra(),
            InputPassportElementErrorSource::Unspecified(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputPassportElementErrorSource::DataField(t) => t.client_id(),
            InputPassportElementErrorSource::File(t) => t.client_id(),
            InputPassportElementErrorSource::Files(t) => t.client_id(),
            InputPassportElementErrorSource::FrontSide(t) => t.client_id(),
            InputPassportElementErrorSource::ReverseSide(t) => t.client_id(),
            InputPassportElementErrorSource::Selfie(t) => t.client_id(),
            InputPassportElementErrorSource::TranslationFile(t) => t.client_id(),
            InputPassportElementErrorSource::TranslationFiles(t) => t.client_id(),
            InputPassportElementErrorSource::Unspecified(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputPassportElementErrorSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputPassportElementErrorSource::_Default)
    }
}

impl AsRef<InputPassportElementErrorSource> for InputPassportElementErrorSource {
    fn as_ref(&self) -> &InputPassportElementErrorSource {
        self
    }
}

/// A data field contains an error. The error is considered resolved when the field's value changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceDataField {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Field name
    field_name: String,
    /// Current data hash
    data_hash: String,
}

impl RObject for InputPassportElementErrorSourceDataField {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceDataField {}

impl InputPassportElementErrorSourceDataField {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceDataFieldBuilder {
        let mut inner = InputPassportElementErrorSourceDataField::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceDataFieldBuilder { inner }
    }

    pub fn field_name(&self) -> &String {
        &self.field_name
    }

    pub fn data_hash(&self) -> &String {
        &self.data_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceDataFieldBuilder {
    inner: InputPassportElementErrorSourceDataField,
}

impl RTDInputPassportElementErrorSourceDataFieldBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceDataField {
        self.inner.clone()
    }

    pub fn field_name<T: AsRef<str>>(&mut self, field_name: T) -> &mut Self {
        self.inner.field_name = field_name.as_ref().to_string();
        self
    }

    pub fn data_hash<T: AsRef<str>>(&mut self, data_hash: T) -> &mut Self {
        self.inner.data_hash = data_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceDataField> for InputPassportElementErrorSourceDataField {
    fn as_ref(&self) -> &InputPassportElementErrorSourceDataField {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceDataField>
    for RTDInputPassportElementErrorSourceDataFieldBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceDataField {
        &self.inner
    }
}

/// The file contains an error. The error is considered resolved when the file changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the file which has the error
    file_hash: String,
}

impl RObject for InputPassportElementErrorSourceFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFile {}

impl InputPassportElementErrorSourceFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceFileBuilder {
        let mut inner = InputPassportElementErrorSourceFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceFileBuilder { inner }
    }

    pub fn file_hash(&self) -> &String {
        &self.file_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFileBuilder {
    inner: InputPassportElementErrorSourceFile,
}

impl RTDInputPassportElementErrorSourceFileBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceFile {
        self.inner.clone()
    }

    pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
        self.inner.file_hash = file_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFile> for InputPassportElementErrorSourceFile {
    fn as_ref(&self) -> &InputPassportElementErrorSourceFile {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFile> for RTDInputPassportElementErrorSourceFileBuilder {
    fn as_ref(&self) -> &InputPassportElementErrorSourceFile {
        &self.inner
    }
}

/// The list of attached files contains an error. The error is considered resolved when the file list changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFiles {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hashes of all attached files
    file_hashes: Vec<String>,
}

impl RObject for InputPassportElementErrorSourceFiles {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFiles {}

impl InputPassportElementErrorSourceFiles {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceFilesBuilder {
        let mut inner = InputPassportElementErrorSourceFiles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceFilesBuilder { inner }
    }

    pub fn file_hashes(&self) -> &Vec<String> {
        &self.file_hashes
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFilesBuilder {
    inner: InputPassportElementErrorSourceFiles,
}

impl RTDInputPassportElementErrorSourceFilesBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceFiles {
        self.inner.clone()
    }

    pub fn file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self {
        self.inner.file_hashes = file_hashes;
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFiles> for InputPassportElementErrorSourceFiles {
    fn as_ref(&self) -> &InputPassportElementErrorSourceFiles {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFiles>
    for RTDInputPassportElementErrorSourceFilesBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceFiles {
        &self.inner
    }
}

/// The front side of the document contains an error. The error is considered resolved when the file with the front side of the document changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceFrontSide {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the file containing the front side
    file_hash: String,
}

impl RObject for InputPassportElementErrorSourceFrontSide {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceFrontSide {}

impl InputPassportElementErrorSourceFrontSide {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceFrontSideBuilder {
        let mut inner = InputPassportElementErrorSourceFrontSide::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceFrontSideBuilder { inner }
    }

    pub fn file_hash(&self) -> &String {
        &self.file_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceFrontSideBuilder {
    inner: InputPassportElementErrorSourceFrontSide,
}

impl RTDInputPassportElementErrorSourceFrontSideBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceFrontSide {
        self.inner.clone()
    }

    pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
        self.inner.file_hash = file_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFrontSide> for InputPassportElementErrorSourceFrontSide {
    fn as_ref(&self) -> &InputPassportElementErrorSourceFrontSide {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceFrontSide>
    for RTDInputPassportElementErrorSourceFrontSideBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceFrontSide {
        &self.inner
    }
}

/// The reverse side of the document contains an error. The error is considered resolved when the file with the reverse side of the document changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceReverseSide {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the file containing the reverse side
    file_hash: String,
}

impl RObject for InputPassportElementErrorSourceReverseSide {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceReverseSide {}

impl InputPassportElementErrorSourceReverseSide {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceReverseSideBuilder {
        let mut inner = InputPassportElementErrorSourceReverseSide::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceReverseSideBuilder { inner }
    }

    pub fn file_hash(&self) -> &String {
        &self.file_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceReverseSideBuilder {
    inner: InputPassportElementErrorSourceReverseSide,
}

impl RTDInputPassportElementErrorSourceReverseSideBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceReverseSide {
        self.inner.clone()
    }

    pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
        self.inner.file_hash = file_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceReverseSide>
    for InputPassportElementErrorSourceReverseSide
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceReverseSide {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceReverseSide>
    for RTDInputPassportElementErrorSourceReverseSideBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceReverseSide {
        &self.inner
    }
}

/// The selfie contains an error. The error is considered resolved when the file with the selfie changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceSelfie {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the file containing the selfie
    file_hash: String,
}

impl RObject for InputPassportElementErrorSourceSelfie {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceSelfie {}

impl InputPassportElementErrorSourceSelfie {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceSelfieBuilder {
        let mut inner = InputPassportElementErrorSourceSelfie::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceSelfieBuilder { inner }
    }

    pub fn file_hash(&self) -> &String {
        &self.file_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceSelfieBuilder {
    inner: InputPassportElementErrorSourceSelfie,
}

impl RTDInputPassportElementErrorSourceSelfieBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceSelfie {
        self.inner.clone()
    }

    pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
        self.inner.file_hash = file_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceSelfie> for InputPassportElementErrorSourceSelfie {
    fn as_ref(&self) -> &InputPassportElementErrorSourceSelfie {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceSelfie>
    for RTDInputPassportElementErrorSourceSelfieBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceSelfie {
        &self.inner
    }
}

/// One of the files containing the translation of the document contains an error. The error is considered resolved when the file with the translation changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the file containing the translation
    file_hash: String,
}

impl RObject for InputPassportElementErrorSourceTranslationFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFile {}

impl InputPassportElementErrorSourceTranslationFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceTranslationFileBuilder {
        let mut inner = InputPassportElementErrorSourceTranslationFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceTranslationFileBuilder { inner }
    }

    pub fn file_hash(&self) -> &String {
        &self.file_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceTranslationFileBuilder {
    inner: InputPassportElementErrorSourceTranslationFile,
}

impl RTDInputPassportElementErrorSourceTranslationFileBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceTranslationFile {
        self.inner.clone()
    }

    pub fn file_hash<T: AsRef<str>>(&mut self, file_hash: T) -> &mut Self {
        self.inner.file_hash = file_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceTranslationFile>
    for InputPassportElementErrorSourceTranslationFile
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFile {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceTranslationFile>
    for RTDInputPassportElementErrorSourceTranslationFileBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFile {
        &self.inner
    }
}

/// The translation of the document contains an error. The error is considered resolved when the list of files changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceTranslationFiles {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hashes of all files with the translation
    file_hashes: Vec<String>,
}

impl RObject for InputPassportElementErrorSourceTranslationFiles {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceTranslationFiles {}

impl InputPassportElementErrorSourceTranslationFiles {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceTranslationFilesBuilder {
        let mut inner = InputPassportElementErrorSourceTranslationFiles::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceTranslationFilesBuilder { inner }
    }

    pub fn file_hashes(&self) -> &Vec<String> {
        &self.file_hashes
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceTranslationFilesBuilder {
    inner: InputPassportElementErrorSourceTranslationFiles,
}

impl RTDInputPassportElementErrorSourceTranslationFilesBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceTranslationFiles {
        self.inner.clone()
    }

    pub fn file_hashes(&mut self, file_hashes: Vec<String>) -> &mut Self {
        self.inner.file_hashes = file_hashes;
        self
    }
}

impl AsRef<InputPassportElementErrorSourceTranslationFiles>
    for InputPassportElementErrorSourceTranslationFiles
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFiles {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceTranslationFiles>
    for RTDInputPassportElementErrorSourceTranslationFilesBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceTranslationFiles {
        &self.inner
    }
}

/// The element contains an error in an unspecified place. The error will be considered resolved when new data is added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementErrorSourceUnspecified {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Current hash of the entire element
    element_hash: String,
}

impl RObject for InputPassportElementErrorSourceUnspecified {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElementErrorSource for InputPassportElementErrorSourceUnspecified {}

impl InputPassportElementErrorSourceUnspecified {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputPassportElementErrorSourceUnspecifiedBuilder {
        let mut inner = InputPassportElementErrorSourceUnspecified::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputPassportElementErrorSourceUnspecifiedBuilder { inner }
    }

    pub fn element_hash(&self) -> &String {
        &self.element_hash
    }
}

#[doc(hidden)]
pub struct RTDInputPassportElementErrorSourceUnspecifiedBuilder {
    inner: InputPassportElementErrorSourceUnspecified,
}

impl RTDInputPassportElementErrorSourceUnspecifiedBuilder {
    pub fn build(&self) -> InputPassportElementErrorSourceUnspecified {
        self.inner.clone()
    }

    pub fn element_hash<T: AsRef<str>>(&mut self, element_hash: T) -> &mut Self {
        self.inner.element_hash = element_hash.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementErrorSourceUnspecified>
    for InputPassportElementErrorSourceUnspecified
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceUnspecified {
        self
    }
}

impl AsRef<InputPassportElementErrorSourceUnspecified>
    for RTDInputPassportElementErrorSourceUnspecifiedBuilder
{
    fn as_ref(&self) -> &InputPassportElementErrorSourceUnspecified {
        &self.inner
    }
}
