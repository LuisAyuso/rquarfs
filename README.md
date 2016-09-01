
## Quarfs!, a new generation using rust

basically, yet another voxel engine.
Do not expect fancy techniques, I am happy with having some cubes on screen.

# TODO:

- [x] refactor camera into module
- [ ] add some axis movement to the camera
- [x] world model into quadtree, octree? 2.5 quadtree.... *virtualQuadTree*
- [ ] check in bounds chunks to be drawn
- [ ] cache world, avoid image reading on load
- [ ] fix atlas neigboohoding problem
- [x] phong shading
- [x] projected shadows
- [ ] not so sure about this anymore ~~add a check for unused uniforms binding~~
- [ ] add some magic for the uniforms types, maybe a macro to generate a type I can return and work with. (maybe this does not need to be that fast)
- [ ] move prerspective matrix into context.
- [ ] create a world class and move the model matrix there. 
- [ ] fix depth 1 issue, add fillings 

# OR:
study vert + TCS + TES + GEOM to generate the cubes.

# future research... 
I have to read some of this:

freetype: 
* http://learnopengl.com/#!In-Practice/Text-Rendering
* https://github.com/PistonDevelopers/freetype-rs
