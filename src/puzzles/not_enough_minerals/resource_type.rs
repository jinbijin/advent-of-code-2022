#[derive(Clone, Copy)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl ResourceType {
    pub fn all_options() -> Vec<Option<ResourceType>> {
        vec![
            Some(ResourceType::Ore),
            Some(ResourceType::Clay),
            Some(ResourceType::Obsidian),
            Some(ResourceType::Geode),
            None,
        ]
    }
}
