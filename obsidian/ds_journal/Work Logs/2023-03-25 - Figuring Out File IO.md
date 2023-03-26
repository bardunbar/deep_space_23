# Saturday, March 25, 2023

---

Today I am hoping to set up a basic texture load and display. I am going to forgo a proper asset manager or rendering system at this time, just to get something up and running on the screen.

I am drawing heavily from macroquad as an inspiration as I learn how to do some of these things in rust. 

The first minor challenge is that the API for loading bytes from disk in miniquad appears to be an asynchronous function. Instead of blocking and returning the bytes it will call a callback function when ready. For desktop builds it seems as though this function will get called right away, effectively making it synchronous. I expect web or other platforms may introduce the latency.

The flip side of this issue of course, is that I could create an asset manager first, that is capable of handling the async nature of this file io.