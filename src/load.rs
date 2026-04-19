use std::sync::Arc;

use futures::future::join_all;
use legion::{Resources, World, query::*};
use macroquad::texture::{Texture2D, load_texture};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{
    comps::{Body, Sprite, Spritesheet, TileMap, Transform, WeaponHolder},
    resources::{Textures, physics::PhysicsContext},
};

pub async fn load(world: &mut World, resources: &mut Resources) {
    let textures = &mut resources.get_mut::<Textures>().unwrap().0;

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

    let mut query = <&TileMap>::query();
    for tilemap in query.iter(world) {
        if !textures.contains_key(&tilemap.tileset_path) {
            img_paths.push(tilemap.tileset_path.clone());
        }
    }

    let mut query = <&WeaponHolder>::query();
    for holder in query.iter(world) {
        if let Some(weapon) = &holder.weapon {
            if !textures.contains_key(&weapon.image_path()) {
                img_paths.push(weapon.image_path());
            }
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
        #[cfg(debug_assertions)]
        println!("Imagem {} carregada.", path);
    }
}

pub fn physics_load(world: &mut World, resources: &mut Resources) {
    let mut ctx_wrapper = resources
        .get_mut::<PhysicsContext>()
        .expect("load.rs: Recursos de física não inicializados corretamente.");
    let ctx = &mut *ctx_wrapper;

    let rigid_bodies = &mut ctx.bodies;
    let colliders = &mut ctx.colliders;

    let mut query = <(&mut Transform, &mut Body)>::query();
    for (mut transform, body) in query.iter_mut(world) {
        body.load(
            crate::comps::BodyType::Rect,
            &mut transform,
            rigid_bodies,
            colliders,
        );
        #[cfg(debug_assertions)]
        println!(
            "Novo body (dinâmico:{}) criado em {}: {:?}",
            body.is_dynamic, transform.position, body.body_handle
        );
    }
}
