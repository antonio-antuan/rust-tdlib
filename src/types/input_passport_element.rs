use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about a Telegram Passport element to be saved
pub trait TDInputPassportElement: Debug + RObject {}

/// Contains information about a Telegram Passport element to be saved
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputPassportElement {
    #[doc(hidden)]
    _Default,
    /// A Telegram Passport element to be saved containing the user's address
    #[serde(rename = "inputPassportElementAddress")]
    Address(InputPassportElementAddress),
    /// A Telegram Passport element to be saved containing the user's bank statement
    #[serde(rename = "inputPassportElementBankStatement")]
    BankStatement(InputPassportElementBankStatement),
    /// A Telegram Passport element to be saved containing the user's driver license
    #[serde(rename = "inputPassportElementDriverLicense")]
    DriverLicense(InputPassportElementDriverLicense),
    /// A Telegram Passport element to be saved containing the user's email address
    #[serde(rename = "inputPassportElementEmailAddress")]
    EmailAddress(InputPassportElementEmailAddress),
    /// A Telegram Passport element to be saved containing the user's identity card
    #[serde(rename = "inputPassportElementIdentityCard")]
    IdentityCard(InputPassportElementIdentityCard),
    /// A Telegram Passport element to be saved containing the user's internal passport
    #[serde(rename = "inputPassportElementInternalPassport")]
    InternalPassport(InputPassportElementInternalPassport),
    /// A Telegram Passport element to be saved containing the user's passport
    #[serde(rename = "inputPassportElementPassport")]
    Passport(InputPassportElementPassport),
    /// A Telegram Passport element to be saved containing the user's passport registration
    #[serde(rename = "inputPassportElementPassportRegistration")]
    PassportRegistration(InputPassportElementPassportRegistration),
    /// A Telegram Passport element to be saved containing the user's personal details
    #[serde(rename = "inputPassportElementPersonalDetails")]
    PersonalDetails(InputPassportElementPersonalDetails),
    /// A Telegram Passport element to be saved containing the user's phone number
    #[serde(rename = "inputPassportElementPhoneNumber")]
    PhoneNumber(InputPassportElementPhoneNumber),
    /// A Telegram Passport element to be saved containing the user's rental agreement
    #[serde(rename = "inputPassportElementRentalAgreement")]
    RentalAgreement(InputPassportElementRentalAgreement),
    /// A Telegram Passport element to be saved containing the user's temporary registration
    #[serde(rename = "inputPassportElementTemporaryRegistration")]
    TemporaryRegistration(InputPassportElementTemporaryRegistration),
    /// A Telegram Passport element to be saved containing the user's utility bill
    #[serde(rename = "inputPassportElementUtilityBill")]
    UtilityBill(InputPassportElementUtilityBill),
}

impl Default for InputPassportElement {
    fn default() -> Self {
        InputPassportElement::_Default
    }
}

impl RObject for InputPassportElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputPassportElement::Address(t) => t.extra(),
            InputPassportElement::BankStatement(t) => t.extra(),
            InputPassportElement::DriverLicense(t) => t.extra(),
            InputPassportElement::EmailAddress(t) => t.extra(),
            InputPassportElement::IdentityCard(t) => t.extra(),
            InputPassportElement::InternalPassport(t) => t.extra(),
            InputPassportElement::Passport(t) => t.extra(),
            InputPassportElement::PassportRegistration(t) => t.extra(),
            InputPassportElement::PersonalDetails(t) => t.extra(),
            InputPassportElement::PhoneNumber(t) => t.extra(),
            InputPassportElement::RentalAgreement(t) => t.extra(),
            InputPassportElement::TemporaryRegistration(t) => t.extra(),
            InputPassportElement::UtilityBill(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputPassportElement::Address(t) => t.client_id(),
            InputPassportElement::BankStatement(t) => t.client_id(),
            InputPassportElement::DriverLicense(t) => t.client_id(),
            InputPassportElement::EmailAddress(t) => t.client_id(),
            InputPassportElement::IdentityCard(t) => t.client_id(),
            InputPassportElement::InternalPassport(t) => t.client_id(),
            InputPassportElement::Passport(t) => t.client_id(),
            InputPassportElement::PassportRegistration(t) => t.client_id(),
            InputPassportElement::PersonalDetails(t) => t.client_id(),
            InputPassportElement::PhoneNumber(t) => t.client_id(),
            InputPassportElement::RentalAgreement(t) => t.client_id(),
            InputPassportElement::TemporaryRegistration(t) => t.client_id(),
            InputPassportElement::UtilityBill(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputPassportElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputPassportElement::_Default)
    }
}

