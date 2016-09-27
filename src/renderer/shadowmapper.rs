extern crate glium;

use renderer::context;
use glium::vertex::MultiVerticesSource;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Shadow mapper code
// http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct ShadowMapper {
    shadow_program: glium::Program,
    shadow_map: glium::Texture2d,
    depth_tex:glium::texture::DepthTexture2d,
}

impl ShadowMapper {

    pub fn new(ctx: &context::Context) -> ShadowMapper {
        use glium::texture;

        //glTexImage2D(GL_TEXTURE_2D, 0,GL_DEPTH_COMPONENT16, 1024, 1024, 0,GL_DEPTH_COMPONENT, GL_FLOAT, 0);
        let texture = texture::Texture2d::empty_with_format(ctx.display(), 
                                    texture::UncompressedFloatFormat::F32,
                                    texture::MipmapsOption::NoMipmap, 
                                    1024, 1024).unwrap();

       //                      glium::texture::DepthFormat::I16, 1024, 1024,).unwrap();
        let depth = texture::DepthTexture2d::empty_with_format(ctx.display(), texture::DepthFormat::F32,
                                            texture::MipmapsOption::NoMipmap, 1024, 1024).unwrap();

        let shadow_program =
            glium::Program::from_source(ctx.display(),
                // vertex shader
            "
				#version 330 core

                uniform mat4 light_space_matrix;
                uniform mat4 model;

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                layout (location = 0) in vec3 position;

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                smooth out float distance;
    
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void main(){
                    gl_Position = light_space_matrix* model * vec4( position, 1.0);
				}
            ",
               // fragment shader
            "
                #version 330 core

                uniform mat4 light_space_matrix;
                uniform mat4 model;
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                smooth in float distance;

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                out float fragmentdepth;

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void main(){
                 //   float x = gl_FragCoord.x / 1024;
                 //   float y = gl_FragCoord.y / 1024;
                 //   float z = gl_FragCoord.z;
                 //   fragmentdepth = z;
                 }
            ", None).unwrap();

        ShadowMapper {
            shadow_program: shadow_program,
            shadow_map: texture,
            depth_tex: depth,
        }
    } // new

    pub fn compute_depth<U>(&self, 
                             ctx: &context::Context,
                             vertices: &glium::vertex::VertexBufferAny,
                             uniforms: &U) 
    where U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;

        let mut framebuffer  = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(), 
                                                                                  &self.shadow_map,
                                                                                  &self.depth_tex,
                                                                                 ).unwrap();
        let indices = glium::index::NoIndices( glium::index::PrimitiveType::TrianglesList);

		let parameters = glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            // nonsense, just debug
            //polygon_mode: glium::draw_parameters::PolygonMode::Line,
			.. Default::default()
		};

        //float 16 buffer, only red componet is used
        //framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        framebuffer.clear_depth(1.0);
        framebuffer.draw(vertices,
                         indices,
                         &self.shadow_program,
                         uniforms, 
                         &parameters).unwrap();
    } 

    pub fn compute_depth_with_indices<U>(&self, 
                             ctx: &context::Context,
                             vertices: &glium::vertex::VertexBufferAny,
                             indices: &glium::index::IndexBufferAny,
                             uniforms: &U) 
    where U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;

        let mut framebuffer  = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(), 
                                                                                  &self.shadow_map,
                                                                                  &self.depth_tex,
                                                                                 ).unwrap();
		let parameters = glium::DrawParameters {
            backface_culling: glium::BackfaceCullingMode::CullClockwise,
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
			.. Default::default()
		};

        //float 16 buffer, only red componet is used
        //framebuffer.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        framebuffer.clear_depth(1.0);
        framebuffer.draw(vertices,
                         indices,
                         &self.shadow_program,
                         uniforms, 
                         &parameters).unwrap();
    } 

    pub fn depth_as_texture(&self) -> &glium::texture::DepthTexture2d{
        &self.depth_tex
    }
}


