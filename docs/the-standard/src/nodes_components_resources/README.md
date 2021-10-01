# The Node System
Nari2D Files consist of a Node system, similar to GLTF 2.0. This makes it easy for items to inherit
things such as position and rotation from their parent node. To refer to other nodes, a system of 
unique IDs and paths are used.
```
Root
|-Character
|- |-Eyes
|- |-(etc etc)
```

A Nari2D file with nodes is referred to as a "Scene". There can be more than 1 scenes when creating a
project.

So

```
Scene1
|-Root
|- |- Nodes..

Scene2
|- Root
|- |- Scene1
```

There are 2 principal types of Nari2D Scenes, based on the type of the Root node.
- Stage
- Model

Stages are like theaters. They can contain multiple child nodes and are more meant to play some
animation. 

Models, however, are meant to be animated themselves. 

There is nothing definitive that seperates a Model and a Stage, but it is worth keeping in mind espically
for terminology and since some may only support one or the other.
