# Thursday, March 23, 2023

---

Today I want to give a little thought to how I might structure this project. Nothing too fancy at first but a basic plan will help inform me on how to set up the initial event handling application.

One thing I liked about the way bevy worked is its use of plugins. I liked how plugins could be complied conditionally and how they helped organize both the engine features as well as application features.  

One goal of the structuring is to hide away all of the references to Miniquad or other utility crates. This may seem like a silly goal but I want to keep the game code agnostic of the implementation details so that it will be easier for me to customize under the hood without directly affecting the game code itself. This would also allow me to more easily replace miniquad or the other utility crates at some point in the future if I ever wish to.