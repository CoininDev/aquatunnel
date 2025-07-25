# Aquatunnel
A game made with Rust, Macroquad, Legion, Rapier2D and modular architecture. This is a work in progress project and honestly one of my favourites <3.
Be a diver, enter in an underwater cave, fight marine creatures and collect rare items. This is a survival roguelite game, with procedural generation, multiples weapons and creatures.

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

### For input:
For detecting input, I wished a system that is really flexible, and could be easily change things, and I think I got it, it even can be used in my next projects!
That's how it works:
There is the InputContext, which is the ECS resource that owns all the input information.
it has: look and move directions (Vec2), currently active actions (HashSet), and the setup.
There is InputActions, that is actions in the context of the game, such as "run", "shoot", etc. And RawAction, that can be a key on the keyboard, button on joystick, mouse button, etc.
The setup define which RawAction corresponds to each InputActions, and it defines too the Method, or Strategy of the directions.
The move and look direction can be defined by some strategies, such as mouse direction in relations to the center, mouse delta movement, joysticks, WASD keys, etc.

I can now easily add and/or remove Input and RawActions, direction strategies changing some enums and the AxisMethod trait.

### For procedural generation:
The map of the game uses a chunk system and noises to create the map, I have tried a bunch of ways to create the chunk system, but the best i've found is Chunks as entities!
Each chunk has an Chunk, ChunkBody and Tilemap components. The Chunk 
