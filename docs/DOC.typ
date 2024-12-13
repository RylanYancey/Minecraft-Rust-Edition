
#text(size: 25pt)[Minecraft: Rust Edition Documentation]
#set heading(numbering: "1.")
#outline(indent: true, depth: 5)

= Core Architecture
MCRE splits the game up into 3 parts: Client, Simulation, and Server. The Client reads and packages user input into `ClientEvent`s, which are sent to the server. The Server reads these events and resolves conflicts with other users. The Server gives these events to the Simulation, which processes them when the Simulation is _ticked_. The Simulation records mutations to its state and packages them into `ServerEvents`.  These events are then sent to back to clients based on their proximity to those events.

== The Simulation
The Simulation reads `ClientEvent`s and outputs `ServerEvent`s. It is loaded as a Plugin either into the `Server` when hosting a multiplayer server or `Client` when playing singleplayer. It is a set of Systems and Resources for managing global world state, and a set of `Dimension`s (`SubApp`s) each with the Resources and Systems they need to run.

Each Dimension has it own `BevyWorld`, Resources, and Systems, but some resources are shared across Dimensions and are syncronized internally by Bevy. The Simulation will load the Overworld on start, but will not load any other Dimensions until explicitly instructed otherwise. 

== The Server

== The Client

= Registries
The `Registry<T>` type is used to describe what should exist in the world and how it behaves. Registries store entries in a `Vec<T>` and a `BTreeMap<GlobalID, usize>` for looking up these entries with hash keys. IDs for entries in a registry must follow the format:

`<namespace>:<block_name>.<state>.<more_state>`

So a furnace facing east will have the ID: "`mc:furnace.east`". 

== LocalID
A LocalID is an index of an Entry in the Registry's `Vec<T>`. Therefore, using `registry.find_by_local` is an _O(n)_ operation. LocalID's are internally a `u16`. This means that registries must not have greater than 65536 entries. LocalIDs may be different across runtimes, platforms, or game versions, and must _not_ be used for serializing registry data or sending over the internet.

== GlobalID
A GlobalID is a hash of a string identifier of an Entry in the Registry's `BTreeMap<GlobalID, usize>`. Therefore, using `registry.find_by_global` is an _O(nlog(n))_ operation. GlobalID is internally just a `u32`. GlobalIDs are guaranteed to be the same regardless of platform, runtime, or game version, making it useful for sending data over the network.

= The World
MCRE Provides a `World` type that is used for storing `BlockState` and related data and sets of iterators for reading and writing to that state. 

See `simulation/world/mod.rs` for the code.

== Memory Representation
Let's break the World down piece-by-piece.

 - The App contains Dimensions
 - Dimensions contain Worlds
 - Worlds contain Chunks 
 - Chunks contain SubChunks
 - SubChunks contain BlockStates

=== Block States
A `BlockState` is defined as a `LocalID` and a `Light`. The LocalID corresponds to an entry in the `Registry<Block>`. Because of this, every possible state a Block can have must be initialized (at program start) in the `Registry<Block>`. For example, directional blocks like Furnaces need 4 entries, one for each direction. The IDs for these 4 entries might look like:

 - mc:furnace.north
 - mc:furnace.south
 - mc:furnace.east
 - mc:furnace.west

Needing to compute every possible state at program startup is intensive and quickly gets out-of-hand; its the reason there is no slab mixing. If we did have slab mixing, there would need to be a registry entry for every possible combination of slabs, which would greatly limit the number of slabs you could have in the game. 

BlockStates also store a `Light` that represents the ambient light level, emission intensity, _hue_ and _lightness_. It packs these 4 values into 16 bits, with 4 bits for each. This means there are 0-15 ambient light levels, 0-15 emission intensity (e.g. torch light), 0-15 hues (HSL Hues), and 0-15 lightnessess (HSL Lightness). A block with an ambient light level of 15 is directly exposed to the sun, while a block with an ambient light of 4 is 10 blocks away from a block with an ambient level of 15. A block with an emission intensity of 4 is 10 blocks away from an emitter. 

For light color, I rejected using RGB because the majority of the color space would be black or gray. When you use HSL without saturation, you are left with white light and all fully-saturated colors. Color mixing is also much more efficient to compute. This is what allows MCRE to have such a high range of bright colors without using a ridiculous amount of memory.

#figure(
  table(
    columns: 4,
    [bits 0-3], [bits 4-7], [bits 8-11], [bits 12-15],
    [0000], [0000], [0000], [0000],
    [ambient], [intensity], [hue], [lightness]
  ),
  caption: [Memory Layout of Block Lights]
)

=== SubChunks
A `SubChunk` is a 32x32x32 volume of blocks in the world. Its origin is considered to be the minimum coordinate within the subchunk, for example the SubChunk with origin [0,32,64] is all blocks between that origin and [32, 64, 96], exclusively. SubChunk origins _must_ always be multiples of 32. This means that the coordinate [32, 32, 1] is not a valid origin because 1 is not a multiple of 32.  

