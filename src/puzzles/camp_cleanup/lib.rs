use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseCampAssignmentError {
    InvalidNumberOfRanges(usize),
    InvalidNumberOfBounds(usize, usize),
    ParseIntError(usize, usize),
}

impl Display for ParseCampAssignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumberOfRanges(count) => {
                write!(f, "contains {} ranges, instead of the expected 2", count)
            }
            Self::InvalidNumberOfBounds(index, count) => write!(
                f,
                "range {} contains {} bounds, instead of the expected 2",
                index + 1,
                count
            ),
            Self::ParseIntError(range_index, bound_index) => write!(
                f,
                "bound {} in range {} contains an invalid value",
                bound_index + 1,
                range_index + 1
            ),
        }
    }
}

impl Debug for ParseCampAssignmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseCampAssignmentError {}

#[derive(Clone, Copy)]
pub struct CampSection {
    pub start: usize,
    pub end: usize, // Note: end is inclusive
}

pub struct CampAssignment([CampSection; 2]);

impl FromStr for CampAssignment {
    type Err = ParseCampAssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split(',').collect::<Vec<&str>>();
        if sections.len() != 2 {
            return Err(ParseCampAssignmentError::InvalidNumberOfRanges(
                sections.len(),
            ));
        }

        let sections = sections
            .iter()
            .enumerate()
            .map(|(section_index, section)| {
                let bounds = section.split('-').collect::<Vec<&str>>();
                if bounds.len() != 2 {
                    return Err(ParseCampAssignmentError::InvalidNumberOfBounds(
                        section_index,
                        bounds.len(),
                    ));
                }
                let bounds = bounds
                    .iter()
                    .enumerate()
                    .map(|(i, b)| {
                        b.parse::<usize>()
                            .map_err(|_| ParseCampAssignmentError::ParseIntError(section_index, i))
                    })
                    .collect::<Result<Vec<usize>, ParseCampAssignmentError>>()?;
                Ok(CampSection {
                    start: bounds[0],
                    end: bounds[1],
                })
            })
            .collect::<Result<Vec<CampSection>, ParseCampAssignmentError>>()?;

        Ok(CampAssignment([sections[0], sections[1]]))
    }
}

impl CampAssignment {
    pub fn one_is_contained_in_other(&self) -> bool {
        let CampAssignment(sections) = self;
        (sections[0].start >= sections[1].start && sections[0].end <= sections[1].end)
            || (sections[0].start <= sections[1].start && sections[0].end >= sections[1].end)
    }

    pub fn overlaps(&self) -> bool {
        let CampAssignment(sections) = self;
        sections[0].start <= sections[1].end && sections[0].end >= sections[1].start
    }
}
