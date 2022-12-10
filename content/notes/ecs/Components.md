---
title: "Componenets"
tags:
- ecs
---

In Bevy, components are data structures that store the state and properties of a game object, such as its position, velocity, or health.

Components in Bevy are defined using regular Rust structs, and can be attached to [entities](notes/ecs/Entities.md) as described in [Spawning with Components](notes/ecs/Entities.md#Spawning%20with%20Components).

## Custom Components

Any struct or enum can be used as a component just by deriving from the `Component` trait. Check out the following examples.

```rs
#[derive(Component)]
enum State {
	Idle,
	Moving,
	Attacking,
}
```

```rs
#[derive(Component)]
struct Velocity {
	x: f32,
	y: f32,
}
```

```rs
#[derive(Component)]
struct Health {
	current: f32,
	max: f32,
}
```