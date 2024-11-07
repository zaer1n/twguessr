pub mod types;
use crate::types::Error;

use image::{ImageBuffer, ImageFormat, Rgba};
use pollster::FutureExt;
use std::io::Cursor;
use std::iter;
use vek::{Extent2, Vec2};

use twgpu::map::{GpuMapData, GpuMapStatic};
use twgpu::textures::Samplers;
use twgpu::{device_descriptor, Camera, GpuCamera, TwRenderPass};
use twgpu_tools::DownloadTexture;
use twmap::{LayerKind, LoadMultiple, TwMap};

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
const LABEL: Option<&str> = Some("Map Render");

pub fn image_to_bytes(
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    format: ImageFormat,
) -> Result<Vec<u8>, Error> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), format)?;
    Ok(bytes)
}

pub fn map_to_image(
    map: &mut TwMap,
    zoom: f32,
    resolution: Extent2<u32>,
    offset: Vec2<u32>,
    full: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    map.embed_images_auto()?;
    map.images.load()?;
    map.groups
        .load_conditionally(|layer| layer.kind() == LayerKind::Tiles)?;

    println!("Connecting to GPU backend");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..wgpu::InstanceDescriptor::default()
    });
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .block_on()
        .expect("No suitable adapter found");
    let (device, queue) = adapter
        .request_device(&device_descriptor(&adapter), None)
        .block_on()?;

    println!("Uploading data to GPU");
    let mut camera = Camera::new(1.);
    let gpu_camera = GpuCamera::upload(&camera, &device);
    let samplers = Samplers::new(&device);
    let map_static = GpuMapStatic::new(FORMAT, &device);
    let map_data = GpuMapData::upload(map, &device, &queue);
    let map_render = map_static.prepare_render(map, &map_data, &gpu_camera, &samplers, &device);

    println!("Rendering picture as PNG");
    let download_texture = DownloadTexture::new(resolution.w, resolution.h, FORMAT, &device);
    camera.switch_aspect_ratio(resolution.w as f32 / resolution.h as f32);
    let position;
    if full {
        let game_layer = map.find_physics_layer::<twmap::GameLayer>().unwrap();
        let map_size: Extent2<f32> = game_layer.tiles.shape().az();
        let map_middle = map_size / 2.;
        position = (
            map_middle.w + offset.x as f32,
            map_middle.h + offset.y as f32,
        );
        let zoom_sizes: Vec2<f32> = Vec2::<f32>::from(map_size) / camera.base_dimensions;
        camera.zoom = (zoom_sizes.x.max(zoom_sizes.y) * zoom).into();
    } else {
        position = (offset.x as f32, offset.y as f32);
        camera.zoom = [zoom, zoom].into();
    }
    camera.position = position.into();
    gpu_camera.update(&camera, &queue);
    map_data.update(map, &camera, resolution.into(), 0, 0, &queue);
    let view = download_texture.texture_view();
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: LABEL });
    {
        let render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: LABEL,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        let mut tw_render_pass = TwRenderPass::new(render_pass, resolution.into(), &camera);
        map_render.render_background(&mut tw_render_pass);
        map_render.render_foreground(&mut tw_render_pass);
    }
    queue.submit(iter::once(command_encoder.finish()));

    let image = download_texture.download_rgba(&device, &queue);
    println!("Render complete");
    Ok(image)
}