A SubChunk is internally a `[BlockState; 32768]`. Each element is a `BlockState`, which has a size of 4 bytes, meaning that a SubChunk has a size of ~131kb. Indexing this array with a 3-D coordinate relative to subchunk origin (in the range [0, 31]) can be done with the following formula: 

#align(center, $y + x dot 32 + z dot 32 dot 32$)

SubChunk data is Y-Major. This means that the BlockStates are _contiguous_ (linear) on the Y-axis. When iterating over the elements of a subchunk, it will be significantly faster (due to cache optimization) to iterate on the Y-Axis. 

=== Chunks
Chunks are vertical columns of subchunks. MCRE makes the assumption, that, if a SubChunk exists, every subchunk between that subchunk and $y=0$ also exists. The number of SubChunks that a Chunk has is equal to the highest block in the Chunks' y-coordinate divided by 32 plus 1. So if the highest block in Chunk [0, 0] is 32, there are two SubChunks - one at [0,0,0] and one at [0,32,0].

The origin of a Chunk is a 2D coordinate containing the Chunks' lowest [x,z] coordinate. The Y value is excluded because all Chunks have a lowest y-value of 0. The coordinate's components must be multiples of 32, just like SubChunks' origin.

=== World
The `World` data structure is internally a `BTreeMap<u64, Chunk>`. 

== Interacting with The World
When you use `World::get_chunk()`, the chunk map has to be searched. This, while fast, isn't something you want to be doing thousands of times per frame. Because of this, we provide sets of iterators and readers for accessing world data more efficiently.

=== WorldReader
A WorldReader caches the most recently accessed chunk. Because of this, it is ideal for random world access where the blocks are known to be relatively close to each other. An ideal use case would be for getting random blocks in a small range.

```rs
use bevy::math::IVec3;
fn read_some_blocks(world: Res<World>) {
    let reader = world.reader();
    let points = [IVec3::new(0, 4, 0), IVec3::new(1, 4, 8), IVec3::new(8, 9, 1)];
    for pt in points {
        println!("{:?}", reader.get_block(pt).unwrap());
    }
}
```

=== Volume
Volumes are used for reading a section of the world. A Volume can be selected by calling `WorldReader::volume(origin, extent)`, but this is just a selection - no actual computation has occured yet. `Volume` provides the functions `Volume::fragments()` and `Volume::columns()`. 

The Fragments iterator accesses each subchunk the volume overlaps, returning a Fragment. A Fragment is a volume that is entirely contained by a single subchunk. This minimizes the number of calls to `World::get_chunk()`, and represent a significant performance improvement over Columns and WorldReader. However, no guarantees can be made about the order blocks are visited.

The Columns iterator returns a Column, which is an iterator over the blocks in a column, in the order of bottom-to-top. If you want to iterate from top-to-bottom, use `Column::rev()`. Columns is slower than Fragments, but the order blocks are visited is guaranteed. 

```rs
use bevy::math::IVec3;
fn read_volume(world: Res<World>) {
    let reader = world.reader();
    // select [8,8,8] to [40,40,40]
    let volume = reader.volume(IVec3::new(8, 8, 8), IVec3::new(32, 32, 32));
    // slower, but order is guaranteed.
    for column in volume.columns() {
        for (point, state) in column.rev() {
          println!("The BlockState at {:?} is {:?}.", point, state);
        }
    }
    // faster, but no guarantee on the visitation order.
    for fragment in volume.fragments() {
        for (point, state) in fragment {
            println!("The BlockState at {:?} is {:?}", point, state);
        }
    }
}
```

=== Neighbours
Accessing the neighbours of a block with WorldReader is very inefficient because we have to compute whether or not the neighbour is in another chunk, which requires alot of `World::get_chunk()`s. Luckily, MCRE pre-computes whether a subchunk coordinate is on a boundary, so this access can be optimized. 

```rs
use bevy::math::IVec3;
fn read_neighbours(world: Res<World>) {
    let reader = world.reader();
    // a block on a +z boundary
    let origin = IVec3::new(8, 9, 31);
    for (dir, state) in reader.neighbours(origin) {
        println!("The Neighbour at {:?} is {state:?}.", dir + origin);
    }
}
```

=== WorldBuffer
WorldBuffers are 

=== Clusters
This is the reader used internally for intense operations on entire chunks, such as computing chunk meshes or light updates. Clusters are limited because they require all 8 neighbouring chunks to be loaded and generated. However, accessing the world this way is much faster than other methods because we don't have to check if the chunk exists. Clusters come in 2x2 and 3x3 forms, where a 3x3 cluster is all chunks within 32 blocks of a block and a 2x2 Cluster is all blocks within 16 blocks of a block. Mutable Clusters, in the form `ClusterMut2x2` and `ClusterMut3x3` use unsafe logic internally because they hold multiple mutable references to the World, but are safe to use.

```rs
use bevy::math::IVec3;
fn read_from_cluster(world: &mut World) {
    let coord = IVec3::new(8, 8, 8);
    let cluster = world.cluster_mut_3x3(coord);
    println!("The Block at {coord:?} is {:?}", cluster.get_block(coord).unwrap());
}
```

=== WorldWriter
The WorldWriter is the main way the World should be mutated. 