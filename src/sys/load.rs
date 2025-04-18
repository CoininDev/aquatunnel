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
    #[resource] textures: &mut HashMap<String, Arc<Texture<'static>>>,
    #[resource] texture_creator: &TextureCreator<WindowContext>,
) {
    if !textures.contains_key(&sprite.image_path) {
        println!("cadastrando sprite '{}' em textures", sprite.image_path);

        let tex = texture_creator
            .load_texture(&sprite.image_path)
            .expect("Erro ao carregar textura");
        textures.insert(
            sprite.image_path.clone(),
            Arc::new(unsafe { std::mem::transmute(tex) }),
        );
    }
}

#[system(for_each)]
pub fn load_spritesheet(
    sprite: &Spritesheet,
    #[resource] textures: &mut HashMap<String, Arc<Texture<'static>>>,
    #[resource] texture_creator: &TextureCreator<WindowContext>,
) {
    if !textures.contains_key(&sprite.image_path) {
        println!(
            "cadastrando spritesheet '{}' em textures",
            sprite.image_path
        );
        let tex = texture_creator
            .load_texture(&sprite.image_path)
            .expect("Erro ao carregar textura");
        textures.insert(
            sprite.image_path.clone(),
            Arc::new(unsafe { std::mem::transmute(tex) }),
        );
    }
}
