//! Simple abstraction for a TOML based Eludris configuration file
mod effis;
mod email;
mod oprish;
mod pandemonium;

use serde::{Deserialize, Serialize};

#[cfg(feature = "logic")]
use anyhow::{bail, Context};
#[cfg(feature = "logic")]
use std::str::FromStr;
#[cfg(feature = "logic")]
use std::{env, fs, path};
#[cfg(feature = "logic")]
use url::Url;

pub use effis::*;
pub use email::*;
pub use oprish::*;
pub use pandemonium::*;

/// Represents a single rate limit.
///
/// -----
///
/// ### Example
///
/// ```json
/// {
///   "reset_after": 60,
///   "limit": 30
/// }
/// ```
#[autodoc(category = "Instance")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitConf {
    /// The amount of seconds after which the rate limit resets.
    pub reset_after: u32,
    /// The amount of requests that can be made within the `reset_after` interval.
    pub limit: u32,
}

#[cfg(feature = "logic")]
macro_rules! validate_rate_limit_limits {
    ($rate_limits:expr, $($bucket_name:ident),+) => {
        if $(
            $rate_limits.$bucket_name.limit == 0
            )||+ {
            bail!("RateLimit limit can't be 0");
        }
    };
}

#[cfg(feature = "logic")]
macro_rules! validate_file_sizes {
    ($($size:expr),+) => {
        if $(
            $size == 0
            )||+ {
            bail!("File size can't be 0");
        }
    };
}

/// Eludris config used for the `Eludris.toml` file.
///
/// For a full example of this check the
/// `[Eludris.toml](https://github.com/eludris/eludris/blob/main/Eludris.toml)` file in the meta repository.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub instance_name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub oprish: OprishConf,
    #[serde(default)]
    pub pandemonium: PandemoniumConf,
    #[serde(default)]
    pub effis: EffisConf,
    #[serde(default)]
    pub email: Option<Email>,
}

#[cfg(feature = "logic")]
impl Conf {
    /// Create a new [`Conf`].
    ///
    /// # Panics
    ///
    /// This function is *intended* to panic if a suitable config is not found.
    ///
    /// That also includes the config file's data failing to deserialise.
    pub fn new<T: AsRef<path::Path>>(path: T) -> anyhow::Result<Self> {
        let data = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file {}", path.as_ref().display()))?;
        let data: Self = toml::from_str(&data).with_context(|| {
            format!("Could not parse {} as valid toml", path.as_ref().display())
        })?;
        data.validate()?;
        Ok(data)
    }

    /// Create a new [`Conf`] by determining it's path based on the "ELUDRIS_CONF" environment
    /// variable or falling back to "Eludris.toml" if it is not found.
    ///
    /// # Panics
    ///
    /// This function is *intended* to panic if a suitable config is not found.
    ///
    /// That also includes the config file's data failing to deserialise.
    pub fn new_from_env() -> anyhow::Result<Self> {
        Self::new(env::var("ELUDRIS_CONF").unwrap_or_else(|_| "Eludris.toml".to_string()))
    }

    #[cfg(test)]
    /// Create a new [`Conf`] with default config from the provided instance name.
    fn from_name(instance_name: String) -> anyhow::Result<Self> {
        let conf = Self {
            instance_name,
            description: None,
            oprish: OprishConf::default(),
            pandemonium: PandemoniumConf::default(),
            effis: EffisConf::default(),
            email: None,
        };
        conf.validate()?;
        Ok(conf)
    }

    /// Validates a [`Conf`]
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.instance_name.is_empty() || self.instance_name.len() > 32 {
            bail!("Invalid instance_name length, must be between 1 and 32 characters long");
        }
        if let Some(description) = &self.description {
            if description.is_empty() || description.len() > 2048 {
                bail!("Invalid description length, must be between 1 and 2048 characters long");
            }
        }
        if self.oprish.message_limit < 1024 {
            bail!("Message limit can not be less than 1024 characters");
        }
        validate_rate_limit_limits!(self.oprish.rate_limits, get_instance_info, create_message);
        validate_rate_limit_limits!(self.pandemonium, rate_limit);
        validate_rate_limit_limits!(self.effis.rate_limits, assets, attachments, fetch_file);

        Url::parse(&self.oprish.url)
            .with_context(|| format!("Invalid oprish url {}", self.oprish.url))?;
        Url::parse(&self.pandemonium.url)
            .with_context(|| format!("Invalid pandemonium url {}", self.pandemonium.url))?;
        Url::parse(&self.effis.url)
            .with_context(|| format!("Invalid effis url {}", self.effis.url))?;

        if let Some(email) = &self.email {
            if email.relay.is_empty() {
                bail!("Invalid SMTP relay url");
            }
            if email.name.is_empty() {
                bail!("Invalid email name");
            }
            if email.address.is_empty() {
                bail!("Invalid email address");
            }
        }

