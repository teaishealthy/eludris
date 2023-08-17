#![macro_use]
use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Shared fields between all error response variants.
#[autodoc(category = "Errors", hidden = true)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedErrorData {
    /// The HTTP status of the error.
    pub status: u16,
    /// A brief explanation of the error.
    pub message: String,
}

/// All the possible error responses that are returned from Eludris HTTP microservices.
#[autodoc(category = "Errors")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorResponse {
    /// The error when the client is missing authorization. This error often occurs when the user
    /// doesn't pass in the required authentication or passes in invalid credentials.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "UNAUTHORIZED",
    ///   "status": 401,
    ///   "message": "The user is missing authentication or the passed credentials are invalid"
    /// }
    /// ```
    Unauthorized {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client *has* been succesfully authorized but does not have the required
    /// permissions to execute an action.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "FORBIDDEN",
    ///   "status": 403,
    ///   "message": "The user is missing the requried permissions to execute this action",
    /// }
    /// ```
    Forbidden {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client requests a resource that does not exist.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "NOT_FOUND",
    ///   "status": 404,
    ///   "message": "The requested resource could not be found"
    /// }
    /// ```
    NotFound {
        #[serde(flatten)]
        shared: SharedErrorData,
    },
    /// The error when a client's request causes a conflict, usually when they're trying to create
    /// something that already exists.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "CONFLICT",
    ///   "status": 409,
    ///   "message": "The request couldn't be completed due to conflicting with other data on the server",
    ///   "item": "username",
    /// }
    /// ```
    Conflict {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The conflicting item.
        item: String,
    },
    /// The error when a server isn't able to reduce a response even though the client's request
    /// isn't explicitly wrong. This usually happens when an instance isn't configured to provide a
    /// response.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "MISDIRECTED",
    ///   "status": 421,
    ///   "message": "Misdirected request",
    ///   "info": "The instance isn't configured to deal with unbased individuals"
    /// }
    /// ```
    Misdirected {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// Extra information about what went wrong.
        info: String,
    },
    /// The error when a request a client sends is incorrect and fails validation.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "VALIDATION",
    ///   "status": 422,
    ///   "message": "Invalid request",
    ///   "value_name": "author",
    ///   "info": "author name is a bit too cringe"
    /// }
    /// ```
    Validation {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The name of the value that failed validation.
        value_name: String,
        /// Extra information about what went wrong.
        info: String,
    },
    /// The error when a client is rate limited.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "RATE_LIMITED",
    ///   "status": 429,
    ///   "message": "You have been rate limited",
    ///   "retry_after": 1234
    /// }
    /// ```
    RateLimited {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// The amount of milliseconds you're still rate limited for.
        retry_after: u64,
    },
    /// The error when the server fails to process a request.
    ///
    /// Getting this error means that it's the server's fault and not the client that the request
    /// failed.
    ///
    /// -----
    ///
    /// ### Example
    ///
    /// ```json
    /// {
    ///   "type": "SERVER",
    ///   "status": 500,
    ///   "message": "Server encountered an unexpected error",
    ///   "info": "Server got stabbed 28 times"
    /// }
    /// ```
    Server {
        #[serde(flatten)]
        shared: SharedErrorData,
        /// Extra information about what went wrong.
        info: String,
    },
}

/// Magic macro that handles instantiating [`ErrorResponse`] variants.
///
/// It also handles wrapping and returning them when a rate limiter is passed as the first argument.
#[cfg(feature = "logic")]
#[macro_export]
macro_rules! error {
    ($rate_limiter:expr, $error:ident, $($val:expr),+) => {
        return $rate_limiter.wrap_response(Err(error!($error, $($val),+)));
    };
    (UNAUTHORIZED) => {
        ErrorResponse::Unauthorized {
            shared: $crate::models::SharedErrorData {
                status: 401,
                message: "The user is missing authentication or the passed credentials are invalid".to_string(),
            }
        }
    };
    (FORBIDDEN) => {
        ErrorResponse::Forbidden {
            shared: $crate::models::SharedErrorData {
                status: 403,
                message: "The user is missing the requried permissions to execute this action".to_string(),
            }
        }
    };
    (NOT_FOUND) => {
        ErrorResponse::NotFound {
            shared: $crate::models::SharedErrorData {
                status: 404,
                message: "The requested resource could not be found".to_string(),
            },
        }
    };
    (CONFLICT, $item:expr) => {
        ErrorResponse::Conflict {
            shared: $crate::models::SharedErrorData {
                status: 409,
                message: "The request couldn't be completed due to conflicting with other data on the server".to_string(),
            },
            item: $item.to_string(),
        }
    };
    (MISDIRECTED, $info:expr) => {
        ErrorResponse::Misdirected {
            shared: $crate::models::SharedErrorData {
                status: 421,
                message: "Misdirected request".to_string(),
            },
            info: $info.to_string(),
        }
    };
    (VALIDATION, $value_name:expr, $info:expr) => {
        ErrorResponse::Validation {
            shared: $crate::models::SharedErrorData {
                status: 422,
                message: "Invalid request".to_string(),
            },
            value_name: $value_name.to_string(),
            info: $info.to_string(),
        }
    };
    (RATE_LIMITED, $retry_after:expr) => {
        ErrorResponse::RateLimited {
            shared: $crate::models::SharedErrorData {
                status: 429,
                message: "You have been rate limited".to_string(),
            },
            retry_after: $retry_after,
        }
    };
    (SERVER, $info:expr) => {
        ErrorResponse::Server {
            shared: $crate::models::SharedErrorData {
                status: 500,
                message: "Server encountered an unexpected error".to_string(),
            },
            info: $info.to_string(),
        }
    }
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponse::Unauthorized { shared, .. } => write!(f, "{}", shared.message),
            ErrorResponse::Forbidden { shared, .. } => write!(f, "{}", shared.message),
            ErrorResponse::NotFound { shared, .. } => write!(f, "{}", shared.message),
            ErrorResponse::Conflict { shared, item } => write!(f, "{}: {}", shared.message, item),
            ErrorResponse::Misdirected { shared, info } => {
                write!(f, "{}: {}", shared.message, info)
            }
            ErrorResponse::Validation {
                info, value_name, ..
            } => write!(f, "Invalid {}: {}", value_name, info),
            ErrorResponse::RateLimited { shared, .. } => write!(f, "{}", shared.message),
            ErrorResponse::Server { shared, info } => write!(f, "{}: {}", shared.message, info),
        }
    }
}

