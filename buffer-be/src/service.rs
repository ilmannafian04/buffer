use chrono::{DateTime, Timelike, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::User;

#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    #[serde(with = "claim_serde")]
    iat: DateTime<Utc>,
    #[serde(with = "claim_serde")]
    exp: DateTime<Utc>,
}

impl Claims {
    fn new(subject: i32, iat: DateTime<Utc>, exp: DateTime<Utc>) -> Self {
        Self {
            sub: subject.to_string(),
            iat: iat
                .date()
                .and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0),
            exp: exp
                .date()
                .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0),
        }
    }

    fn into_jwt_string(self, secret_key: &String) -> String {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&secret_key.as_bytes()),
        )
        .unwrap()
    }
}

mod claim_serde {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| de::Error::custom("Invalid unix timestamp value"))
    }
}

impl User {
    pub fn authennticate(self, password: &String) -> Result<String, ()> {
        match argon2::verify_encoded(&self.password, &password.as_bytes()) {
            Ok(is_valid) => {
                if is_valid {
                    Ok(
                        Claims::new(self.id, Utc::now(), Utc::now() + chrono::Duration::weeks(4))
                            .into_jwt_string(&"test_key".to_owned()),
                    )
                } else {
                    Err(())
                }
            }
            Err(_) => Err(()),
        }
    }
}
