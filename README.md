
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
- [x] fix depth 1 issue, add fillings 



# future research... 
I have to read some of this:

OGL
* http://github.prideout.net/modern-opengl-prezo/

terrain:
* height maps: https://github.com/prideout/heman
* http://vterrain.org/Elevation/Artificial/

smoke, clouds:
* http://prideout.net/blog/?p=63
* http://prideout.net/blog/?p=67
* http://prideout.net/blog/?p=58
* http://prideout.net/blog/?p=69
* http://vterrain.org/Atmosphere/Clouds/index.html
* http://ws.iat.sfu.ca/papers/clouds.pdf
* http://www.markmark.net/PDFs/RTCloudsForGames_HarrisGDC2002.pdf
* http://nis-ei.eng.hokudai.ac.jp/~doba/papers/sig00_cloud.pdf

flocks:
* http://www.red3d.com/cwr/boids/

text:
* http://learnopengl.com/#!In-Practice/Text-Rendering
* https://github.com/PistonDevelopers/freetype-rs
