use crate::entities::user::UserEntity;
use anyhow::Context;
use hmac::{Hmac, Mac};
use jwt;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;
use uuid::Uuid;

pub fn create_token(
    user: &UserEntity,
    jwt_hmac_key: &str,
    expiration: i64,
) -> anyhow::Result<String> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_hmac_key.as_bytes())?;
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("sub".to_string(), user.id.to_string());
    claims.insert(
        "exp".to_string(),
        (chrono::Utc::now().timestamp() + expiration).to_string(),
    );
    claims.insert(
        "iat".to_string(),
        chrono::Utc::now().timestamp().to_string(),
    );
    claims.insert("iss".to_string(), "kowalski".to_string());
    claims.insert("aud".to_string(), "kowalski".to_string());

    let token = Token::new(header, claims).sign_with_key(&key)?;
    return Ok(token.as_str().to_string());
}

pub fn verify_token(token: String, jwt_hmac_key: &str) -> anyhow::Result<Uuid> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(jwt_hmac_key.as_bytes())?;
    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)?;
    let header = token.header();
    if header.algorithm != AlgorithmType::Hs384 {
        anyhow::bail!("Invalid Token")
    }
    let claims = token.claims();
    let sub = claims.get("sub").context("No sub claim")?;

    let exp = claims.get("exp").context("No exp claim")?;
    let exp = exp.parse::<i64>().context("Invalid exp claim")?;
    let now = chrono::Utc::now().timestamp();
    if now > exp {
        anyhow::bail!("Token expired")
    }
    let user_id = Uuid::parse_str(sub).context("Invalid sub claim")?;
    Ok(user_id)
}
