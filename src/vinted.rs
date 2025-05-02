use bon::Builder;
use serde::Serialize;
use uuid::Uuid;

use crate::prelude::*;

/// User authentication request.
///
/// The web version works in two steps:
///
/// 1. Username and password are being sent to trigger the 2FA.
/// 2. 2FA code is being sent to obtain the authentication tokens.
#[must_use]
#[derive(Builder, Serialize)]
pub struct AuthenticationRequest<'a> {
    #[builder(default = "web")]
    pub client_id: &'a str,

    #[builder(default = "user")]
    pub scope: &'a str,

    #[builder(default = "password")]
    pub grant_type: &'a str,

    #[builder(default = true)]
    pub is_trusted_device: bool,

    #[serde(flatten)]
    pub credentials: Credentials<'a>,
}

#[must_use]
#[derive(Serialize)]
#[serde(untagged)]
pub enum Credentials<'a> {
    User(UserAuthentication<'a>),
    TwoFactorChallengeCode(TwoFactorChallengeCode<'a>),
}

#[must_use]
#[derive(Serialize, Builder)]
pub struct UserAuthentication<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[must_use]
#[derive(Serialize, Builder)]
pub struct TwoFactorChallengeCode<'a> {
    #[builder(default = "two_factor_challenge_code")]
    pub password_type: &'a str,

    pub control_code: Uuid,
    pub verification_code: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_authentication_request_ok() -> Result {
        let user = UserAuthentication::builder().username("user").password("pass").build();
        let request = AuthenticationRequest::builder().credentials(Credentials::User(user)).build();
        // language=json
        assert_eq!(
            serde_json::to_string(&request)?,
            r#"{"client_id":"web","scope":"user","grant_type":"password","is_trusted_device":true,"username":"user","password":"pass"}"#
        );
        Ok(())
    }

    #[test]
    fn test_2fa_request_ok() -> Result {
        let code = TwoFactorChallengeCode::builder()
            .control_code(Uuid::nil())
            .verification_code("1234")
            .build();
        let request = AuthenticationRequest::builder()
            .credentials(Credentials::TwoFactorChallengeCode(code))
            .build();
        // language=json
        assert_eq!(
            serde_json::to_string(&request)?,
            r#"{"client_id":"web","scope":"user","grant_type":"password","is_trusted_device":true,"password_type":"two_factor_challenge_code","control_code":"00000000-0000-0000-0000-000000000000","verification_code":"1234"}"#
        );
        Ok(())
    }
}
