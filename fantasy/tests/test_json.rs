

pub enum UpdateAuthorizationState {
  Closed             (AuthorizationStateClosed               ),
  Closing            (AuthorizationStateClosing              ),
  LoggingOut         (AuthorizationStateLoggingOut           ),
  Ready              (AuthorizationStateReady                ),
  WaitCode           (AuthorizationStateWaitCode             ),
  WaitEncryptionKey  (AuthorizationStateWaitEncryptionKey    ),
  WaitPassword       (AuthorizationStateWaitPassword         ),
  WaitPhoneNumber    (AuthorizationStateWaitPhoneNumber      ),
  WaitTdlibParameters(AuthorizationStateWaitTdlibParameters  ),
}

pub struct AuthorizationStateClosed {}

pub struct AuthorizationStateClosing {}

pub struct AuthorizationStateLoggingOut {}

pub struct AuthorizationStateReady {}

pub struct AuthorizationStateWaitCode {
  /// True, if the user is already registered.
  is_registered: Option<bool>,
  /// Telegram terms of service, which should be accepted before user can continue registration; may be null.
  terms_of_service: Option<TermsOfService>,
  /// Information about the authorization code that was sent.
  code_info: Option<AuthenticationCodeInfo>,
}

pub struct AuthorizationStateWaitEncryptionKey {
  /// True, if the database is currently encrypted.
  is_encrypted: Option<bool>,
}

pub struct AuthorizationStateWaitPassword {
  /// Hint for the password; may be empty.
  password_hint: Option<String>,
  /// True if a recovery email address has been set up.
  has_recovery_email_address: Option<bool>,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent.
  recovery_email_address_pattern: Option<String>,

}

pub struct AuthorizationStateWaitPhoneNumber {}

pub struct AuthorizationStateWaitTdlibParameters {}

pub struct AuthenticationCodeInfo {
  /// A phone number that is being authenticated.
  phone_number: String,
  /// Describes the way the code was sent to the user.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: AuthenticationCodeType,
  /// Describes the way the next code will be sent to the user; may be null.
  next_type: Option<AuthenticationCodeType>,
  /// Timeout before the code should be re-sent, in seconds.
  timeout: i32,
}

pub enum AuthenticationCodeType {
  TypeCall             (AuthenticationCodeTypeCall            ),
  TypeFlashCall        (AuthenticationCodeTypeFlashCall       ),
  TypeSms              (AuthenticationCodeTypeSms             ),
  TypeTelegramMessage  (AuthenticationCodeTypeTelegramMessage ),
}

pub struct AuthenticationCodeTypeCall {
  /// Length of the code.
  length: i32,
}

pub struct AuthenticationCodeTypeFlashCall {
  /// Pattern of the phone number from which the call will be made.
  pattern: String,

}

pub struct AuthenticationCodeTypeSms {
  /// Length of the code.
  length: i32,

}

pub struct AuthenticationCodeTypeTelegramMessage {
  /// Length of the code.
  length: i32,
}



#[test]
fn test_authorization_state() {
  let json = r#"{"@type":"updateAuthorizationState","@struct":"UpdateAuthorizationState","authorization_state":{"@type":"authorizationStateWaitTdlibParameters","@struct":"AuthorizationStateWaitTdlibParameters"}}"#;

}
