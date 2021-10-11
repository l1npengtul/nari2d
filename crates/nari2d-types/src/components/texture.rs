use crate::components::scale::ScaleComponent;
use euclid::{Point3D, Rotation3D, UnknownUnit};
use gltf::Scene;
use std::borrow::Cow;

// // Always assumes RGBA
// pub struct TextureComponent {
//     height_x: u32,
//     width_y: u32,
//     scale: ScaleComponent,
//     // traaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaans
//     transparency: f32,
//     data: Cow<'static, [u8]>,
// }
//
// impl TextureComponent {}
//
// pub enum WindowMode {
//     Crop,
//     Scale,
// }
//
// // A image using a window into a 3d world
// pub struct RenderedTextureComponent {
//     height_x: u32,
//     width_y: u32,
//     source_x: u32,
//     source_y: u32,
//     mode: WindowMode,
//     scale: ScaleComponent,
//     image_data: Cow<'static, [u8]>,
//     render_model: Scene<'static>
// }
//
// impl RenderedTextureComponent {
//
// }

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum EmbedScaleMode {
    Crop,
    Scale,
}

#[derive(Clone, Debug)]
pub enum TextureType {
    Image,
    Embedded3D {
        source_height: u32,
        source_width: u32,
        scale_mode: EmbedScaleMode,
        model: Scene<'static>,
        mdl_path: String,
    },
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Modulation(f32, f32, f32, f32);

#[derive(Clone, Debug)]
pub struct TextureComponent {
    height: u32,
    width: u32,
    texture_type: TextureType,
    scale: ScaleComponent,
    transparency: f32,
    modulate: Modulation,
    self_modulate: Modulation,
    image_data: Cow<'static, [u8]>,
}
