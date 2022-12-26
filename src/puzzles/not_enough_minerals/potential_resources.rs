use std::collections::HashSet;

use super::resources::Resources;

#[derive(Debug)]
pub struct PotentialResources {
    resources: HashSet<Resources>,
}

impl PotentialResources {
    pub fn new() -> Self {
        PotentialResources {
            resources: HashSet::new(),
        }
    }

    pub fn insert(&mut self, value: Resources) {
        if !self.resources.iter().any(|r| value <= *r) {
            let to_delete = self
                .resources
                .iter()
                .filter_map(|r| if value >= *r { Some(*r) } else { None })
                .collect::<Vec<Resources>>();
            for item in to_delete {
                self.resources.remove(&item);
            }
            self.resources.insert(value);
        }
    }

    pub fn add<'a, I>(&'a mut self, resources: I)
    where
        I: Iterator<Item = Resources>,
    {
        for r in resources {
            self.insert(r);
        }
    }

    pub fn produce<'a>(
        &'a self,
        production: Resources,
    ) -> Box<dyn Iterator<Item = Resources> + 'a> {
        Box::new(self.resources.iter().map(move |r| *r + production))
    }

    pub fn buy_and_produce<'a>(
        &'a self,
        cost: Resources,
        production: Resources,
    ) -> Option<Box<dyn Iterator<Item = Resources> + 'a>> {
        if self.resources.iter().any(|r| r >= &cost) {
            Some(Box::new(self.resources.iter().filter_map(move |r| {
                if r >= &cost {
                    Some(*r - cost + production)
                } else {
                    None
                }
            })))
        } else {
            None
        }
    }

    pub fn max_geodes(&self) -> usize {
        match self.resources.iter().map(|r| r.geode).max() {
            Some(max) => max,
            None => 0,
        }
    }
}
