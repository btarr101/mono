use bevy::ecs::resource::Resource;

mod entry;

#[derive(Resource)]
struct HistoryResource(Vec<entry::HistoryResourceEntry>);
