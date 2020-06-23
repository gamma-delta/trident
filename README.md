# Trident

![The Trident logo rendered with Trident](logo.png)

Trident is a parser for polygon meshes, designed for use with ggez.

I made Trident for a game I'm working on. Normally I like to use pixel art, but for this game I needed more resolution. I also really like the low-poly aesthetic, but wanted to keep the game 2D. So, Trident is how I'll be writing the sprites.

# `trident-viewer`

A viewer for `.tri` files, written with ggez. 

Controls:
- WASD: pan
- QZ: zoom
- R: reload

Pick the file to open from the command line (`trident-viewer path/to/file.tri`). If you make a change to the file, press `r` to reload it.

# Examples

```
# Comment stuff with hashtags

# This creates a triangle with points at (0, 0), (10, 0), and (5, 0), that is red.
# Each point must be seperated with semicolons.
# The period indicates the end of the points.
0, 0; 10, 0; 5, 10;. ff0000

# Whitespace can be inserted to make things clearer.
0, 0  ; 5, 2.5  ; 0, 5  ;. 000000
0, 5  ; 5, 7.5  ; 0, 10 ;. 001000
0, 10 ; 5, 12.5 ; 0, 15 ;. 002000
0, 15 ; 5, 17.5 ; 0, 20 ;. 003000
# This makes a sawtooth pattern that gets greener as it goes down.
```

Here's the Trident logo:

```
# The background splash
-5, 20; 15, -8; 6, 32;. f3dfbf # Big splash
-4, 10; 14, 3; 15, 25;. eb8a90 # Smaller part


# The prongs are 10 tall and 4 wide.
2, 0 ; 4, 10 ; 0, 10;. 42e2b8
6, 0 ; 8, 10 ; 4, 10;. 42e2b8
10, 0; 12, 10; 8, 10;. 42e2b8

# Handle is 40 tall and 12 wide
0, 10 ; 12, 10 ; 6, 50 ;. 2d82b7

# Inlay
1.0, 12; 11, 12; 6, 45 ;. 07004d
```

See [the examples](examples) for more!