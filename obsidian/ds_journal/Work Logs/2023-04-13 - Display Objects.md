# Thursday, April 13, 2023

---

Today I want to change the sprite batcher to accept display objects rather than just raw textures. The goal will be to create many different types of display objects, each capable of customizing the way the are rendered. Hopefully this can all be done from within the sprite batching system.

However, since things like drawing lines, arbitrary 2d polygons, or text may require unique shaders we may need to have different pipelines. Either that, or we could look into using a master shader that could handle all of these cases. 

The sprite batch now assembles its draw calls using the `DisplayObject` interface, though currently it is doing so with a large amount of copying. I have also created a `DisplaySprite` object that represents a single image.

I can now create `DisplaySprite` objects that only draw a portion of a larger image. Should be easy to support a sprite atlas object now!