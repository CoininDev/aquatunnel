use std::{collections::HashMap, sync::Arc};

use crate::{comps::*, physics::PhysicsContext};
use legion::{world::SubWorld, *};
use rapier2d::{
    na::vector,
    prelude::{ColliderBuilder, RigidBodyBuilder},
};
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

#[system]
#[write_component(DynamicBody)]
#[write_component(StaticBody)]
#[read_component(Transform)]
pub fn load_physics(world: &mut SubWorld, #[resource] physics: &mut PhysicsContext) {
    let mut dyn_query = <(&Transform, &mut DynamicBody)>::query();
    for (transform, body) in dyn_query.iter_mut(world) {
        let rb = RigidBodyBuilder::dynamic()
            .translation(vector![transform.position.x, transform.position.y])
            .rotation(transform.rotation as f32)
            .build();

        let handle = physics.bodies.insert(rb);
        body.handle = Some(handle);
        let collider = ColliderBuilder::cuboid(
            body.size.x * transform.scale.x,
            body.size.y * transform.scale.y,
        )
        .build();

        physics
            .colliders
            .insert_with_parent(collider, handle, &mut physics.bodies);
    }

    let mut stt_query = <(&Transform, &mut StaticBody)>::query();
    for (transform, body) in stt_query.iter_mut(world) {
        let rb = RigidBodyBuilder::fixed()
            .translation(vector![transform.position.x, transform.position.y])
            .rotation(transform.rotation as f32)
            .build();

        let handle = physics.bodies.insert(rb);
        body.handle = Some(handle);
        let collider = ColliderBuilder::cuboid(
            body.size.x * transform.scale.x,
            body.size.y * transform.scale.y,
        )
        .build();
        println!("{}, {}", 
            body.size.x * transform.scale.x,
            body.size.y * transform.scale.y,
        );

        physics
            .colliders
            .insert_with_parent(collider, handle, &mut physics.bodies);
    }
}
