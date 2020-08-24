use bevy::prelude::*;
use bevy::asset::{LoadState, HandleId};
use bevy::sprite::TextureAtlasBuilder;

fn main() {
    App::build()
        .init_resource::<RpgSpriteHandles>()
        .add_default_plugins()
        .add_startup_system(setup.system())
        // .add_system(animate_sprite_system.system())
        .add_system(load_atlas.system())
        .run();
}

#[derive(Default)]
pub struct RpgSpriteHandles {
    handles: Vec<HandleId>,
    atlas_loaded: bool,
}

fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in &mut query.iter() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server
        .load_asset_folder("examples/brian/frames")
        .unwrap();
    // let texture_handle = asset_server
    //     .load_sync(
    //         &mut textures,
    //         "assets/textures/rpg/chars/gabe/gabe-idle-run.png",
    //     )
    //     .unwrap();
    // let texture = textures.get(&texture_handle).unwrap();
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 7, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // commands
    //     .spawn(Camera2dComponents::default())
    //     .spawn(SpriteSheetComponents {
    //         texture_atlas: texture_atlas_handle,
    //         scale: Scale(6.0),
    //         ..Default::default()
    //     })
    //     .with(Timer::from_seconds(0.1, true));
}
fn load_atlas(
    mut commands: Commands,
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if rpg_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let Some(LoadState::Loaded(_)) =
    asset_server.get_group_load_state(&rpg_sprite_handles.handles)
    {
        for texture_id in rpg_sprite_handles.handles.iter() {
            let handle = Handle::from_id(*texture_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let texture_atlas_texture = texture_atlas.texture;

        commands
            .spawn(Camera2dComponents::default())
            .spawn(SpriteComponents {
                scale: Scale(4.0),
                material: materials.add(texture_atlas_texture.into()),
                translation: Vec3::new(-300.0, 0., 0.0).into(),
                ..Default::default()
            });

        rpg_sprite_handles.atlas_loaded = true;
    }
}
