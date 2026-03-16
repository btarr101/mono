use bevy::ecs::{entity::Entity, resource::Resource};

#[derive(Resource)]
pub struct TerrainResource {
    width: usize,
    cells: Vec<TerrainCell>,
}

pub struct TerrainCell {
    elevated_slot: ElevatedTerrainCellSlot,
    ground_slot: GroundTerrainCellSlot,
}

impl Default for TerrainCell {
    fn default() -> Self {
        TerrainCell {
            elevated_slot: ElevatedTerrainCellSlot::Empty,
            ground_slot: GroundTerrainCellSlot::Ground,
        }
    }
}

pub enum ElevatedTerrainCellSlot {
    Empty,
    Some(Entity),
    Wall,
}

pub enum GroundTerrainCellSlot {
    Chasm,
    Ground,
}
