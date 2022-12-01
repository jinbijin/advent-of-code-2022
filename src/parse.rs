use std::str::FromStr;

pub fn parse_as_newline_separated<T>(
    contents: String,
) -> Result<Vec<Option<T>>, <T as FromStr>::Err>
where
    T: FromStr,
{
    contents
        .lines()
        .map(|line| -> Result<Option<T>, <T as FromStr>::Err> {
            if line.is_empty() {
                Ok(None)
            } else {
                let line = line.parse::<T>()?;
                Ok(Some(line))
            }
        })
        .collect::<Result<Vec<Option<T>>, <T as FromStr>::Err>>()
}
