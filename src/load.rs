use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;
use legion::{Resources, World, query::*};
use macroquad::texture::{Texture2D, load_texture};

use crate::comps::{Sprite, Spritesheet};

pub async fn load(world: &mut World, resources: &mut Resources) {
    let mut textures = resources
        .get_mut::<HashMap<String, Arc<Texture2D>>>()
        .unwrap();

    let mut img_paths: Vec<String> = Vec::new();
    let mut query = <&Sprite>::query();
    for spr in query.iter(world) {
        if !textures.contains_key(&spr.image_path) {
            img_paths.push(spr.image_path.clone());
        }
    }
    let mut query = <&Spritesheet>::query();
    for spr in query.iter(world) {
        if !textures.contains_key(&spr.image_path) {
            img_paths.push(spr.image_path.clone());
        }
    }

    let futures = img_paths.iter().map(|path| async move {
        let tex = load_texture(path).await.unwrap();
        (path.clone(), Arc::new(tex))
    });

    let loaded_texs: Vec<(String, Arc<Texture2D>)> = join_all(futures).await;

    for (path, tex) in loaded_texs {
        tex.set_filter(macroquad::texture::FilterMode::Nearest);
        textures.insert(path.clone(), tex);
        println!("imagem {} carregada.", path);
    }
}
