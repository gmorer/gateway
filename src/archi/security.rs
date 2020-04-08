use jsonwebtoken::{ encode, EncodingKey, Header, Validation, DecodingKey, decode };
use serde::{ Deserialize, Serialize };
use std::time::{ Duration, SystemTime, UNIX_EPOCH };
use hyper::{ Request, Body };
use once_cell::sync::OnceCell;
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
const ACCESS_SECRET: &str = "super secret";
const REFRESH_SECRET: &str = "another secret";

const ACCESS_TOKEN_DURATION: u64 = 60 * 10; /* 10 minutes in seconds */
const REFRESH_TOKEN_DURATION: u64 = 60 * 60 * 24 * 30; /* 1 month in seconds */

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
	sub: String, /*  Username  */
	exp: usize,  /* expiration */
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
	pub sub: String, /*  Username  */
	exp: usize,  /* expiration */
	iss: String, /*  Token id  */
}

pub enum TokenType {
	AccessToken,
	RefreshToken,
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
			&AccessTokenClaims {
				sub: username,
				exp: get_now_plus(ACCESS_TOKEN_DURATION),
			},
			&EncodingKey::from_secret(ACCESS_SECRET.as_bytes()),
		),
		TokenType::RefreshToken => encode(
			&Header::default(),
			&RefreshTokenClaims {
				sub: username,
				exp: get_now_plus(REFRESH_TOKEN_DURATION),
				iss: "RandomID".to_string(),
			},
			&EncodingKey::from_secret(REFRESH_SECRET.as_bytes()),
		),
	}
	.expect("Error during token creation")
}

pub fn get_username(req: &Request<Body>) -> Result<Option<String>, String> {
	static VALUES: OnceCell<(DecodingKey, Validation )> = OnceCell::new();
	let (decoding_key, validation) = VALUES.get_or_init(|| {
		(DecodingKey::from_secret(ACCESS_SECRET.as_ref()), Validation::default())
	});
	match req.headers().get("Authorization") {
		Some(header) => {
			let header = header.to_str().map_err(|e| format!("Authorization header: {}", e))?;
			if header.len() < "bearer ".len() + 1 {
				return Err("Invalid Authorization header".to_string());
			}
			let header = &header["bearer ".len()..];
			let token = decode::<AccessTokenClaims>(&header, &decoding_key, &validation).map_err(|e| format!("Invalid JWT: {}", e))?;
			Ok(Some(token.claims.sub))
		},
		None => Ok(None)
	}
}

/* pub fn validate token */

/*
	? Refresh token in the midleware ?
	Midleware Data: Validation, decodingKey
*/
