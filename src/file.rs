use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

struct FileErrorWithLine<E> {
    line: usize,
    error: E,
}

pub struct FileErrorCollection<E>(Vec<FileErrorWithLine<E>>);

impl<E> Display for FileErrorCollection<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Errors:")?;
        let FileErrorCollection(file_errors) = self;
        for file_error in file_errors {
            writeln!(f, "Line {}: {}", file_error.line + 1, file_error.error)?;
        }

        Ok(())
    }
}

impl<E> Debug for FileErrorCollection<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <FileErrorCollection<E> as Display>::fmt(self, f)
    }
}

impl<E> Error for FileErrorCollection<E> where E: Display {}

enum FileParseResult<T, E> {
    FileOk(Vec<T>),
    FileErr(FileErrorCollection<E>),
}

impl<T, E> FromIterator<Result<T, E>> for FileParseResult<T, E> {
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        let mut errors: Vec<FileErrorWithLine<E>> = Vec::new();
        let mut result: Vec<T> = Vec::new();
        for (line, item) in iter.into_iter().enumerate() {
            match item {
                Ok(value) => result.push(value),
                Err(error) => errors.push(FileErrorWithLine { line, error }),
            }
        }

        if errors.is_empty() {
            FileParseResult::FileOk(result)
        } else {
            FileParseResult::FileErr(FileErrorCollection(errors))
        }
    }
}

impl<T, E> From<FileParseResult<T, E>> for Result<Vec<T>, FileErrorCollection<E>> {
    fn from(value: FileParseResult<T, E>) -> Self {
        match value {
            FileParseResult::FileOk(result) => Ok(result),
            FileParseResult::FileErr(errors) => Err(errors),
        }
    }
}

pub fn parse_lines<T>(
    file_contents: String,
) -> Result<Vec<T>, FileErrorCollection<<T as FromStr>::Err>>
where
    T: FromStr,
{
    file_contents
        .lines()
        .map(|line| -> Result<T, <T as FromStr>::Err> { line.parse::<T>() })
        .collect::<FileParseResult<T, <T as FromStr>::Err>>()
        .into()
}

pub fn parse_optional_lines<T>(
    file_contents: String,
) -> Result<Vec<Option<T>>, FileErrorCollection<<T as FromStr>::Err>>
where
    T: FromStr,
{
    file_contents
        .lines()
        .map(|line| -> Result<Option<T>, <T as FromStr>::Err> {
            if line.is_empty() {
                Ok(None)
            } else {
                line.parse::<T>().map(|value| Some(value))
            }
        })
        .collect::<FileParseResult<Option<T>, <T as FromStr>::Err>>()
        .into()
}
