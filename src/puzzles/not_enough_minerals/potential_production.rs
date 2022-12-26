use std::collections::{hash_map::Iter, HashMap};

use super::{potential_resources::PotentialResources, resources::Resources};

#[derive(Debug)]
pub struct PotentialProduction {
    production: HashMap<Resources, PotentialResources>,
}

impl PotentialProduction {
    pub fn new() -> Self {
        PotentialProduction {
            production: HashMap::new(),
        }
    }

    pub fn initial() -> Self {
        let mut production = HashMap::new();
        let mut resources = PotentialResources::new();
        resources.insert(Resources::new(0, 0, 0, 0));
        production.insert(Resources::new(1, 0, 0, 0), resources);
        PotentialProduction { production }
    }

    pub fn iter(&self) -> Iter<Resources, PotentialResources> {
        self.production.iter()
    }

    pub fn add_to<'a, I>(&'a mut self, production: Resources, resources: I)
    where
        I: Iterator<Item = Resources>,
    {
        if let Some(potential_resources) = self.production.get_mut(&production) {
            potential_resources.add(resources);
        } else {
            let mut potential_resources: PotentialResources = PotentialResources::new();
            potential_resources.add(resources);
            self.production.insert(production, potential_resources);
        }
    }

    pub fn max_geodes(&self) -> usize {
        match self.production.iter().map(|(_, r)| r.max_geodes()).max() {
            Some(max) => max,
            None => 0,
        }
    }
}
