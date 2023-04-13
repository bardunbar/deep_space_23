# Saturday, April 08, 2023

---

Now that we have a simple texture being rendered to the screen I want to figure out how best to expose image creation and manipulation to game modules. A simple goal would be an interface similar to Corona/Solar SDK, with display groups of display objects that act hierarchically. 

Speaking of display objects, since I will likely wish to implement other 2D objects to display, namely shapes and text, it may be beneficial to start thinking about an object interface or implementation that would enable the creation of sprites as well as lines, polygons, and other shapes.

Thinking about the requirements for the space station game I want to make sure I can really efficiently render a dynamic tilemap. I may have been using it wrong, but I was quite unhappy with the performance of the tilemap plugin for bevy. 

Depending on the expected number of tiles visible at any one time, we may be able to get away with treating each tile as its own object to be rendered, and then making sure to properly batch the draws with the planned sprite batcher. The other approach would be to define "chunks" of the map each as their own mesh, updating these chunks as the player interacts with them.

The sprite batcher will almost certainly be a simpler option to start with, and even if we then need to implement the chunking system for the map, other systems would still use the sprite batcher, so this is definitely the right place to start.