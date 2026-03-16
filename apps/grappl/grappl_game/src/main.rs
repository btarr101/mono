use std::path::Path;

use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    asset::{AssetPath, AssetServer},
    camera::Camera2d,
    ecs::system::{Commands, Res},
    image::{Image, ImageArrayLayout, ImageLoaderSettings, ImagePlugin},
    math::UVec2,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use rand::RngExt;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AsepriteUltraPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: This expects like a single column or row??? Seems inefficient
    let tileset = asset_server.load_with_settings::<Image, ImageLoaderSettings>(
        AssetPath::from_path(Path::new("tiles_test.png")),
        |settings| {
            settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 6 });
        },
    );

    let chunk_size = UVec2::splat(64);
    let mut rng = rand::rng();

    let tile_data = (0..chunk_size.element_product())
        .map(|_| rng.random_range(0..36))
        .map(|index| Some(TileData::from_tileset_index(index)))
        .collect::<Vec<_>>();

    commands.spawn((
        TilemapChunk {
            chunk_size: UVec2::splat(64),
            tile_display_size: UVec2::splat(32),
            tileset,
            ..Default::default()
        },
        TilemapChunkTileData(tile_data),
    ));

    commands.spawn(Camera2d);
}
