---
title: "Overview"
tags:
- ecs
---

In Bevy's ECS, [entities](notes/ecs/Entities.md) are unique identifiers that represent a single game object, such as a player character or a bullet. [Components](notes/ecs/Components.md) are data structures that store the state and properties of an entity, such as its position, velocity, or health. [Systems](notes/ecs/Systems.md) are reusable pieces of logic that operate on entities and their components, and are used to implement the behavior of the game.

The ECS architecture is designed to provide a number of benefits for game development, including improved performance, modularity, and maintainability. By separating the data and behavior of game objects into distinct entities, components, and systems, Bevy's ECS makes it easier to write efficient code and avoid common pitfalls such as tight coupling and code duplication. It also allows developers to easily reuse and combine different systems to create complex game mechanics, and to easily make changes and additions to the game without breaking existing code.