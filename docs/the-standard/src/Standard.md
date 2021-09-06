# Nari2D 

## Architecture
The Nari2D Architecture will consist of a parent-child node system.
```
Root
|-Character
|--Eyes
|--(etc etc)
```

### Global ID System
Everything in a Nari2D should have an u64 global ID. It is unique and be able to used to refer to
anything from a Node to an Animation. However, there should be specific IDs as well to get
e.g. Animations only.

### Trees and Nodes

#### Nodes
Nodes consist of an ID(u64), Name(String), Components, and their Children. They are designed to
be as versatile as possible to enable future extensions to them. Most of their logic is handled by
Components. 

#### Components
Components are either logic or data that is assigned to a Node. These can be 
as simple as a position to as advanced as animations and transformation matrices. 

**List of Components**:

- Position(Default): Stores the (x, y) position of the Node.
- Visible(Default): Stores if the Node should be rendered or not
- ImgData: The image data for the Node
- Transform: Stores the transformation matrix for the node
- Bone: Stores a riggable bone
- Static: Stores if this should be static. 
- Physics: Stores the physic simulation related data.
- Animation: Stores animation related data. 
- ParticleEmitter: Stores data for a particle emitter.
- ParticleParticle: Stores data for a particle emitted.
- Light: Stores light related data
- Scene: Stores data related to a scene. Organizes Nodes below it.
- Entity: Stores data related to an entity, like a character. Organizes Nodes below it.

Some are mutually exclusive.

#### Systems
Systems are provided by Nari2D itself and interact with above components, separating data 
and logic. 

### Contexts, Extensions, Rendering

#### Context
Every user will have to construct a new `Context` in order to interface with the Nari2D API.
The goals for the context is to have it extensible and thread-safe for certain APIs.

#### Extensions
Users should be able to define or override systems provided by Nari by implementing the 
`System` trait and defining it in the building of the `Context`.

#### Rendering
The renderer will be provided as a system, and we will provide many backends 
such as Godot or Bevy or WGPU through the use of backend abstraction crates. We will also
have a bare abstraction crate that will provide an intermediate style renderer allowing plugging
in of Nari2D into any place.

### File Format

#### File Extensions

- Outputs:
  - `.n2d`: A compiled Nari2D file.
  - `.n2db`: A compiled Nari2D in binary format (embedded data)
- Project Configuration
  - `.n2d-manifest`: A Nari2D Manifest file containing author data for e.g. an editor
  - `.n2ds`: Nari2D Script file
- Plugins
  - `n2dp`: Nari2D Plugin File

#### Internal Format
The configuration of a Nari2D file will be made in XML. For images, the TIFF should be used
to provide cross-application layered information, with the final data being exported to either a 
PNG or embedded directly in the output file.

#### XML Format
See 

