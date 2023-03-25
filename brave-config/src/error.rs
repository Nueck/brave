use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum ConfigError {
    DbnameMissing,
    DbnameEmpty,
    UserNameMissing,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DbnameMissing => write!(f, "configuration property \"dbname\" not found"),
            Self::DbnameEmpty => write!(
                f,
                "configuration property \"dbname\" contains an empty string",
            ),
            Self::UserNameMissing => write!(f, "configuration property \"user\" not found",),
        }
    }
}
