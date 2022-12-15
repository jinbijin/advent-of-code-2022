use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::{Chars, FromStr},
};

use crate::common::position::Position;

use super::convert::sections::{CustomSectionError, FromLines};

pub enum ParseGridError {
    EmptyGrid,
    InvalidDimensions,
    InvalidRowFormat,
    ParseError { description: String },
}

impl Display for ParseGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGrid => write!(f, "unexpected empty grid"),
            Self::InvalidDimensions => write!(f, "invalid width of grid"),
            Self::InvalidRowFormat => write!(f, "invalid row format"),
            Self::ParseError { description } => write!(f, "parse error: {}", description),
        }
    }
}

impl Debug for ParseGridError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseGridError {}

impl CustomSectionError for ParseGridError {}

pub struct Grid<const N: usize, const S: usize, T> {
    width: usize,
    height: usize,
    // Single vector containing the rows one after another
    contents: Vec<T>,
}

impl<const N: usize, const S: usize, T> Grid<N, S, T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_value(&self, position: Position<usize>) -> &T {
        &self.contents[position.y * self.width + position.x]
    }

    pub fn positions<'a>(&'a self) -> GridPositions<'a, N, S, T> {
        GridPositions {
            grid: self,
            current: Position { x: 0, y: 0 },
        }
    }
}

impl<const N: usize, const S: usize, T> FromLines for Grid<N, S, T>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = ParseGridError;

    fn from_lines(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let height = lines.len();

        if height == 0 {
            return Err(Self::Err::EmptyGrid);
        }

        let width = lines[0].chars().count();
        if S != 0 && width % N + S != N {
            return Err(Self::Err::InvalidDimensions);
        }

        let mut contents: Vec<T> = Vec::new();

        for line in lines {
            let chars = line.chars().collect::<Vec<char>>();
            if chars.len() != width {
                return Err(Self::Err::InvalidDimensions);
            }

            for chunk in line.char_chunks::<N, S>() {
                let chunk = chunk?;
                let item = chunk.parse::<T>().map_err(|err| Self::Err::ParseError {
                    description: err.to_string(),
                })?;
                contents.push(item);
            }
        }

        Ok(Grid {
            width,
            height,
            contents,
        })
    }
}

enum ParseCharChunksError {
    InvalidSeparator,
    InvalidEndOfLine,
}

impl From<ParseCharChunksError> for ParseGridError {
    fn from(_: ParseCharChunksError) -> Self {
        Self::InvalidRowFormat
    }
}

struct CharChunks<const N: usize, const S: usize, T>
where
    T: Iterator<Item = char>,
{
    iterator: T,
}

impl<const N: usize, const S: usize, T> Iterator for CharChunks<N, S, T>
where
    T: Iterator<Item = char>,
{
    type Item = Result<String, ParseCharChunksError>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.iterator.by_ref().take(N).collect::<Vec<char>>();
        if result.len() == N {
            let separator = self.iterator.by_ref().take(S).collect::<Vec<char>>();
            if (separator.len() == S || separator.len() == 0) && separator.iter().all(|c| *c == ' ')
            {
                Some(Ok(result.into_iter().collect::<String>()))
            } else {
                Some(Err(ParseCharChunksError::InvalidSeparator))
            }
        } else if result.len() == 0 {
            None
        } else {
            Some(Err(ParseCharChunksError::InvalidEndOfLine))
        }
    }
}

trait AsCharChunks<T>
where
    T: Iterator<Item = char>,
{
    fn char_chunks<const N: usize, const S: usize>(self) -> CharChunks<N, S, T>;
}

impl<'a> AsCharChunks<Chars<'a>> for &'a str {
    fn char_chunks<const N: usize, const S: usize>(self) -> CharChunks<N, S, Chars<'a>> {
        CharChunks {
            iterator: self.chars(),
        }
    }
}

pub struct GridPositions<'a, const N: usize, const S: usize, T> {
    grid: &'a Grid<N, S, T>,
    current: Position<usize>,
}

impl<'a, const N: usize, const S: usize, T> Iterator for GridPositions<'a, N, S, T> {
    type Item = Position<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;

        if result.y >= self.grid.height {
            return None;
        }

        self.current.x += 1;
        if self.current.x == self.grid.width {
            self.current.x = 0;
            self.current.y += 1;
        }

        Some(result)
    }
}
