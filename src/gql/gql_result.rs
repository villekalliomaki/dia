use async_graphql::Error as GraphQLError;
use redis::RedisError;
use sqlx::Error as SqlxError;
use thiserror::Error;
use validator::ValidationErrors;
use std::str::Utf8Error;

/// The type GraphQL handler functions returns.
/// The `?`-syntax converts any supported error into `E`.
pub type R<T> = std::result::Result<T, E>;

/// E is for formatting common error types into more readable forms.
/// For every supported type a `From<T>` implementation is required.
#[derive(Error, Debug)]
pub enum E {
    #[error("UTF-8 error.")]
    Utf8(#[from] Utf8Error),
    #[error("Tokio task join error.")]
    TokioJoin(#[from] tokio::task::JoinError),
    // Temporary
    // Some queries might still return this poorly formatted type
    #[error("An error occurred: {}", .0)]
    Anyhow(#[from] anyhow::Error),
    #[error("GQL error.")]
    GraphQL(GraphQLError),
    #[error("Redis error: {:?}", .0.kind())]
    Redis(RedisError),
    #[error("Invalid input.")]
    InvalidInput,
    #[error("Not found.")]
    NotFound,
    #[error("{} not found.", .0)]
    ItemNotFound(String),
    #[error("A database error occurred.")]
    DatabaseError,
    #[error("An unknown error occurred.")]
    Unknown,
    #[error("{}", .0)]
    Message(String),
}

impl From<SqlxError> for E {
    fn from(error: SqlxError) -> E {
        match error {
            SqlxError::RowNotFound => E::NotFound,
            _ => E::Unknown,
        }
    }
}

impl From<String> for E {
    fn from(error: String) -> E {
        E::Message(error)
    }
}

impl From<GraphQLError> for E {
    fn from(error: GraphQLError) -> E {
        E::GraphQL(error)
    }
}

impl From<ValidationErrors> for E {
    fn from(error: ValidationErrors) -> E {
        let fields = error.field_errors();

        let mut messages: Vec<String> = vec![];

        // Iterate over every field and it's errors
        for (field, errors) in fields {
            // Collect formatted errors
            let mut formatted: Vec<String> = vec![];

            for error in errors {
                // If the error has a message, use it
                if let Some(message) = &error.message {
                    formatted.push(message.to_string());
                }
                // In case a message is not included, use the code
                else {
                    formatted.push(format!("a validation error with the code {}", error.code));
                }
            }

            // Format all the field's errors

            // No list for a single error
            if formatted.len() == 1 {
                messages.push(format!("Field {} {}.", field, formatted[0]));
            }
            // If the vec is longer format in a list
            else {
                let mut message_list: Vec<String> = vec![];

                for (i, msg) in formatted.iter().enumerate() {
                    if i + 1 == formatted.len() {
                        // If the message is the last one, end the sentence
                        message_list.push(format!("{}", msg));
                    } else {
                        // If it's not the last continue the list
                        message_list.push(format!("{},", msg));
                    }
                }

                messages.push(format!("Field {}: {}.", field, message_list.join(" ")));
            }
        }

        E::Message(messages.join(" "))
    }
}
