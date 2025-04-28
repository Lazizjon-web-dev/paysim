pub enum PaysimError {
    UserNotFound,
    InsufficientFunds,
    AuthFailed,
    IoError(std::io::Error),
    SerderError(serde_json::Error),
    argon2Error(argon2::Error),
    TransactionFailed(String),
    InvalidInput(String),
    DatabaseError(String),
}