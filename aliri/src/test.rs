#![allow(dead_code)]

use std::collections::HashSet;

use once_cell::sync::Lazy;

use crate::jwt;

#[cfg(feature = "rsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "rsa")))]
pub mod rsa {
    pub const TEST_KEY_ID: &str = "EkKhyPqtd";
    pub const JWK: &str = include_str!("../data/rsa/jwk.json");
    pub const JWK_MINIMAL: &str = include_str!("../data/rsa/jwk-min.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_WITH_PRIVATE_KEY: &str = include_str!("../data/rsa/jwk-priv.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_WITH_MINIMAL_PRIVATE_KEY: &str = include_str!("../data/rsa/jwk-priv-min.json");

    pub const JWKS: &str = include_str!("../data/rsa/jwks.json");
}

#[cfg(feature = "ec")]
#[cfg_attr(docsrs, doc(cfg(feature = "ec")))]
pub mod ec {
    pub const TEST_KEY_ID: &str = "VJUjkP9KO";
    pub const JWK_P256: &str = include_str!("../data/ec/jwk-p256.json");
    pub const JWK_P256_MINIMAL: &str = include_str!("../data/ec/jwk-p256-min.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P256_WITH_PRIVATE_KEY: &str = include_str!("../data/ec/jwk-p256-priv.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P256_WITH_MINIMAL_PRIVATE_KEY: &str =
        include_str!("../data/ec/jwk-p256-priv-min.json");

    pub const JWK_P384: &str = include_str!("../data/ec/jwk-p384.json");
    pub const JWK_P384_MINIMAL: &str = include_str!("../data/ec/jwk-p384-min.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P384_WITH_PRIVATE_KEY: &str = include_str!("../data/ec/jwk-p384-priv.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P384_WITH_MINIMAL_PRIVATE_KEY: &str =
        include_str!("../data/ec/jwk-p384-priv-min.json");

    pub const JWK_P521: &str = include_str!("../data/ec/jwk-p521.json");
    pub const JWK_P521_MINIMAL: &str = include_str!("../data/ec/jwk-p521-min.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P521_WITH_PRIVATE_KEY: &str = include_str!("../data/ec/jwk-p521-priv.json");
    #[cfg(feature = "private-keys")]
    #[cfg_attr(docsrs, doc(cfg(feature = "private-keys")))]
    pub const JWK_P521_WITH_MINIMAL_PRIVATE_KEY: &str =
        include_str!("../data/ec/jwk-p521-priv-min.json");
}

#[cfg(feature = "hmac")]
#[cfg_attr(docsrs, doc(cfg(feature = "hmac")))]
pub mod hmac {
    pub const TEST_KEY_ID: &str = "4y_2kKqYO";
    pub const JWK: &str = include_str!("../data/hmac/jwk.json");
    pub const JWK_MINIMAL: &str = include_str!("../data/hmac/jwk-min.json");
}

#[cfg(all(feature = "hmac", feature = "rsa", feature = "ec"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "hmac", feature = "rsa", feature = "ec")))
)]
pub mod mixed {
    pub const JWKS: &str = include_str!("../data/jwks.json");
}

pub static TEST_AUD: Lazy<&'static jwt::AudienceRef> =
    Lazy::new(|| jwt::AudienceRef::from_str("TEST_AUDIENCE"));
pub static VALID_AUD: Lazy<HashSet<String>> = Lazy::new(|| {
    [TEST_AUD.as_str()]
        .iter()
        .map(|&s| String::from(s))
        .collect()
});
