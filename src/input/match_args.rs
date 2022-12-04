use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum MatchArgsError<T> {
    ParseError(T),
    EndOfArgsError,
}

impl<T> Display for MatchArgsError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(err) => write!(f, "Could not parse argument correctly: {}", err),
            Self::EndOfArgsError => write!(f, "Missing argument"),
        }
    }
}

impl<T> Debug for MatchArgsError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl<T> Error for MatchArgsError<T>
where
    T: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseError(err) => Some(err),
            Self::EndOfArgsError => None,
        }
    }
}

pub trait MatchArgs: Sized {
    type Err;

    fn match_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Self::Err>;
}

impl<T> MatchArgs for T
where
    T: FromStr,
{
    type Err = MatchArgsError<<T as FromStr>::Err>;

    fn match_args(args: &mut impl Iterator<Item = String>) -> Result<Self, Self::Err> {
        match args.next() {
            Some(value) => value.parse::<T>().map_err(|err| Self::Err::ParseError(err)),
            None => Err(Self::Err::EndOfArgsError),
        }
    }
}

pub trait MatchArgsIterator {
    fn next_match<T>(&mut self) -> Result<T, T::Err>
    where
        T: MatchArgs;
}

impl<TIter> MatchArgsIterator for TIter
where
    TIter: Iterator<Item = String>,
{
    fn next_match<T>(&mut self) -> Result<T, T::Err>
    where
        T: MatchArgs,
    {
        T::match_args(self)
    }
}
