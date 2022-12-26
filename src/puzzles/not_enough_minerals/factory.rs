use super::{
    blueprint::Blueprint, potential_production::PotentialProduction, resource_type::ResourceType,
};

pub struct Factory<'a> {
    blueprint: &'a Blueprint,
    production: PotentialProduction,
}

impl<'a> Factory<'a> {
    pub fn run(&mut self, total_minutes: usize) -> usize {
        for _ in 0..total_minutes {
            self.step();
        }

        self.production.max_geodes()
    }

    fn step(&mut self) {
        self.production = self.next_production();
    }

    fn next_production(&self) -> PotentialProduction {
        let mut next_production: PotentialProduction = PotentialProduction::new();

        for (production, stored) in self.production.iter() {
            for resource_option in ResourceType::all_options() {
                match resource_option {
                    Some(resource_type) => match self.blueprint.max_needed(resource_type) {
                        Some(max_needed) => {
                            if production.get(resource_type) < max_needed {
                                if let Some(bought) = stored.buy_and_produce(
                                    self.blueprint.cost(resource_type),
                                    *production,
                                ) {
                                    next_production
                                        .add_to(*production + resource_type.into(), bought);
                                }
                            }
                        }
                        None => {
                            if let Some(bought) = stored
                                .buy_and_produce(self.blueprint.cost(resource_type), *production)
                            {
                                next_production.add_to(*production + resource_type.into(), bought);
                            }
                        }
                    },
                    None => next_production.add_to(*production, stored.produce(*production)),
                }
            }
        }

        next_production
    }
}

pub trait AsFactory {
    fn factory<'a>(&'a self) -> Factory<'a>;
}

impl AsFactory for Blueprint {
    fn factory<'a>(&'a self) -> Factory<'a> {
        Factory {
            blueprint: self,
            production: PotentialProduction::initial(),
        }
    }
}
