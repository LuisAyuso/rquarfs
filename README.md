
## Quarfs!, a new generation using rust

basically, yet another voxel engine.
Do not expect fancy techniques, I am happy with having some cubes on screen.

[![Build Status](https://travis-ci.org/LuisAyuso/rquarfs.svg?branch=master)](https://travis-ci.org/LuisAyuso/rquarfs)

# TODO:
- [x] single file shader pipeline. 
    - [ ] improve error reporting
    - [ ] add common text (uniforms and shading version)
- [x] tessellated terrain. 
    - [x] fix issue with the two triangles, one up, one down. (is there a way to identify triangles from the same primitive quad?) dot product to the rescue!
    - [x] create side quads. end the cubes.
    - [ ] non linier interpolation for the tessellation level, I want it to change fast at short distances but not so much in the larger distances.
    - [ ] do not generate cubes for levels < 64 or 32. those are just elevation. 
    - [ ] pass recomended minimun detail for a chunk. a peak should never turn flat.
- [ ] feedback buffer. we dont want to tessellate all the time, some oclusion would be nice


