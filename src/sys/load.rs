use std::{collections::HashMap, sync::Arc};

use crate::comps::*;
use legion::system;
use sdl2::{
    image::LoadTexture as _,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

#[system(for_each)]
pub fn load(
    sprite: &Sprite,
    #[resource] textures: &HashMap<String, Arc<Texture<'_>>>,
    #[resource] texture_creator: &TextureCreator<WindowContext>,
) {
    textures
        .get(sprite.image_path.as_str())
        .get_or_insert(&Arc::new(
            texture_creator
                .load_texture(sprite.image_path.as_str())
                .unwrap(),
        ));
}

