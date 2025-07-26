# Aquatunnel
A 2D roguelite survival game made with Rust, using Macroquad, Legion, Rapier2D, and a modular architecture. This is a work-in-progress project — and honestly, one of my favorites <3.
Play as a diver exploring mysterious underwater caves, battling marine creatures and collecting rare items.
Aquatunnel is a survival roguelite with procedural generation, multiple weapons, and a variety of creatures.


## run it locally:
⚠️ This project is still in development!
```
git clone https://github.com/CoininDev/aquatunnel.git
cd aquatunnel
cargo run --release
```

## Explaining some features and technology.
I am using Legion as an ECS framework. ECS is a design model that compounds the world of a game with entities, components and systems. Entities are everything in the world, which has its components, and the system will run the logic of the game, searching out components and using them in an efficient way.
ECS design helps me to build a modular structure in the game.
Legion, besides entities and systems, also exists resources, which is universal information that systems could use, and I actually use it a lot in my project.

### For rendering:
Before the game starts, we search in our scene if has some Sprite components, which of them will have a path to the image, and with that we create a resource with loaded images.


And then, for each frame, we run the Render System. The render system do:
1. register each renderable entities, such as Sprites, DebugSprites, Tilemaps, etc. And then saves it all as Renderables (a custom trait that defines how to render something).
2. sort it based on the z_order() function of Renderable trait.
3. run the render() function on order.

files:
[load.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/load.rs) (image loading in the beggining of the game),
[resources/renderable.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/resources/renderable.rs),
[sys/render.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/sys/render.rs)

### For input:
For detecting input, I wanted a highly flexible input system that would be easy to modify, and I think I’ve achieved that. It’s even reusable for future projects!
That's how it works:
There is the InputContext, which is the ECS resource that owns all the input information.
it has: look and move directions (Vec2), currently active actions (HashSet), and the setup.
There is InputActions, that is actions in the context of the game, such as "run", "shoot", etc. And RawAction, that can be a key on the keyboard, button on joystick, mouse button, etc.
The setup define which RawAction corresponds to each InputActions, and it defines too the Method, or Strategy of the directions.
The move and look direction can be defined by some strategies, such as mouse direction in relations to the center, mouse delta movement, joysticks, WASD keys, etc.

I can now easily add and/or remove Input and RawActions, direction strategies changing some enums and the AxisMethod trait.

files: [resources/input.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/resources/input.rs)

### For procedural generation:
The map of the game uses a chunk system and noises to create the map, I have tried a bunch of ways to create the chunk system, but the best i've found is Chunks as entities!
Each chunk has an Chunk, ChunkBody and Tilemap components. The Chunk manages chunk loading and unloading, ChunkBody deals with the physics of eath block, and Tilemap render the Chunk.
Chunk as entities is useful because it makes the system much more simple reusing a done model, and enjoying the multithread processing built in in Legion.

The Chunk loading:
Chunks have 3 states: Loaded, unloaded and destroyed. Loaded chunks have active items, entities and all the blocks, Unloaded ones have unactive item and entities, without any block anymore. Destroyed chunks are completely removed from memory.

files:
[sys/chunk.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/sys/chunk.rs),
[resources/chunk_manager.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/resources/chunk_manager.rs),
[comps/chunk.rs](https://github.com/CoininDev/aquatunnel/blob/ECSChunkSystem/src/comps/chunk.rs),
