#[derive(Clone, Copy)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl ResourceType {
    pub fn all() -> Vec<ResourceType> {
        vec![
            ResourceType::Geode,
            ResourceType::Obsidian,
            ResourceType::Clay,
            ResourceType::Ore,
        ]
    }

    pub fn all_options() -> Vec<Option<ResourceType>> {
        vec![
            Some(ResourceType::Geode),
            Some(ResourceType::Obsidian),
            Some(ResourceType::Clay),
            Some(ResourceType::Ore),
            None,
        ]
    }
}