impl AsRef<InputPassportElement> for InputPassportElement {
    fn as_ref(&self) -> &InputPassportElement {
        self
    }
}

/// A Telegram Passport element to be saved containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The address to be saved
    address: Address,
}

impl RObject for InputPassportElementAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementAddress {}

impl InputPassportElementAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementAddressBuilder {
        let mut inner = InputPassportElementAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementAddressBuilder { inner }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[doc(hidden)]
pub struct InputPassportElementAddressBuilder {
    inner: InputPassportElementAddress,
}

#[deprecated]
pub type RTDInputPassportElementAddressBuilder = InputPassportElementAddressBuilder;

impl InputPassportElementAddressBuilder {
    pub fn build(&self) -> InputPassportElementAddress {
        self.inner.clone()
    }

    pub fn address<T: AsRef<Address>>(&mut self, address: T) -> &mut Self {
        self.inner.address = address.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementAddress> for InputPassportElementAddress {
    fn as_ref(&self) -> &InputPassportElementAddress {
        self
    }
}

impl AsRef<InputPassportElementAddress> for InputPassportElementAddressBuilder {
    fn as_ref(&self) -> &InputPassportElementAddress {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementBankStatement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The bank statement to be saved
    bank_statement: InputPersonalDocument,
}

impl RObject for InputPassportElementBankStatement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementBankStatement {}

impl InputPassportElementBankStatement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementBankStatementBuilder {
        let mut inner = InputPassportElementBankStatement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementBankStatementBuilder { inner }
    }

    pub fn bank_statement(&self) -> &InputPersonalDocument {
        &self.bank_statement
    }
}

#[doc(hidden)]
pub struct InputPassportElementBankStatementBuilder {
    inner: InputPassportElementBankStatement,
}

#[deprecated]
pub type RTDInputPassportElementBankStatementBuilder = InputPassportElementBankStatementBuilder;

impl InputPassportElementBankStatementBuilder {
    pub fn build(&self) -> InputPassportElementBankStatement {
        self.inner.clone()
    }

    pub fn bank_statement<T: AsRef<InputPersonalDocument>>(
        &mut self,
        bank_statement: T,
    ) -> &mut Self {
        self.inner.bank_statement = bank_statement.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementBankStatement> for InputPassportElementBankStatement {
    fn as_ref(&self) -> &InputPassportElementBankStatement {
        self
    }
}

impl AsRef<InputPassportElementBankStatement> for InputPassportElementBankStatementBuilder {
    fn as_ref(&self) -> &InputPassportElementBankStatement {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementDriverLicense {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The driver license to be saved
    driver_license: InputIdentityDocument,
}

impl RObject for InputPassportElementDriverLicense {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementDriverLicense {}

impl InputPassportElementDriverLicense {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementDriverLicenseBuilder {
        let mut inner = InputPassportElementDriverLicense::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementDriverLicenseBuilder { inner }
    }

    pub fn driver_license(&self) -> &InputIdentityDocument {
        &self.driver_license
    }
}

#[doc(hidden)]
pub struct InputPassportElementDriverLicenseBuilder {
    inner: InputPassportElementDriverLicense,
}

#[deprecated]
pub type RTDInputPassportElementDriverLicenseBuilder = InputPassportElementDriverLicenseBuilder;

impl InputPassportElementDriverLicenseBuilder {
    pub fn build(&self) -> InputPassportElementDriverLicense {
        self.inner.clone()
    }

    pub fn driver_license<T: AsRef<InputIdentityDocument>>(
        &mut self,
        driver_license: T,
    ) -> &mut Self {
        self.inner.driver_license = driver_license.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementDriverLicense> for InputPassportElementDriverLicense {
    fn as_ref(&self) -> &InputPassportElementDriverLicense {
        self
    }
}

impl AsRef<InputPassportElementDriverLicense> for InputPassportElementDriverLicenseBuilder {
    fn as_ref(&self) -> &InputPassportElementDriverLicense {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The email address to be saved

    #[serde(default)]
    email_address: String,
}

impl RObject for InputPassportElementEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementEmailAddress {}

impl InputPassportElementEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementEmailAddressBuilder {
        let mut inner = InputPassportElementEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementEmailAddressBuilder { inner }
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct InputPassportElementEmailAddressBuilder {
    inner: InputPassportElementEmailAddress,
}

#[deprecated]
pub type RTDInputPassportElementEmailAddressBuilder = InputPassportElementEmailAddressBuilder;

impl InputPassportElementEmailAddressBuilder {
    pub fn build(&self) -> InputPassportElementEmailAddress {
        self.inner.clone()
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementEmailAddress> for InputPassportElementEmailAddress {
    fn as_ref(&self) -> &InputPassportElementEmailAddress {
        self
    }
}

impl AsRef<InputPassportElementEmailAddress> for InputPassportElementEmailAddressBuilder {
    fn as_ref(&self) -> &InputPassportElementEmailAddress {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementIdentityCard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identity card to be saved
    identity_card: InputIdentityDocument,
}

impl RObject for InputPassportElementIdentityCard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementIdentityCard {}

impl InputPassportElementIdentityCard {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementIdentityCardBuilder {
        let mut inner = InputPassportElementIdentityCard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementIdentityCardBuilder { inner }
    }

    pub fn identity_card(&self) -> &InputIdentityDocument {
        &self.identity_card
    }
}

#[doc(hidden)]
pub struct InputPassportElementIdentityCardBuilder {
    inner: InputPassportElementIdentityCard,
}

#[deprecated]
pub type RTDInputPassportElementIdentityCardBuilder = InputPassportElementIdentityCardBuilder;

impl InputPassportElementIdentityCardBuilder {
    pub fn build(&self) -> InputPassportElementIdentityCard {
        self.inner.clone()
    }

    pub fn identity_card<T: AsRef<InputIdentityDocument>>(
        &mut self,
        identity_card: T,
    ) -> &mut Self {
        self.inner.identity_card = identity_card.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementIdentityCard> for InputPassportElementIdentityCard {
    fn as_ref(&self) -> &InputPassportElementIdentityCard {
        self
    }
}

impl AsRef<InputPassportElementIdentityCard> for InputPassportElementIdentityCardBuilder {
    fn as_ref(&self) -> &InputPassportElementIdentityCard {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementInternalPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The internal passport to be saved
    internal_passport: InputIdentityDocument,
}

impl RObject for InputPassportElementInternalPassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementInternalPassport {}

impl InputPassportElementInternalPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementInternalPassportBuilder {
        let mut inner = InputPassportElementInternalPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementInternalPassportBuilder { inner }
    }

    pub fn internal_passport(&self) -> &InputIdentityDocument {
        &self.internal_passport
    }
}

#[doc(hidden)]
pub struct InputPassportElementInternalPassportBuilder {
    inner: InputPassportElementInternalPassport,
}

#[deprecated]
pub type RTDInputPassportElementInternalPassportBuilder =
    InputPassportElementInternalPassportBuilder;

impl InputPassportElementInternalPassportBuilder {
    pub fn build(&self) -> InputPassportElementInternalPassport {
        self.inner.clone()
    }

    pub fn internal_passport<T: AsRef<InputIdentityDocument>>(
        &mut self,
        internal_passport: T,
    ) -> &mut Self {
        self.inner.internal_passport = internal_passport.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementInternalPassport> for InputPassportElementInternalPassport {
    fn as_ref(&self) -> &InputPassportElementInternalPassport {
        self
    }
}

impl AsRef<InputPassportElementInternalPassport> for InputPassportElementInternalPassportBuilder {
    fn as_ref(&self) -> &InputPassportElementInternalPassport {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The passport to be saved
    passport: InputIdentityDocument,
}

impl RObject for InputPassportElementPassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementPassport {}

impl InputPassportElementPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementPassportBuilder {
        let mut inner = InputPassportElementPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementPassportBuilder { inner }
    }

    pub fn passport(&self) -> &InputIdentityDocument {
        &self.passport
    }
}

#[doc(hidden)]
pub struct InputPassportElementPassportBuilder {
    inner: InputPassportElementPassport,
}

#[deprecated]
pub type RTDInputPassportElementPassportBuilder = InputPassportElementPassportBuilder;

impl InputPassportElementPassportBuilder {
    pub fn build(&self) -> InputPassportElementPassport {
        self.inner.clone()
    }

    pub fn passport<T: AsRef<InputIdentityDocument>>(&mut self, passport: T) -> &mut Self {
        self.inner.passport = passport.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementPassport> for InputPassportElementPassport {
    fn as_ref(&self) -> &InputPassportElementPassport {
        self
    }
}

impl AsRef<InputPassportElementPassport> for InputPassportElementPassportBuilder {
    fn as_ref(&self) -> &InputPassportElementPassport {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's passport registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPassportRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The passport registration page to be saved
    passport_registration: InputPersonalDocument,
}

impl RObject for InputPassportElementPassportRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementPassportRegistration {}

impl InputPassportElementPassportRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementPassportRegistrationBuilder {
        let mut inner = InputPassportElementPassportRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementPassportRegistrationBuilder { inner }
    }

    pub fn passport_registration(&self) -> &InputPersonalDocument {
        &self.passport_registration
    }
}

#[doc(hidden)]
pub struct InputPassportElementPassportRegistrationBuilder {
    inner: InputPassportElementPassportRegistration,
}

#[deprecated]
pub type RTDInputPassportElementPassportRegistrationBuilder =
    InputPassportElementPassportRegistrationBuilder;

impl InputPassportElementPassportRegistrationBuilder {
    pub fn build(&self) -> InputPassportElementPassportRegistration {
        self.inner.clone()
    }

    pub fn passport_registration<T: AsRef<InputPersonalDocument>>(
        &mut self,
        passport_registration: T,
    ) -> &mut Self {
        self.inner.passport_registration = passport_registration.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementPassportRegistration> for InputPassportElementPassportRegistration {
    fn as_ref(&self) -> &InputPassportElementPassportRegistration {
        self
    }
}

impl AsRef<InputPassportElementPassportRegistration>
    for InputPassportElementPassportRegistrationBuilder
{
    fn as_ref(&self) -> &InputPassportElementPassportRegistration {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Personal details of the user
    personal_details: PersonalDetails,
}

impl RObject for InputPassportElementPersonalDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementPersonalDetails {}

impl InputPassportElementPersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementPersonalDetailsBuilder {
        let mut inner = InputPassportElementPersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementPersonalDetailsBuilder { inner }
    }

    pub fn personal_details(&self) -> &PersonalDetails {
        &self.personal_details
    }
}

#[doc(hidden)]
pub struct InputPassportElementPersonalDetailsBuilder {
    inner: InputPassportElementPersonalDetails,
}

#[deprecated]
pub type RTDInputPassportElementPersonalDetailsBuilder = InputPassportElementPersonalDetailsBuilder;

impl InputPassportElementPersonalDetailsBuilder {
    pub fn build(&self) -> InputPassportElementPersonalDetails {
        self.inner.clone()
    }

    pub fn personal_details<T: AsRef<PersonalDetails>>(
        &mut self,
        personal_details: T,
    ) -> &mut Self {
        self.inner.personal_details = personal_details.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementPersonalDetails> for InputPassportElementPersonalDetails {
    fn as_ref(&self) -> &InputPassportElementPersonalDetails {
        self
    }
}

impl AsRef<InputPassportElementPersonalDetails> for InputPassportElementPersonalDetailsBuilder {
    fn as_ref(&self) -> &InputPassportElementPersonalDetails {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The phone number to be saved

    #[serde(default)]
    phone_number: String,
}

impl RObject for InputPassportElementPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementPhoneNumber {}

impl InputPassportElementPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementPhoneNumberBuilder {
        let mut inner = InputPassportElementPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct InputPassportElementPhoneNumberBuilder {
    inner: InputPassportElementPhoneNumber,
}

#[deprecated]
pub type RTDInputPassportElementPhoneNumberBuilder = InputPassportElementPhoneNumberBuilder;

impl InputPassportElementPhoneNumberBuilder {
    pub fn build(&self) -> InputPassportElementPhoneNumber {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<InputPassportElementPhoneNumber> for InputPassportElementPhoneNumber {
    fn as_ref(&self) -> &InputPassportElementPhoneNumber {
        self
    }
}

impl AsRef<InputPassportElementPhoneNumber> for InputPassportElementPhoneNumberBuilder {
    fn as_ref(&self) -> &InputPassportElementPhoneNumber {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementRentalAgreement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The rental agreement to be saved
    rental_agreement: InputPersonalDocument,
}

impl RObject for InputPassportElementRentalAgreement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementRentalAgreement {}

impl InputPassportElementRentalAgreement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementRentalAgreementBuilder {
        let mut inner = InputPassportElementRentalAgreement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementRentalAgreementBuilder { inner }
    }

    pub fn rental_agreement(&self) -> &InputPersonalDocument {
        &self.rental_agreement
    }
}

#[doc(hidden)]
pub struct InputPassportElementRentalAgreementBuilder {
    inner: InputPassportElementRentalAgreement,
}

#[deprecated]
pub type RTDInputPassportElementRentalAgreementBuilder = InputPassportElementRentalAgreementBuilder;

impl InputPassportElementRentalAgreementBuilder {
    pub fn build(&self) -> InputPassportElementRentalAgreement {
        self.inner.clone()
    }

    pub fn rental_agreement<T: AsRef<InputPersonalDocument>>(
        &mut self,
        rental_agreement: T,
    ) -> &mut Self {
        self.inner.rental_agreement = rental_agreement.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementRentalAgreement> for InputPassportElementRentalAgreement {
    fn as_ref(&self) -> &InputPassportElementRentalAgreement {
        self
    }
}

impl AsRef<InputPassportElementRentalAgreement> for InputPassportElementRentalAgreementBuilder {
    fn as_ref(&self) -> &InputPassportElementRentalAgreement {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementTemporaryRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The temporary registration document to be saved
    temporary_registration: InputPersonalDocument,
}

impl RObject for InputPassportElementTemporaryRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementTemporaryRegistration {}

impl InputPassportElementTemporaryRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementTemporaryRegistrationBuilder {
        let mut inner = InputPassportElementTemporaryRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementTemporaryRegistrationBuilder { inner }
    }

    pub fn temporary_registration(&self) -> &InputPersonalDocument {
        &self.temporary_registration
    }
}

#[doc(hidden)]
pub struct InputPassportElementTemporaryRegistrationBuilder {
    inner: InputPassportElementTemporaryRegistration,
}

#[deprecated]
pub type RTDInputPassportElementTemporaryRegistrationBuilder =
    InputPassportElementTemporaryRegistrationBuilder;

impl InputPassportElementTemporaryRegistrationBuilder {
    pub fn build(&self) -> InputPassportElementTemporaryRegistration {
        self.inner.clone()
    }

    pub fn temporary_registration<T: AsRef<InputPersonalDocument>>(
        &mut self,
        temporary_registration: T,
    ) -> &mut Self {
        self.inner.temporary_registration = temporary_registration.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementTemporaryRegistration>
    for InputPassportElementTemporaryRegistration
{
    fn as_ref(&self) -> &InputPassportElementTemporaryRegistration {
        self
    }
}

impl AsRef<InputPassportElementTemporaryRegistration>
    for InputPassportElementTemporaryRegistrationBuilder
{
    fn as_ref(&self) -> &InputPassportElementTemporaryRegistration {
        &self.inner
    }
}

/// A Telegram Passport element to be saved containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPassportElementUtilityBill {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The utility bill to be saved
    utility_bill: InputPersonalDocument,
}

impl RObject for InputPassportElementUtilityBill {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputPassportElement for InputPassportElementUtilityBill {}

impl InputPassportElementUtilityBill {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputPassportElementUtilityBillBuilder {
        let mut inner = InputPassportElementUtilityBill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputPassportElementUtilityBillBuilder { inner }
    }

    pub fn utility_bill(&self) -> &InputPersonalDocument {
        &self.utility_bill
    }
}

#[doc(hidden)]
pub struct InputPassportElementUtilityBillBuilder {
    inner: InputPassportElementUtilityBill,
}

#[deprecated]
pub type RTDInputPassportElementUtilityBillBuilder = InputPassportElementUtilityBillBuilder;

impl InputPassportElementUtilityBillBuilder {
    pub fn build(&self) -> InputPassportElementUtilityBill {
        self.inner.clone()
    }

    pub fn utility_bill<T: AsRef<InputPersonalDocument>>(&mut self, utility_bill: T) -> &mut Self {
        self.inner.utility_bill = utility_bill.as_ref().clone();
        self
    }
}

impl AsRef<InputPassportElementUtilityBill> for InputPassportElementUtilityBill {
    fn as_ref(&self) -> &InputPassportElementUtilityBill {
        self
    }
}

impl AsRef<InputPassportElementUtilityBill> for InputPassportElementUtilityBillBuilder {
    fn as_ref(&self) -> &InputPassportElementUtilityBill {
        &self.inner
    }
}
