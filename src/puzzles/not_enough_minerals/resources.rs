use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

use super::resource_type::{self, ResourceType};

#[derive(Debug, Clone, Copy)]
pub struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

fn time_until_affordable(current: usize, production: usize, cost: usize) -> Option<usize> {
    if current >= cost {
        Some(0)
    } else if current < cost && production == 0 {
        None
    } else {
        let div = (cost - current) / production;
        let has_rem = ((cost - current) % production) != 0;
        if has_rem {
            Some(div + 1)
        } else {
            Some(div)
        }
    }
}

impl Resources {
    pub fn new(ore: usize, clay: usize, obsidian: usize, geode: usize) -> Self {
        Resources {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    pub fn time_until_affordable(&self, production: Resources, cost: Resources) -> Option<usize> {
        let times_until_affordable = ResourceType::all()
            .into_iter()
            .filter_map(|resource_type| {
                time_until_affordable(
                    self.get(resource_type),
                    production.get(resource_type),
                    cost.get(resource_type),
                )
            })
            .collect::<Vec<usize>>();
        if times_until_affordable.len() < 4 {
            None
        } else {
            times_until_affordable.into_iter().max()
        }
    }

    pub fn geode_count(&self) -> usize {
        self.geode
    }

    pub fn get(&self, resource_type: ResourceType) -> usize {
        match resource_type {
            ResourceType::Ore => self.ore,
            ResourceType::Clay => self.clay,
            ResourceType::Obsidian => self.obsidian,
            ResourceType::Geode => self.geode,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
            && self.geode <= other.geode
        {
            if self.ore == other.ore
                && self.clay == other.clay
                && self.obsidian == other.obsidian
                && self.geode == other.geode
            {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        } else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
        {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl PartialEq for Resources {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}

impl From<ResourceType> for Resources {
    fn from(resource_type: ResourceType) -> Self {
        match resource_type {
            ResourceType::Ore => Resources {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            ResourceType::Clay => Resources {
                ore: 0,
                clay: 1,
                obsidian: 0,
                geode: 0,
            },
            ResourceType::Obsidian => Resources {
                ore: 0,
                clay: 0,
                obsidian: 1,
                geode: 0,
            },
            ResourceType::Geode => Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 1,
            },
        }
    }
}

impl Add<Resources> for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        Self::Output {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub<Resources> for Resources {
    type Output = Resources;

    fn sub(self, rhs: Resources) -> Self::Output {
        Self::Output {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Resources;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::Output {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}
