<h1><img src="assets/icon.png" width="100" align="left">&nbsp;sobinary</h1>

&nbsp; Toolset for shared object binary analysis

## About

Sometimes in embedded developer life there is a moment where regular developers tools are limited. I had such experience with `objdump`, `readelf` - both provides an information about dependencies from the *ELF* files but (as it is fully understandable) does not extend them recursively.
There are alternatives like `lddtree` or `libtree`... In the typical embedded use case there is a difference between target platform and host, and both of them should be available on the host.

## Goal

Find dependencies in given sysroots and destinations, conflicting libraries in the shared object linking tree and overview of the given destination.

## Dependencies

- `clap` - CLI arguments parsing,
- `goblin` - parsing ELFs,
- `simplelog` and `logger` - logging.

## Build

Just execute from the shell `cargo build`.
