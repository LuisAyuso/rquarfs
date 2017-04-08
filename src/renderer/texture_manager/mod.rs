use super::context::Context;
use super::context::IdType;
use super::context::ManagerError;

/// texture manager takes care of images to read from and to write to.
///  Textures
///  Depth buffer
///  and frame buffers which are backed up by the former two.

trait Texture {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct Texture2D {}

struct DepthTexture {}

/// a Canvas is an object we can write to.
///  it is created from a texture and an optional depth buffer
struct Canvas {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

struct TextureManager {}

impl TextureManager {
    fn new() -> TextureManager {
        TextureManager {}
    }

    fn create_texture_2D() -> Result<IdType, ManagerError> {
        unimplemented!();
    }

    fn create_depth_texture() -> Result<IdType, ManagerError> {
        unimplemented!();
    }

    fn create_canvas(&mut self, tex_id: IdType) -> Result<IdType, ManagerError> {
        unimplemented!();
        //Err(ManagerError::ItemRedefinition)
    }

    fn create_canvas_with_depth(&mut self,
                                tex_id: IdType,
                                depth_id: IdType)
                                -> Result<IdType, ManagerError> {
        unimplemented!();
        //Err(ManagerError::ItemRedefinition)
    }

    /// retrieves a texture we can read from
    fn get_texture_src(&self, id: IdType) -> Result<Texture2D, ManagerError> {
        unimplemented!();
        //Err(ManagerError::ItemRedefinition)
    }

    /// retrieves a texture we can write to.
    /// (it has a draw method)
    fn get_texture_sink(&self, id: IdType) -> Result<Canvas, ManagerError> {
        unimplemented!();
        //Err(ManagerError::ItemRedefinition)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let _ = TextureManager::new();
    }

}
