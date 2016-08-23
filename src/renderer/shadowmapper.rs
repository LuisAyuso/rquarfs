extern crate glium;

use renderer::context;

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

        //glTexImage2D(GL_TEXTURE_2D, 0,GL_DEPTH_COMPONENT16, 1024, 1024, 0,GL_DEPTH_COMPONENT, GL_FLOAT, 0);
        let texture = glium::texture::Texture2d::empty_with_format(ctx.display(), 
                                    glium::texture::UncompressedFloatFormat::F32,
                                    glium::texture::MipmapsOption::NoMipmap, 
                                    1024, 1024).unwrap();

       // let depth = glium::framebuffer::DepthRenderBuffer::new(ctx.display(),
       //                      glium::texture::DepthFormat::I16, 1024, 1024,).unwrap();
        let depth = glium::texture::DepthTexture2d::empty(ctx.display(), 1024, 1024).unwrap();



        let shadow_program =
            glium::Program::from_source(ctx.display(),
                // vertex shader
            "
				#version 330 core

                uniform mat4 light_space_matrix;
                uniform mat4 model;

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                layout (location = 0) in vec3 position;
                layout (location = 1) in vec3 normal;
                layout (location = 2) in vec2 tex_coord;
                layout (location = 3) in vec3 world_position;
                layout (location = 4) in vec3 in_color;      
                layout (location = 5) in vec2 tex_offset;      

                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

                smooth out float distance;
    
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

				void main(){
                    vec4 tmp = vec4(position + world_position, 1.0);
                    gl_Position = light_space_matrix* model * tmp;
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

    pub fn draw_depth<O,U>(&self, 
                             ctx: &context::Context,
                             obj : &O, 
                             instances: &context::VerticesT, 
                             uniforms: &U) 
    where O: context::DrawIndexed, U: glium::uniforms::Uniforms
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
        framebuffer.draw((obj.get_vertices(), instances.per_instance().unwrap()),
                         obj.get_indices(),
                         &self.shadow_program,
                         uniforms, 
                         &parameters).unwrap();
    } 

    pub fn texture(&self) -> &glium::Texture2d{
        &self.shadow_map
    }
    pub fn depth_as_texture(&self) -> &glium::texture::DepthTexture2d{
        &self.depth_tex
    }
}


