use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

use super::{resource_type::ResourceType, resources::Resources};

#[derive(Debug)]
pub enum ParseBlueprintError {
    InvalidFormat,
    InvalidId(ParseIntError),
    InvalidResourceCount(ParseIntError),
}

impl Display for ParseBlueprintError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::InvalidId(error) => write!(f, "invalid ID: {}", error),
            Self::InvalidResourceCount(error) => write!(f, "invalid resource count: {}", error),
        }
    }
}

impl Error for ParseBlueprintError {}

pub struct Blueprint {
    pub id: usize,
    pub ore: Resources,
    pub clay: Resources,
    pub obsidian: Resources,
    pub geode: Resources,
}

impl Blueprint {
    pub fn cost(&self, resource_type: ResourceType) -> Resources {
        match resource_type {
            ResourceType::Ore => self.ore,
            ResourceType::Clay => self.clay,
            ResourceType::Obsidian => self.obsidian,
            ResourceType::Geode => self.geode,
        }
    }
}

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.strip_prefix("Blueprint ") {
            Some(s) => Ok(s),
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (id, s) = match s.split_once(": ") {
            Some((id, s)) => {
                let id = match id.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidId(err)),
                }?;
                Ok((id, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let s = match s.strip_prefix("Each ore robot costs ") {
            Some(s) => Ok(s),
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (ore_ore, s) = match s.split_once(" ore. ") {
            Some((ore_ore, s)) => {
                let ore_ore = match ore_ore.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidResourceCount(err)),
                }?;
                Ok((ore_ore, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let s = match s.strip_prefix("Each clay robot costs ") {
            Some(s) => Ok(s),
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (clay_ore, s) = match s.split_once(" ore. ") {
            Some((clay_ore, s)) => {
                let clay_ore = match clay_ore.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidResourceCount(err)),
                }?;
                Ok((clay_ore, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let s = match s.strip_prefix("Each obsidian robot costs ") {
            Some(s) => Ok(s),
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (obsidian_ore, s) = match s.split_once(" ore and ") {
            Some((obsidian_ore, s)) => {
                let obsidian_ore = match obsidian_ore.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidResourceCount(err)),
                }?;
                Ok((obsidian_ore, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (obsidian_clay, s) = match s.split_once(" clay. ") {
            Some((obsidian_clay, s)) => {
                let obsidian_clay = match obsidian_clay.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidResourceCount(err)),
                }?;
                Ok((obsidian_clay, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let s = match s.strip_prefix("Each geode robot costs ") {
            Some(s) => Ok(s),
            None => Err(Self::Err::InvalidFormat),
        }?;

        let (geode_ore, s) = match s.split_once(" ore and ") {
            Some((geode_ore, s)) => {
                let geode_ore = match geode_ore.parse::<usize>() {
                    Ok(id) => Ok(id),
                    Err(err) => Err(Self::Err::InvalidResourceCount(err)),
                }?;
                Ok((geode_ore, s))
            }
            None => Err(Self::Err::InvalidFormat),
        }?;

        let geode_obsidian = match s.strip_suffix(" obsidian.") {
            Some(geode_obsidian) => match geode_obsidian.parse::<usize>() {
                Ok(geode_obsidian) => Ok(geode_obsidian),
                Err(err) => Err(Self::Err::InvalidResourceCount(err)),
            },
            None => Err(Self::Err::InvalidFormat),
        }?;

        Ok(Blueprint {
            id,
            ore: Resources::new(ore_ore, 0, 0, 0),
            clay: Resources::new(clay_ore, 0, 0, 0),
            obsidian: Resources::new(obsidian_ore, obsidian_clay, 0, 0),
            geode: Resources::new(geode_ore, 0, geode_obsidian, 0),
        })
    }
}
