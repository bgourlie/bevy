mod sprite_discovery;

use crate::sprite_discovery::{read_animated_tiles, AnimatedSprite};
use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    sprite::TextureAtlasBuilder,
};
use std::{cmp::Ordering, collections::HashMap, convert::TryFrom};

fn main() {
    App::build()
        .init_resource::<SpriteAnimations>()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(load_atlas.system())
        .add_system(animate_sprite_system.system())
        .run();
}

#[derive(Default)]
pub struct SpriteAnimations {
    frames: HashMap<String, HashMap<String, Vec<u8>>>,
    handle_ids: Vec<HandleId>,
    atlas_loaded: bool,
}

fn animate_sprite_system(mut query: Query<(&mut Timer, &mut TextureAtlasSprite)>) {
    for (timer, mut sprite) in &mut query.iter() {
        if timer.finished {
            sprite.index = (sprite.index + 1) % 4; //((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(mut sprite_animations: ResMut<SpriteAnimations>, asset_server: Res<AssetServer>) {
    let (frames, handle_ids) = {
        let mut animations: HashMap<String, HashMap<String, Vec<u8>>> = HashMap::default();
        let mut handle_ids = Vec::default();
        let mut tiles =
            read_animated_tiles("examples/brian/frames").collect::<Vec<AnimatedSprite>>();

        tiles.sort_by(|a, b| {
            let name_cmp = a.tile_name.cmp(&b.tile_name);
            if name_cmp == Ordering::Equal {
                let anim_cmp = a.animation_name.cmp(&b.animation_name);
                if anim_cmp == Ordering::Equal {
                    a.frame_number.cmp(&b.frame_number)
                } else {
                    anim_cmp
                }
            } else {
                name_cmp
            }
        });

        for tile in tiles {
            println!("{}", tile.file_path);
            handle_ids.push(asset_server.load_untyped(tile.file_path).unwrap());
            animations
                .entry(tile.tile_name.clone())
                .or_default()
                .entry(tile.animation_name.clone())
                .or_default()
                .push(u8::try_from(handle_ids.len() - 1).unwrap());
        }

        (animations, handle_ids)
    };

    sprite_animations.frames = frames;
    sprite_animations.handle_ids = handle_ids;
}

fn load_atlas(
    mut commands: Commands,
    mut sprite_animations: ResMut<SpriteAnimations>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if sprite_animations.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&sprite_animations.handle_ids)
    {
        for handle_id in &sprite_animations.handle_ids {
            let handle = Handle::from_id(*handle_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle,
            scale: Scale(6.0),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));

    sprite_animations.atlas_loaded = true;
}
