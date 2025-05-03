use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;
use legion::{Resources, World, query::*};
use macroquad::texture::{Texture2D, load_texture};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};

use crate::{comps::{Body, Sprite, Spritesheet, Transform}, physics::PhysicsContext};

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
        #[cfg(debug_assertions)]
        println!("Imagem {} carregada.", path);
    }
}

pub fn physics_load(world: &mut World, resources: &mut Resources) {
    let ctx = resources
        .get_mut::<PhysicsContext>()
        .expect("load.rs: Recursos de física não inicializados corretamente.");

    let mut rigid_bodies = ctx.bodies.borrow_mut();
    let mut colliders = ctx.colliders.borrow_mut();

    let mut query = <(&Transform, &mut Body)>::query();
    for (transform, body) in query.iter_mut(world) {
        let mut rb = RigidBodyBuilder::dynamic()
            .translation(vector![transform.position.x, transform.position.y])
            .build();
        if !body.is_dynamic {
            rb = RigidBodyBuilder::fixed()
                .translation(vector![transform.position.x, transform.position.y])
                .build();
        }
        let col = ColliderBuilder::cuboid(body.size.x, body.size.y).build();
        body.body_handle = Some(rigid_bodies.insert(rb));
        body.collider_handle =
            Some(colliders.insert_with_parent(col, body.body_handle.unwrap(), &mut rigid_bodies));
        #[cfg(debug_assertions)]
        println!("Novo body (dinâmico:{}) criado em {}: {:?}", body.is_dynamic, transform.position, body.body_handle);
    }
}
