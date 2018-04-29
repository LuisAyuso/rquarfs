use super::context::Context;
use super::context::IdType;
use super::context::ManagerError;

use std::collections::BTreeMap;

use glium::texture;

/// texture manager takes care of images to read from and to write to.
///  Textures
///  Depth buffer
///  and frame buffers which are backed up by the former two.

trait Texture {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct Texture2D {
    tex: texture::Texture2d,
}

struct DepthTexture {}

/// a Canvas is an object we can write to.
///  it is created from a texture and an optional depth buffer
struct Canvas {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct TextureManager {
    textures: BTreeMap<IdType, Texture2D>,
    sinks: BTreeMap<IdType, Canvas>,
}

impl TextureManager {
    fn new() -> TextureManager {
        TextureManager {
            textures: BTreeMap::new(),
            sinks: BTreeMap::new(),
        }
    }

    fn create_texture_2D(&mut self,
                         ctx: &mut Context,
                         name: &str,
                         h: u32,
                         w: u32)
                         -> Result<IdType, ManagerError> {
        let tex =
            texture::Texture2d::empty_with_format(ctx.display(),
                                                  texture::UncompressedFloatFormat::F32F32F32F32,
                                                  texture::MipmapsOption::NoMipmap,
                                                  h,
                                                  w);
        if tex.is_err() {
            return Err(ManagerError::BackEndErrror);
        }
        let id = ctx.get_id_for(name);
        self.textures.insert(id, Texture2D { tex: tex.unwrap() });
        Ok(id)
    }

    fn create_depth_texture(&mut self, ctx: &mut Context) -> Result<IdType, ManagerError> {
        unimplemented!();
    }

    fn create_canvas(&mut self, ctx: &mut Context, tex_id: IdType) -> Result<IdType, ManagerError> {
        unimplemented!();
        // Err(ManagerError::ItemRedefinition)
    }

    fn create_canvas_with_depth(&mut self,
                                ctx: &mut Context,
                                tex_id: IdType,
                                depth_id: IdType)
                                -> Result<IdType, ManagerError> {
        unimplemented!();
        // Err(ManagerError::ItemRedefinition)
    }

    /// retrieves a texture we can read from
    fn get_texture_src(&self, id: IdType) -> Option<&Texture2D> {
        self.textures.get(&id)
    }

    /// retrieves a texture we can write to.
    /// (it has a draw method)
    fn get_texture_sink(&self, id: IdType) -> Option<&Canvas> {
        self.sinks.get(&id)
        // Err(ManagerError::ItemRedefinition)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ctor() {
        let _ = TextureManager::new();
    }

    #[test]
    fn create() {
        let mut ctx = Context::new_headless(100, 100).unwrap();
        let mut mgr = TextureManager::new();
        let id = mgr.create_texture_2D(&mut ctx, "texture", 100, 100);
        assert!(id.is_ok());
        let tex = mgr.get_texture_src(id.unwrap());
        assert!(tex.is_some());
        let tex = mgr.get_texture_sink(id.unwrap());
        assert!(tex.is_none());

    }
}
