use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
pub enum ParseTravelInstructionSequenceError {
    InvalidInstruction,
}

impl From<ParseIntError> for ParseTravelInstructionSequenceError {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidInstruction
    }
}

impl Display for ParseTravelInstructionSequenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction => write!(f, "invalid instruction"),
        }
    }
}

impl Error for ParseTravelInstructionSequenceError {}

pub enum TravelInstruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

pub struct TravelInstructionSequence(pub Vec<TravelInstruction>);

impl FromStr for TravelInstructionSequence {
    type Err = ParseTravelInstructionSequenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut travel_instructions: Vec<TravelInstruction> = Vec::new();

        let turn_instructions = ['L', 'R'];
        for part in s.split_inclusive(turn_instructions) {
            if part.ends_with(turn_instructions) {
                let (distance, turn) = part.split_at(part.len() - 1);
                let distance = distance.parse::<usize>()?;
                travel_instructions.push(TravelInstruction::Move(distance));

                let turn = match turn {
                    "L" => Ok(TravelInstruction::TurnLeft),
                    "R" => Ok(TravelInstruction::TurnRight),
                    _ => Err(Self::Err::InvalidInstruction),
                }?;
                travel_instructions.push(turn);
            } else if part.len() > 0 {
                let distance = part.parse::<usize>()?;
                travel_instructions.push(TravelInstruction::Move(distance));
            }
        }

        Ok(Self(travel_instructions))
    }
}
