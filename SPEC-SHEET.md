BUTTER ENGINE
------------------------------

Goals
1. Provide UI standard in HTML or other ways
2. Debug UI for entities or other data
3. Audio
4. Physics
5. Hot reloading of game code
6. Streamlined development
7. Networking
8. Tick updates vs frame updates
9. GLTF level editor with custom data for entites
10. Based off of Bevy Game Engine
11. Light baking
12. Cascading shadow maps
13. Screen space reflections

Entities
- World entities vs Game entities
- World entities are added on map load (Terrain, Lights, Props)
- Game entites are added by code (Players, Weapons, Input)

HTML UI
- Ultralight
- JavaScript interop
- TODO: Figure out what else is required
- Make as plugin
- Look for other solutions in Bevy

Debug UI
- Egui + bevy-inspector-egui

Audio
- steam-audio
- TODO: Figure out what else is required
- Make as plugin

Physics
- Rapier? PhysX?
- TODO: Figure out what else is required

Networking
- Packets
- RPC
- Execute on all clients
- Executa on a specific client
- Get info from server
- Update Packet
- TODO: Figure out what else is required

Bevy
- Fixed updates/ticks
- Networking?
- Hot reloading

Rendering
- Light baking
- Cascading shadow maps
- Screen space reflections

GLTF Level Editor
- Blender/Unity/Godot/Custom as a base
- Mesh geometry/materials/models
- Export as GLTF
- Custom entities with logic written in Rust
- Export custom data in GLTF