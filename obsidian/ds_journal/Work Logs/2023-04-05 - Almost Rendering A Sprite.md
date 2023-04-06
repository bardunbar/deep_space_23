# Wednesday, April 05, 2023

---

Today I want to go back to basics slightly and just get a texture rendering on the screen. I am getting slightly ahead of myself trying to set up sprite batching and scene organization too early. Best break this up in to easier to digest chunks and this will let me know if my current method for loading textures has any immediate flaws that would need to be addressed.

So I ended up using the sprite batch object as a base anyway since it ended up being a convenient place to store the pipeline and binding information. Still, some optimizations will be done to it once we are past the texture rendering state. At the moment I create and throw away the buffers every frame, but realistically I should be saving them across multiple frames. This should be relatively easy to add.

Well after over an hour of searching for the cause of a weird compiler error I discovered I was missing a "#" in front of the `version` pre-processor directive for my fragment shader. I should make a task to read these shaders in from an asset if I ever have to implement anything fancier than what I currently have. 

However, even though that is now working, I am getting a new crash with a bizarre memory violation. I am guessing it has to do with me using buffers incorrectly... but lets see if we can figure it out tonight. 

Right, yea that must have had something to do with creating and deleting buffers over and over, I changed it so the buffers are created once and never deleted and the access violation went away. Now we are staring at a blank screen! Which is unfortunate since there should in theory be a texture being drawn...