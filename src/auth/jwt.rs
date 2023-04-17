extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac, NewMac};
use jwt::{Header, Token, VerifyWithKey, token};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use actix_web::HttpRequest;
use std::time::{SystemTime, UNIX_EPOCH};


// TODO: replace i32 to u64
pub struct JwtToken {
    pub user_id: i32,
    pub created_date: i32, 
    pub body: String
}

impl JwtToken {
    // TODO: secret string should be replaced by something more secure
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let now = SystemTime::now();
        let date = now.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        claims.insert("created_date", date as i32);
        let token_str = claims.sign_with_key(&key).unwrap();
        return token_str
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"secret").unwrap();
        let token_str = encoded_token.as_str();
        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> = 
            VerifyWithKey::verify_with_key(token_str, &key);
        match token {
            Ok(token) => {
                let claims = token.claims();
                return Ok(JwtToken { 
                    user_id: claims["user_id"], 
                    created_date: claims["created_date"],
                    body: encoded_token 
                })
            },
            Err(_) => return Err("Could not decode token")
        }
    }

    pub fn decode_from_request(req: HttpRequest) -> Result<JwtToken, &'static str> {
        match req.headers().get("user-token") {
            Some(token) => JwtToken::decode(String::from(token.to_str().unwrap())),
            None => Err("There is no token in request ")
        }
    }
}


