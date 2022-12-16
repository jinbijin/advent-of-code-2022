use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    str::FromStr,
};

pub enum ParseValveError {
    InvalidFormat,
    InvalidFlowRate { input: String },
}

impl Display for ParseValveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidFlowRate { input } => write!(f, "invalid flow rate '{}'", input),
        }
    }
}

impl Debug for ParseValveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl Error for ParseValveError {}

pub struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub connected_to: Vec<String>,
}

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (valve, connected_to) = match s.split_once("; ") {
            None => Err(Self::Err::InvalidFormat),
            Some(pair) => Ok(pair),
        }?;

        let valve_prefix = "Valve ";
        let valve = if valve.starts_with(valve_prefix) {
            Ok(&valve[(valve_prefix.len())..])
        } else {
            Err(Self::Err::InvalidFormat)
        }?;

        let (name, flow_rate) = match valve.split_once(" has flow rate=") {
            None => Err(Self::Err::InvalidFormat),
            Some(pair) => Ok(pair),
        }?;
        let name = name.to_string();

        let flow_rate =
            flow_rate
                .parse::<usize>()
                .map_err(|_| ParseValveError::InvalidFlowRate {
                    input: flow_rate.to_string(),
                })?;

        let connected_to_prefix_plural = "tunnels lead to valves ";
        let connected_to_prefix_singular = "tunnel leads to valve ";
        let connected_to = if connected_to.starts_with(connected_to_prefix_singular) {
            Ok(&connected_to[(connected_to_prefix_singular.len())..])
        } else if connected_to.starts_with(connected_to_prefix_plural) {
            Ok(&connected_to[(connected_to_prefix_plural.len())..])
        } else {
            Err(Self::Err::InvalidFormat)
        }?;

        let connected_to = connected_to
            .split(", ")
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        Ok(Valve {
            name,
            flow_rate,
            connected_to,
        })
    }
}
