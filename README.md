
## Quarfs!, a new generation using rust

basically, yet another voxel engine.
Do not expect fancy techniques, I am happy with having some cubes on screen.

# TODO:

- [x] refactor camera into module
- [ ] world model into quadtree, octree? 2.5 quadtree.... *virtualQuadTree*
- [ ] check in bounds chunks to be drawn
- [ ] cache world, avoid image reading on load
- [ ] fix atlas neigboohoding problem
- [x] phong shading
- [ ] projected shadows
- [ ] add a check for unused uniforms binding.... this is the second time I get lost here.


# future research... 
I have to read some of this:

freetype: 
* http://learnopengl.com/#!In-Practice/Text-Rendering
* https://github.com/PistonDevelopers/freetype-rs
