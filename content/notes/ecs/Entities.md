---
title: "Entities"
tags:
- ecs
---

In Bevy, entities are a thing in your game, such as a player character, a bullet, or a piece of terrain. Entities are used to manage the data and behavior of game objects within the Bevy engine, and are the core building blocks of Bevy's ECS architecture.

Entities in Bevy are represented by an `Entity` struct, which is a simple, lightweight data structure that stores a unique integer ID that identifies the entity. This ID is used to reference the entity within Bevy's systems, and make changes to the entity such as adding new components or despawning it.

## Spawning

To spawn an empty entity you can use the commands parameter within a [system](notes/ecs/Systems.md). The code would look something like the following.

```rs
fn my_system(mut commands: Commands) {
	commmands.spawn_empty();
}
```

## Spawning with Components

However, most of the time you will want to spawn an entity with [components](notes/ecs/Components.md) attached to it, that can be done with the following.

```rs
fn my_system(mut commands: Commands) {
	commmands.spawn(MyComponent);
}
```

```rs
fn my_system(mut commands: Commands) {
	commmands.spawn((
		MyFirstComponent,
		MySecondComponent,
		MyThirdComponent,
	));
}
```

## Referencing an Entity

To reference an entity later you can get its ID with the following.

```rs
fn my_system(mut commands: Commands) {
	let id = commmands.spawn(MyComponent).id();
}
```

You can then edit or despawn it with the following.

```rs
fn my_system(mut commands: Commands) {
	// ...
	commands.entity(id).insert(MyFourthComponent);
}
```

```rs
fn my_system(mut commands: Commands) {
	// ...
	commands.entity(id).despawn();
}
```