use std::sync::LazyLock;

use regex::Regex;
use sqlx::FromRow;

use thiserror::Error;

use crate::util;

#[derive(sqlx::Encode, sqlx::Decode)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Option<Self> {
        static ALLOWED_USERNAME_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("^[a-zA-Z0-9._@-]{6,25}$").unwrap());

        ALLOWED_USERNAME_REGEX
            .is_match(&username)
            .then_some(Self(username))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl sqlx::Type<sqlx::Sqlite> for Username {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("password pair mismatch")]
    PairMismatch,
    #[error("user input doesn't meet requirements")]
    RequirementsMismatch,
    #[error("failed to generate password hash: {0}")]
    HashFailed(pbkdf2::password_hash::Error),
}

#[derive(sqlx::Encode, sqlx::Decode)]
pub struct Password(String);

impl Password {
    pub fn new(password: String, password_v2: String) -> Result<Self, PasswordError> {
        (password == password_v2)
            .then_some(())
            .ok_or(PasswordError::PairMismatch)?;

        matches!(password.len(), 8..30)
            .then_some(())
            .ok_or(PasswordError::RequirementsMismatch)?;

        let hash = util::hash_string(&password).map_err(|err| PasswordError::HashFailed(err))?;
        Ok(Self(hash))
    }

    pub fn verify(&self, password: &str) -> bool {
        util::verify_hash(password, &self.0).is_some()
    }

    pub fn as_hash_str(&self) -> &str {
        self.0.as_str()
    }
}

impl sqlx::Type<sqlx::Sqlite> for Password {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[derive(FromRow)]
pub struct DbSdkAccountRow {
    pub uid: i32,
    pub token: String,
    pub username: Username,
    pub password: Password,
}
