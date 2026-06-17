# Rusted NES

Small NES emulator written in Rust.

It isn't intended as a completely 100% accurate emulator. With time I'm planning on increasing accuracy, but as for now it can emulate some games, including Super Mario Bros.

# Implemented components
- CPU - per cycle correct emulation with support for all legal instructions
- PPU - per slice (1x8 pixels) emulation

## Credits
- [NESdev Wiki](https://www.nesdev.org/wiki/Nesdev_Wiki) - Information about NES inner workings, recommended palette
