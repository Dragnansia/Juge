#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SerenityError(serenity::Error),
    VarError(std::env::VarError),
    NoFoundUser,
    NoFoundParameter(&'static str),
    NoFoundCommand,
    NoFoundGuildID,
    NoFoundGuild(u64),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<serenity::Error> for Error {
    fn from(err: serenity::Error) -> Self {
        Self::SerenityError(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::VarError(err)
    }
}
