use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains the type of a Telegram Passport element
pub trait TDPassportElementType: Debug + RObject {}

/// Contains the type of a Telegram Passport element
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElementType {
    #[doc(hidden)]
    _Default,
    /// A Telegram Passport element containing the user's address
    #[serde(rename(
        serialize = "passportElementTypeAddress",
        deserialize = "passportElementTypeAddress"
    ))]
    Address(PassportElementTypeAddress),
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(
        serialize = "passportElementTypeBankStatement",
        deserialize = "passportElementTypeBankStatement"
    ))]
    BankStatement(PassportElementTypeBankStatement),
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(
        serialize = "passportElementTypeDriverLicense",
        deserialize = "passportElementTypeDriverLicense"
    ))]
    DriverLicense(PassportElementTypeDriverLicense),
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(
        serialize = "passportElementTypeEmailAddress",
        deserialize = "passportElementTypeEmailAddress"
    ))]
    EmailAddress(PassportElementTypeEmailAddress),
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(
        serialize = "passportElementTypeIdentityCard",
        deserialize = "passportElementTypeIdentityCard"
    ))]
    IdentityCard(PassportElementTypeIdentityCard),
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(
        serialize = "passportElementTypeInternalPassport",
        deserialize = "passportElementTypeInternalPassport"
    ))]
    InternalPassport(PassportElementTypeInternalPassport),
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(
        serialize = "passportElementTypePassport",
        deserialize = "passportElementTypePassport"
    ))]
    Passport(PassportElementTypePassport),
    /// A Telegram Passport element containing the registration page of the user's passport
    #[serde(rename(
        serialize = "passportElementTypePassportRegistration",
        deserialize = "passportElementTypePassportRegistration"
    ))]
    PassportRegistration(PassportElementTypePassportRegistration),
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(
        serialize = "passportElementTypePersonalDetails",
        deserialize = "passportElementTypePersonalDetails"
    ))]
    PersonalDetails(PassportElementTypePersonalDetails),
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(
        serialize = "passportElementTypePhoneNumber",
        deserialize = "passportElementTypePhoneNumber"
    ))]
    PhoneNumber(PassportElementTypePhoneNumber),
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(
        serialize = "passportElementTypeRentalAgreement",
        deserialize = "passportElementTypeRentalAgreement"
    ))]
    RentalAgreement(PassportElementTypeRentalAgreement),
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(
        serialize = "passportElementTypeTemporaryRegistration",
        deserialize = "passportElementTypeTemporaryRegistration"
    ))]
    TemporaryRegistration(PassportElementTypeTemporaryRegistration),
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(
        serialize = "passportElementTypeUtilityBill",
        deserialize = "passportElementTypeUtilityBill"
    ))]
    UtilityBill(PassportElementTypeUtilityBill),
}

impl Default for PassportElementType {
    fn default() -> Self {
        PassportElementType::_Default
    }
}

impl RObject for PassportElementType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PassportElementType::Address(t) => t.extra(),
            PassportElementType::BankStatement(t) => t.extra(),
            PassportElementType::DriverLicense(t) => t.extra(),
            PassportElementType::EmailAddress(t) => t.extra(),
            PassportElementType::IdentityCard(t) => t.extra(),
            PassportElementType::InternalPassport(t) => t.extra(),
            PassportElementType::Passport(t) => t.extra(),
            PassportElementType::PassportRegistration(t) => t.extra(),
            PassportElementType::PersonalDetails(t) => t.extra(),
            PassportElementType::PhoneNumber(t) => t.extra(),
            PassportElementType::RentalAgreement(t) => t.extra(),
            PassportElementType::TemporaryRegistration(t) => t.extra(),
            PassportElementType::UtilityBill(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PassportElementType::Address(t) => t.client_id(),
            PassportElementType::BankStatement(t) => t.client_id(),
            PassportElementType::DriverLicense(t) => t.client_id(),
            PassportElementType::EmailAddress(t) => t.client_id(),
            PassportElementType::IdentityCard(t) => t.client_id(),
            PassportElementType::InternalPassport(t) => t.client_id(),
            PassportElementType::Passport(t) => t.client_id(),
            PassportElementType::PassportRegistration(t) => t.client_id(),
            PassportElementType::PersonalDetails(t) => t.client_id(),
            PassportElementType::PhoneNumber(t) => t.client_id(),
            PassportElementType::RentalAgreement(t) => t.client_id(),
            PassportElementType::TemporaryRegistration(t) => t.client_id(),
            PassportElementType::UtilityBill(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PassportElementType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PassportElementType::_Default)
    }
}

impl AsRef<PassportElementType> for PassportElementType {
    fn as_ref(&self) -> &PassportElementType {
        self
    }
}

/// A Telegram Passport element containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeAddress {}

