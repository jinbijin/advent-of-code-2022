use super::{blueprint::Blueprint, resource_type::ResourceType, resources::Resources};

pub struct CollectionState<'a> {
    blueprint: &'a Blueprint,
    start_of_minute: usize,
    total_minutes: usize,
    stored: Resources,
    production: Resources,
    options: Box<dyn Iterator<Item = CollectionState<'a>> + 'a>,
}

impl<'a> Iterator for CollectionState<'a> {
    type Item = CollectionState<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.options.next()
    }
}

impl<'a> CollectionState<'a> {
    pub fn new(
        blueprint: &'a Blueprint,
        start_of_minute: usize,
        stored: Resources,
        production: Resources,
        total_minutes: usize,
    ) -> CollectionState<'a> {
        let options = Box::new(ResourceType::all_options().into_iter().filter_map(
            move |resource_type| match resource_type {
                Some(resource_type) => {
                    // Wait for resource to be affordable, then buy it.
                    match stored.time_until_affordable(production, blueprint.cost(resource_type)) {
                        Some(time_until_affordable) => {
                            let time_elapsed = time_until_affordable + 1;

                            if start_of_minute + time_elapsed > total_minutes {
                                None
                            } else {
                                Some(CollectionState::new(
                                    blueprint,
                                    start_of_minute + time_elapsed,
                                    stored + (production * time_elapsed)
                                        - blueprint.cost(resource_type),
                                    production + resource_type.into(),
                                    total_minutes,
                                ))
                            }
                        }
                        None => None,
                    }
                }
                None => {
                    // Run until end
                    let time_elapsed = total_minutes + 1 - start_of_minute;

                    Some(CollectionState::new(
                        blueprint,
                        total_minutes + 1,
                        stored + (production * time_elapsed),
                        production,
                        total_minutes,
                    ))
                }
            },
        ));
        CollectionState {
            blueprint,
            start_of_minute,
            total_minutes,
            stored,
            production,
            options,
        }
    }

    pub fn start_of_minute(&self) -> usize {
        self.start_of_minute
    }

    pub fn geode_count(&self) -> usize {
        self.stored.geode_count()
    }

    pub fn potential_geode_count(&self) -> usize {
        let time_remaining = self.total_minutes + 1 - self.start_of_minute;

        if time_remaining == 0 {
            self.stored.geode_count()
        } else {
            self.stored.geode_count()
                + ((2 * self.production.geode_count() + time_remaining - 1) * time_remaining / 2)
        }
    }
}