        validate_file_sizes!(
            self.effis.file_size,
            self.effis.attachment_file_size,
            self.effis.rate_limits.assets.file_size_limit,
            self.effis.rate_limits.attachments.file_size_limit
        );

        Ok(())
    }
}

#[cfg(feature = "logic")]
impl FromStr for Conf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Self = toml::from_str(s).context("Could not parse provided toml as a Conf")?;
        data.validate()?;
        Ok(data)
    }
}

#[cfg(feature = "logic")]
#[cfg(test)]
mod tests {
    use crate::conf::*;

    #[test]
    fn try_deserialize() {
        // This is yucky since there is leading space but TOML thankfully doesn't mind it
        let conf_str = r#"
            instance_name = "WooChat"
            description = "The poggest place to chat"

            [oprish]
            url = "https://example.com"

            [oprish.rate_limits]
            get_instance_info = { reset_after = 10, limit = 2}

            [pandemonium]
            url = "wss://foo.bar"
            rate_limit = { reset_after = 20, limit = 10}

            [effis]
            file_size = "100MB"
            url = "https://example.com"

            [effis.rate_limits]
            attachments = { reset_after = 600, limit = 20, file_size_limit = "500MB"}

            [email]
            relay = "smtp.foo.com"
            name = "Fenni"
            address = "fenni@fenrir.den"
            "#;

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf {
            instance_name: "WooChat".to_string(),
            description: Some("The poggest place to chat".to_string()),
            oprish: OprishConf {
                rate_limits: OprishRateLimits {
                    get_instance_info: RateLimitConf {
                        reset_after: 10,
                        limit: 2,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            pandemonium: PandemoniumConf {
                rate_limit: RateLimitConf {
                    reset_after: 20,
                    limit: 10,
                },
                url: "wss://foo.bar".to_string(),
            },
            effis: EffisConf {
                file_size: 100_000_000,
                rate_limits: EffisRateLimits {
                    attachments: EffisRateLimitConf {
                        reset_after: 600,
                        limit: 20,
                        file_size_limit: 500_000_000,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            email: Some(Email {
                relay: "smtp.foo.com".to_string(),
                name: "Fenni".to_string(),
                address: "fenni@fenrir.den".to_string(),
                credentials: None,
                subjects: EmailSubjects::default(),
            }),
        };

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    #[test]
    fn default_conf() {
        let conf_str = "instance_name = \"TestInstance\"";

        let conf_str: Conf = toml::from_str(conf_str).unwrap();

        let conf = Conf::from_name("TestInstance".to_string()).unwrap();

        assert_eq!(format!("{:?}", conf_str), format!("{:?}", conf));
    }

    macro_rules! test_limit {
        ($conf:expr, $($limit:expr),+) => {
            $(
                $limit.limit = 0;
                assert!($conf.validate().is_err());
                $limit.limit = 1;
                assert!($conf.validate().is_ok());
            )+
        };
    }

    macro_rules! test_urls {
        ($conf:expr, $($service:ident),+) => {
            $(
                $conf.$service.url = "notavalidurl".to_string();
                assert!($conf.validate().is_err());
                $conf.$service.url = "http://avalid.url".to_string();
                assert!($conf.validate().is_ok());
            )+
        };
    }

    macro_rules! test_file_sizes {
        ($conf:expr, $($size:expr),+) => {
            $(
                $size = 0;
                assert!($conf.validate().is_err());
                $size = 1;
                assert!($conf.validate().is_ok());
            )+
        };
    }

    #[test]
    fn validate() {
        let mut conf = Conf::from_name("WooChat".to_string()).unwrap();

        assert!(conf.validate().is_ok());
        conf.instance_name = "".to_string();
        assert!(conf.validate().is_err());
        conf.instance_name = "h".repeat(33);
        assert!(conf.validate().is_err());
        conf.instance_name = "woo".to_string();
        assert!(conf.validate().is_ok());

        conf.description = Some("".to_string());
        assert!(conf.validate().is_err());
        conf.description = Some("h".repeat(2049));
        assert!(conf.validate().is_err());
        conf.description = Some("very cool".to_string());
        assert!(conf.validate().is_ok());

        conf.oprish.message_limit = 2;
        assert!(conf.validate().is_err());
        conf.oprish.message_limit = 1024;
        assert!(conf.validate().is_ok());

        test_limit!(
            conf,
            conf.pandemonium.rate_limit,
            conf.effis.rate_limits.assets,
            conf.effis.rate_limits.attachments,
            conf.effis.rate_limits.fetch_file,
            conf.oprish.rate_limits.get_instance_info,
            conf.oprish.rate_limits.create_message
        );

        test_urls!(conf, oprish, pandemonium, effis);

        test_file_sizes!(
            conf,
            conf.effis.file_size,
            conf.effis.attachment_file_size,
            conf.effis.rate_limits.assets.file_size_limit,
            conf.effis.rate_limits.attachments.file_size_limit
        );
    }
}
