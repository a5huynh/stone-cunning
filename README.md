# stone cunning

`stone-cunning` is a Dwarf Fortress inspired town builder.

## Building & Running

You can run the current version of `stone-cunning` by running the following
command in the project root directory:

    > cargo run --release


## Development

Stone cunning is divided up using [Cargo workspaces][cargo-workspaces], with
the idea that each individual library can be tested and run by itself without the need to run the entire game.

[cargo-workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

Libraries can depend on each other and often refer to each other, i.e. `libpath`
will need to refer to types in `libterrain` to make sure a path is unencumbered.

### libdwarf

The core of `stone-cunning`, used to simulate the dwarfs and (eventually) monsters.

### libterrain

Library used to generate terrain and eventually handle chunking, saving/loading
maps, etc.