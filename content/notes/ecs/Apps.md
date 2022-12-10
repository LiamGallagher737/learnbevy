---
title: "Apps"
tags:
- ecs
---

Apps are the core building blocks of Bevy games and applications, and are typically used to manage the overall structure and flow of the game.

## Setup

Setting up your Bevy app could look something like this

```rs
// Make all the most common Bevy
// things accessable in this file
use bevy::prelude::*;

// Runs when the program is started
fn main() {
    // Create a new app
    App::new()
        // Add the default things almost 
        // all apps and games will need
        .add_plugins(DefaultPlugins)
        // Run the app and open the window
        .run();
}
```

Running this code should open up an empty window.