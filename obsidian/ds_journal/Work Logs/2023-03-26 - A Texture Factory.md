# Sunday, March 26, 2023

---

One thought I had last night was to delay the creation of the texture itself until it can be done in a system that has easy access to the graphics context. So what I have now is a `GraphicsSystem` that can start loading a texture and then poll until the data is available later in the frame from within an private update that has access to the graphics context object passed in by the event loop.

I am still not convinced this is the best plan, but its an interesting idea of the moment. Another possibility would be to wrap the graphics context object around my own context object and just pass that through to gameplay code. The gameplay code is going to end up needing some sort of context object anyway in order to access the engine or the systems.

I wonder if so many people end up making ECS frameworks because the technique lends itself well to rusts ownership model. I would be curious what other good models there are for doing something similar to the singleton manager pattern.