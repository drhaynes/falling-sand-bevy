use bevy::asset::Handle;
use bevy::prelude::{Image, Deref};
use bevy::ecs::system::Resource;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::extract_resource::ExtractResource;
use bevy::render::render_asset::RenderAssetUsages;

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct CellularAutomataImage(pub Handle<Image>);

pub fn create_image(width: u32, height: u32) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD
    );

    image.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    image
}
