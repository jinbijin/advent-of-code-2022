use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseCommandLineError {
    InvalidCommand(String),
    InvalidItem(String),
}

impl Display for ParseCommandLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCommand(input) => write!(f, "command '{}' is invalid", input),
            Self::InvalidItem(input) => write!(f, "item '{}' is invalid", input),
        }
    }
}

impl Debug for ParseCommandLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Error for ParseCommandLineError {}

pub enum ChangeDirectoryTarget {
    Into(String),
    Out,
    Root,
}

pub enum DirectoryItem {
    Directory { name: String },
    File { size: usize, name: String },
}

pub enum CommandLine {
    List,
    ChangeDirectory(ChangeDirectoryTarget),
    Item(DirectoryItem),
}

impl FromStr for CommandLine {
    type Err = ParseCommandLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("$ ") {
            if s == "$ ls" {
                Ok(Self::List)
            } else if s.starts_with("$ cd ") {
                let s = &s[5..];
                match s {
                    "/" => Ok(Self::ChangeDirectory(ChangeDirectoryTarget::Root)),
                    ".." => Ok(Self::ChangeDirectory(ChangeDirectoryTarget::Out)),
                    s => Ok(Self::ChangeDirectory(ChangeDirectoryTarget::Into(
                        s.to_string(),
                    ))),
                }
            } else {
                Err(Self::Err::InvalidCommand(s.to_string()))
            }
        } else {
            if let Some((kind, name)) = s.split_once(' ') {
                match kind {
                    "dir" => Ok(Self::Item(DirectoryItem::Directory {
                        name: name.to_string(),
                    })),
                    size => match size.parse::<usize>() {
                        Ok(size) => Ok(Self::Item(DirectoryItem::File {
                            size,
                            name: name.to_string(),
                        })),
                        Err(_) => Err(Self::Err::InvalidItem(s.to_string())),
                    },
                }
            } else {
                Err(Self::Err::InvalidItem(s.to_string()))
            }
        }
    }
}
