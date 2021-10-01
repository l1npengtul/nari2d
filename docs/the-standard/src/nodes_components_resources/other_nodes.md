# Node Paths and IDs

## Node Paths
Node paths work similarly to linux file paths. A path starting with "/" indicates an absolute path,
starting from the Scene root. Anything else is a relative path.

A ".." means go up a step. It is similar to file paths or Godot `NodePath`s.

## Node UUIDs
A UUID is an 64 bit identifier that is assigned to a Resource, Node, or Scene to be able to address them
without any need for locality. 

Here is the UUID Format, as it would be represented in Hex (1 character = 4 bits):
```
{TYPE: 4 BITS}-{MIMETYPE: 12 BITS}-{ID: 48 BITS}
```

Type: A Type of thing such as a Node, Resource, Scene, Animation, etc.
ID: An ID of specified type. Guaranteed to be unique. 