impl PassportElementTypeAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeAddressBuilder {
        let mut inner = PassportElementTypeAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeAddressBuilder {
    inner: PassportElementTypeAddress,
}

impl RTDPassportElementTypeAddressBuilder {
    pub fn build(&self) -> PassportElementTypeAddress {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeAddress> for PassportElementTypeAddress {
    fn as_ref(&self) -> &PassportElementTypeAddress {
        self
    }
}

impl AsRef<PassportElementTypeAddress> for RTDPassportElementTypeAddressBuilder {
    fn as_ref(&self) -> &PassportElementTypeAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeBankStatement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeBankStatement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeBankStatement {}

impl PassportElementTypeBankStatement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeBankStatementBuilder {
        let mut inner = PassportElementTypeBankStatement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeBankStatementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeBankStatementBuilder {
    inner: PassportElementTypeBankStatement,
}

impl RTDPassportElementTypeBankStatementBuilder {
    pub fn build(&self) -> PassportElementTypeBankStatement {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeBankStatement> for PassportElementTypeBankStatement {
    fn as_ref(&self) -> &PassportElementTypeBankStatement {
        self
    }
}

impl AsRef<PassportElementTypeBankStatement> for RTDPassportElementTypeBankStatementBuilder {
    fn as_ref(&self) -> &PassportElementTypeBankStatement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeDriverLicense {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeDriverLicense {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeDriverLicense {}

impl PassportElementTypeDriverLicense {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeDriverLicenseBuilder {
        let mut inner = PassportElementTypeDriverLicense::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeDriverLicenseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeDriverLicenseBuilder {
    inner: PassportElementTypeDriverLicense,
}

impl RTDPassportElementTypeDriverLicenseBuilder {
    pub fn build(&self) -> PassportElementTypeDriverLicense {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeDriverLicense> for PassportElementTypeDriverLicense {
    fn as_ref(&self) -> &PassportElementTypeDriverLicense {
        self
    }
}

impl AsRef<PassportElementTypeDriverLicense> for RTDPassportElementTypeDriverLicenseBuilder {
    fn as_ref(&self) -> &PassportElementTypeDriverLicense {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeEmailAddress {}

impl PassportElementTypeEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeEmailAddressBuilder {
        let mut inner = PassportElementTypeEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeEmailAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeEmailAddressBuilder {
    inner: PassportElementTypeEmailAddress,
}

impl RTDPassportElementTypeEmailAddressBuilder {
    pub fn build(&self) -> PassportElementTypeEmailAddress {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeEmailAddress> for PassportElementTypeEmailAddress {
    fn as_ref(&self) -> &PassportElementTypeEmailAddress {
        self
    }
}

impl AsRef<PassportElementTypeEmailAddress> for RTDPassportElementTypeEmailAddressBuilder {
    fn as_ref(&self) -> &PassportElementTypeEmailAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeIdentityCard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeIdentityCard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeIdentityCard {}

impl PassportElementTypeIdentityCard {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeIdentityCardBuilder {
        let mut inner = PassportElementTypeIdentityCard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeIdentityCardBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeIdentityCardBuilder {
    inner: PassportElementTypeIdentityCard,
}

impl RTDPassportElementTypeIdentityCardBuilder {
    pub fn build(&self) -> PassportElementTypeIdentityCard {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeIdentityCard> for PassportElementTypeIdentityCard {
    fn as_ref(&self) -> &PassportElementTypeIdentityCard {
        self
    }
}

impl AsRef<PassportElementTypeIdentityCard> for RTDPassportElementTypeIdentityCardBuilder {
    fn as_ref(&self) -> &PassportElementTypeIdentityCard {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeInternalPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeInternalPassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeInternalPassport {}

impl PassportElementTypeInternalPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeInternalPassportBuilder {
        let mut inner = PassportElementTypeInternalPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeInternalPassportBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeInternalPassportBuilder {
    inner: PassportElementTypeInternalPassport,
}

impl RTDPassportElementTypeInternalPassportBuilder {
    pub fn build(&self) -> PassportElementTypeInternalPassport {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeInternalPassport> for PassportElementTypeInternalPassport {
    fn as_ref(&self) -> &PassportElementTypeInternalPassport {
        self
    }
}

impl AsRef<PassportElementTypeInternalPassport> for RTDPassportElementTypeInternalPassportBuilder {
    fn as_ref(&self) -> &PassportElementTypeInternalPassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypePassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypePassport {}

impl PassportElementTypePassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePassportBuilder {
        let mut inner = PassportElementTypePassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypePassportBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportBuilder {
    inner: PassportElementTypePassport,
}

impl RTDPassportElementTypePassportBuilder {
    pub fn build(&self) -> PassportElementTypePassport {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePassport> for PassportElementTypePassport {
    fn as_ref(&self) -> &PassportElementTypePassport {
        self
    }
}

impl AsRef<PassportElementTypePassport> for RTDPassportElementTypePassportBuilder {
    fn as_ref(&self) -> &PassportElementTypePassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the registration page of the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassportRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypePassportRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypePassportRegistration {}

impl PassportElementTypePassportRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePassportRegistrationBuilder {
        let mut inner = PassportElementTypePassportRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypePassportRegistrationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportRegistrationBuilder {
    inner: PassportElementTypePassportRegistration,
}

impl RTDPassportElementTypePassportRegistrationBuilder {
    pub fn build(&self) -> PassportElementTypePassportRegistration {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePassportRegistration> for PassportElementTypePassportRegistration {
    fn as_ref(&self) -> &PassportElementTypePassportRegistration {
        self
    }
}

impl AsRef<PassportElementTypePassportRegistration>
    for RTDPassportElementTypePassportRegistrationBuilder
{
    fn as_ref(&self) -> &PassportElementTypePassportRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypePersonalDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypePersonalDetails {}

impl PassportElementTypePersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePersonalDetailsBuilder {
        let mut inner = PassportElementTypePersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypePersonalDetailsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePersonalDetailsBuilder {
    inner: PassportElementTypePersonalDetails,
}

impl RTDPassportElementTypePersonalDetailsBuilder {
    pub fn build(&self) -> PassportElementTypePersonalDetails {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePersonalDetails> for PassportElementTypePersonalDetails {
    fn as_ref(&self) -> &PassportElementTypePersonalDetails {
        self
    }
}

impl AsRef<PassportElementTypePersonalDetails> for RTDPassportElementTypePersonalDetailsBuilder {
    fn as_ref(&self) -> &PassportElementTypePersonalDetails {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypePhoneNumber {}

impl PassportElementTypePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePhoneNumberBuilder {
        let mut inner = PassportElementTypePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePhoneNumberBuilder {
    inner: PassportElementTypePhoneNumber,
}

impl RTDPassportElementTypePhoneNumberBuilder {
    pub fn build(&self) -> PassportElementTypePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePhoneNumber> for PassportElementTypePhoneNumber {
    fn as_ref(&self) -> &PassportElementTypePhoneNumber {
        self
    }
}

impl AsRef<PassportElementTypePhoneNumber> for RTDPassportElementTypePhoneNumberBuilder {
    fn as_ref(&self) -> &PassportElementTypePhoneNumber {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeRentalAgreement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeRentalAgreement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeRentalAgreement {}

impl PassportElementTypeRentalAgreement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeRentalAgreementBuilder {
        let mut inner = PassportElementTypeRentalAgreement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeRentalAgreementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeRentalAgreementBuilder {
    inner: PassportElementTypeRentalAgreement,
}

impl RTDPassportElementTypeRentalAgreementBuilder {
    pub fn build(&self) -> PassportElementTypeRentalAgreement {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeRentalAgreement> for PassportElementTypeRentalAgreement {
    fn as_ref(&self) -> &PassportElementTypeRentalAgreement {
        self
    }
}

impl AsRef<PassportElementTypeRentalAgreement> for RTDPassportElementTypeRentalAgreementBuilder {
    fn as_ref(&self) -> &PassportElementTypeRentalAgreement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeTemporaryRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeTemporaryRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeTemporaryRegistration {}

impl PassportElementTypeTemporaryRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeTemporaryRegistrationBuilder {
        let mut inner = PassportElementTypeTemporaryRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeTemporaryRegistrationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeTemporaryRegistrationBuilder {
    inner: PassportElementTypeTemporaryRegistration,
}

impl RTDPassportElementTypeTemporaryRegistrationBuilder {
    pub fn build(&self) -> PassportElementTypeTemporaryRegistration {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeTemporaryRegistration> for PassportElementTypeTemporaryRegistration {
    fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration {
        self
    }
}

impl AsRef<PassportElementTypeTemporaryRegistration>
    for RTDPassportElementTypeTemporaryRegistrationBuilder
{
    fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeUtilityBill {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PassportElementTypeUtilityBill {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElementType for PassportElementTypeUtilityBill {}

impl PassportElementTypeUtilityBill {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeUtilityBillBuilder {
        let mut inner = PassportElementTypeUtilityBill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTypeUtilityBillBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeUtilityBillBuilder {
    inner: PassportElementTypeUtilityBill,
}

impl RTDPassportElementTypeUtilityBillBuilder {
    pub fn build(&self) -> PassportElementTypeUtilityBill {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeUtilityBill> for PassportElementTypeUtilityBill {
    fn as_ref(&self) -> &PassportElementTypeUtilityBill {
        self
    }
}

impl AsRef<PassportElementTypeUtilityBill> for RTDPassportElementTypeUtilityBillBuilder {
    fn as_ref(&self) -> &PassportElementTypeUtilityBill {
        &self.inner
    }
}
