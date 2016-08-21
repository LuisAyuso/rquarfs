extern crate glium;

use renderer::context;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//    Shadow mapper code
// http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct ShadowMapper {
    shadow_program: glium::Program,
    shadow_map: glium::Texture2d,
    depth_buff:glium::framebuffer::DepthRenderBuffer,
}

impl ShadowMapper {

    pub fn new(ctx: &context::Context) -> ShadowMapper {

        //glTexImage2D(GL_TEXTURE_2D, 0,GL_DEPTH_COMPONENT16, 1024, 1024, 0,GL_DEPTH_COMPONENT, GL_FLOAT, 0);
        let texture = glium::texture::Texture2d::empty_with_format(ctx.display(), 
                                    glium::texture::UncompressedFloatFormat::F32F32F32F32, 
                                    glium::texture::MipmapsOption::NoMipmap, 
                                    1024, 1024).unwrap();

        let depth = glium::framebuffer::DepthRenderBuffer::new(ctx.display(),
                             glium::texture::DepthFormat::I24, 1024, 1024,).unwrap();


        let shadow_program =
            glium::Program::from_source(ctx.display(),
                // vertex shader
            "
				#version 330 core

                layout (location = 0) in vec3 position;
                layout (location = 3) in vec3 world_position;

                uniform mat4 perspective_matrix;
                uniform mat4 view_matrix;
                uniform mat4 model_matrix;

				void main(){
                    vec4 tmp = vec4(position + world_position, 1.0);
                    gl_Position = perspective_matrix * view_matrix * model_matrix * tmp;
				}
            ",
               // fragment shader
            "
				#version 330 core

				// Ouput data
				//layout(location = 0) out float fragmentdepth;
                layout(location = 0) out vec4 frag_color;

				void main(){
					// Not really needed, OpenGL does it anyway
					//fragmentdepth = gl_FragCoord.z;
                    frag_color = vec4(2.0, 4.0, 0.0, 1.0);
				}
            ", None).unwrap();

        ShadowMapper {
            shadow_program: shadow_program,
            shadow_map: texture,
            depth_buff: depth,
        }
    } // new

    pub fn draw_depth<O,U>(&self, 
                             ctx: &context::Context,
                             obj : &O, 
                             instances: &context::VerticesT, 
                             uniforms: &U) 
    where O: context::DrawIndexed, U: glium::uniforms::Uniforms
    {
        //println!("b");
        use glium::Surface;

        let mut target  = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(ctx.display(), 
                                                                                  &self.shadow_map,
                                                                                  &self.depth_buff,
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

		target.clear_depth(0.0);
        target.draw((obj.get_vertices(), instances.per_instance().unwrap()),
                         obj.get_indices(),
                         &self.shadow_program,
                         uniforms, 
                         &parameters).unwrap();
    } 

    pub fn texture(&self) -> &glium::Texture2d{
        &self.shadow_map
    }
}