#[cfg(feature = "logic")]
impl ErrorResponse {
    pub fn shared(&self) -> &SharedErrorData {
        match self {
            ErrorResponse::Unauthorized { shared, .. } => shared,
            ErrorResponse::Forbidden { shared, .. } => shared,
            ErrorResponse::NotFound { shared, .. } => shared,
            ErrorResponse::Conflict { shared, .. } => shared,
            ErrorResponse::Misdirected { shared, .. } => shared,
            ErrorResponse::Validation { shared, .. } => shared,
            ErrorResponse::RateLimited { shared, .. } => shared,
            ErrorResponse::Server { shared, .. } => shared,
        }
    }
}

#[cfg(feature = "logic")]
#[cfg(test)]
mod tests {
    use crate::models::{ErrorResponse, SharedErrorData};

    #[test]
    fn unauthorized_error() {
        assert_eq!(
            error!(UNAUTHORIZED),
            ErrorResponse::Unauthorized {
                shared: SharedErrorData {
                    status: 401,
                    message:
                        "The user is missing authentication or the passed credentials are invalid"
                            .to_string(),
                },
            }
        );
    }

    #[test]
    fn forbidden_error() {
        assert_eq!(
            error!(FORBIDDEN),
            ErrorResponse::Forbidden {
                shared: SharedErrorData {
                    status: 403,
                    message: "The user is missing the requried permissions to execute this action"
                        .to_string(),
                },
            }
        );
    }

    #[test]
    fn not_found_error() {
        assert_eq!(
            error!(NOT_FOUND),
            ErrorResponse::NotFound {
                shared: SharedErrorData {
                    status: 404,
                    message: "The requested resource could not be found".to_string(),
                },
            }
        );
    }

    #[test]
    fn conflict_error() {
        assert_eq!(
            error!(CONFLICT, "username"),
            ErrorResponse::Conflict {
                shared: SharedErrorData {
                    status: 409,
                    message: "The request couldn't be completed due to conflicting with other data on the server".to_string(),
                },
                item: "username".to_string(),
            }
        );
    }

    #[test]
    fn validation_error() {
        assert_eq!(
            error!(
                VALIDATION,
                "beans", "You have supplied an invalid amount of beans"
            ),
            ErrorResponse::Validation {
                shared: SharedErrorData {
                    status: 422,
                    message: "Invalid request".to_string(),
                },
                value_name: "beans".to_string(),
                info: "You have supplied an invalid amount of beans".to_string()
            }
        );
    }

    #[test]
    fn rate_limited_error() {
        assert_eq!(
            error!(RATE_LIMITED, 1234),
            ErrorResponse::RateLimited {
                shared: SharedErrorData {
                    status: 429,
                    message: "You have been rate limited".to_string(),
                },
                retry_after: 1234,
            }
        );
    }

    #[test]
    fn server_error() {
        assert_eq!(
            error!(SERVER, "Server ran out of Day Do Doh Don De Doh"),
            ErrorResponse::Server {
                shared: SharedErrorData {
                    status: 500,
                    message: "Server encountered an unexpected error".to_string(),
                },
                info: "Server ran out of Day Do Doh Don De Doh".to_string(),
            }
        );
    }
}
