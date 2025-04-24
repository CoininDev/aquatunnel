use std::{collections::HashMap, sync::Arc};

use crate::{comps::*, physics::PhysicsContext};
use legion::{query::*, system, world::SubWorld, World};
use macroquad::texture::{load_texture, Texture2D};
use rapier2d::{
    na::vector,
    prelude::{ColliderBuilder, RigidBodyBuilder},
};

#[system(for_each)]
pub async fn load(
    sprite: &Sprite,
    #[resource] textures: &mut HashMap<String, Arc<Texture2D>>,
) {
    if !textures.contains_key(&sprite.image_path) {
        println!("cadastrando sprite '{}' em textures", sprite.image_path);

        let tex = match load_texture(&sprite.image_path).await {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Erro ao carregar textura '{}': {:?}", sprite.image_path, e);
                return;
            }
        };
        textures.insert(
            sprite.image_path.clone(),
            Arc::new(tex),
        );
    }
}

#[system(for_each)]
pub async fn load_spritesheet(
    sprite: &Spritesheet,
    #[resource] textures: &mut HashMap<String, Arc<Texture2D>>,
) {
    if !textures.contains_key(&sprite.image_path) {
        println!("cadastrando sprite '{}' em textures", sprite.image_path);

        let tex = match load_texture(&sprite.image_path).await {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Erro ao carregar textura '{}': {:?}", sprite.image_path, e);
                return;
            }
        };
        textures.insert(
            sprite.image_path.clone(),
            Arc::new(tex),
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
            body.size.x,// * transform.scale.x,
            body.size.y,// * transform.scale.y,
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
            body.size.x,// * transform.scale.x,
            body.size.y,// * transform.scale.y,
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
