use serde::{ Serialize, Deserialize };
use jsonwebtoken::{ encode, Header, EncodingKey };
use std::time::{ Duration, SystemTime, UNIX_EPOCH };
/* 
	Access token are used to access api endpoints it live only 10 minutes
	sub: username
	exp: timestamp of the date generated plus 10 minutes
*/

/* 
	Refresh token are used to generate new Access token
	sub: username
	exp: timestamp of the date generated plus 1 month
	iss: ID of the token ( can be blacklisted ) if token not in database < ID >
*/

// TODO: .env file
const JWT_SECRET: &str = "Toto-Tata";

const ACCESS_TOKEN_DURATION: u64 = 60 * 10; /* 10 minutes in seconds */
// const ACCESS_TOKEN_DURATION: Duration = Duration::from_secs(60 * 10); /* 10 minutes in seconds */
const REFRESH_TOKEN_DURATION: u64 = 60 * 60 * 24 * 30; /* 1 month in seconds */
// const REFRESH_TOKEN_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 30); /* 1 month in seconds */

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
	sub: String,	/*  Username  */
	exp: usize,		/* expiration */
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
	sub: String,	/*  Username  */
	exp: usize,		/* expiration */
	iss: String,	/*  Token id  */
}

pub enum TokenType {
	AccessToken,
	RefreshToken
}

fn get_now_plus(exp: u64) -> usize {
	SystemTime::now()
		.checked_add(Duration::from_secs(exp))
		.expect("Error during timestamp manipulation")
		.duration_since(UNIX_EPOCH)
		.expect("Error during timestamp manipulation")
		.as_secs() as usize
}

pub fn create_token(username: String, token: TokenType) -> String {
	match token {
		TokenType::AccessToken => encode(
			&Header::default(),
			&AccessTokenClaims { sub: username, exp: get_now_plus(ACCESS_TOKEN_DURATION) },
			&EncodingKey::from_secret(JWT_SECRET.as_bytes())
		),
		TokenType::RefreshToken => encode(
			&Header::default(),
			&RefreshTokenClaims { sub: username, exp: get_now_plus(REFRESH_TOKEN_DURATION), iss: "RandomID".to_string() },
			&EncodingKey::from_secret(JWT_SECRET.as_bytes())
		)
	}.expect("Error during token creation")
}