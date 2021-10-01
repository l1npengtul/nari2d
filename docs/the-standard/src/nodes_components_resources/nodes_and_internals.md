# Nodes and their Internals

The base Nari2D Node only contains a couple parts:
- The UUID
- Its Name
- The UUID of its parent
- Its Components

The rest of Node behaviour is added through Components and data through Resources.

## What are Components?

Components are a way of modularizing logic and seperating them from the main node, increasing efficiency
by not doing stuff we do not need to do. For example, a node that only sits still does not need 
logic for animations, etc.

- Position(Default): Stores the (x, y) position of the Node.
- Visible(Default): Stores if the Node should be rendered or not
- Transform: Stores the transformation matrix for the node
- Physics: Stores the physics related data.
- Animation: Stores animation related data.
- ParticleEmitter: Stores data for a particle emitter.
- ParticleParticle: Stores data for a particle emitted.
- Light: Stores light related data

## What are Resources?
So far, we have only talked about Components and Nodes. However, there is data that needs to be held by components.
Resources are our way of holding references to data.

Resources can by any type of data. Animation tracks, image layers, bone data, etc.
