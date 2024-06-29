use bytemuck::{cast_slice, Pod, Zeroable};
use rand::{Rng, thread_rng};
use wgpu::{BindGroup, BindGroupLayout, Extent3d, ImageDataLayout, SamplerBindingType, ShaderStages, StorageTextureAccess, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDimension};
use crate::inbuilt::setup::Setup;
use crate::utility::structs::PingPongData;


pub struct AutomataPackage {
   pub size: Extent3d,
   pub bind_group_layout: BindGroupLayout,
   pub bind_groups: PingPongData<BindGroup>,
}
impl AutomataPackage {
   pub fn new(setup: &Setup, width: u32, height: u32) -> Self {
      let size = Extent3d { width, height, depth_or_array_layers: 1, };

      let texture_1 = setup.device.create_texture(&TextureDescriptor {
            label: Some("texture_1"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R32Float,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
         });
      let texture_2 = setup.device.create_texture(&TextureDescriptor {
         label: Some("texture_1"),
         size,
         mip_level_count: 1,
         sample_count: 1,
         dimension: TextureDimension::D2,
         format: TextureFormat::R32Float,
         usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING,
         view_formats: &[],
      });

      Self::write_texture_data(&setup, &texture_1, size, &Self::generate_random_data(size)); // the first one in the flipper must be written to
      Self::write_texture_data(&setup, &texture_2, size, &Self::generate_random_data(size));

      let view_1 = texture_1.create_view(&wgpu::TextureViewDescriptor::default());
      let view_2 = texture_2.create_view(&wgpu::TextureViewDescriptor::default());


      let sampler = setup.device.create_sampler(&wgpu::SamplerDescriptor {
         address_mode_u: wgpu::AddressMode::ClampToEdge,
         address_mode_v: wgpu::AddressMode::ClampToEdge,
         address_mode_w: wgpu::AddressMode::ClampToEdge,
         mag_filter: wgpu::FilterMode::Nearest,
         min_filter: wgpu::FilterMode::Nearest,
         mipmap_filter: wgpu::FilterMode::Nearest,
         ..Default::default()
      });

      let bind_group_layout = setup.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
          entries: &[
             wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                   access: StorageTextureAccess::ReadWrite,
                   format: TextureFormat::R32Float,
                   view_dimension: TextureViewDimension::D2,
                },
                count: None,
             },
             wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(SamplerBindingType::NonFiltering),
                count: None,
             },
          ],
          label: Some("texture_bind_group_layout"),
      });


      let bind_group_1 = setup.device.create_bind_group(&wgpu::BindGroupDescriptor {
         layout: &bind_group_layout,
         entries: &[
            wgpu::BindGroupEntry {
               binding: 0,
               resource: wgpu::BindingResource::TextureView(&view_1),
            },
            wgpu::BindGroupEntry {
               binding: 1,
               resource: wgpu::BindingResource::Sampler(&sampler),
            }
         ],
         label: Some("diffuse_bind_group"),
      });

      let bind_group_2 = setup.device.create_bind_group(&wgpu::BindGroupDescriptor {
         layout: &bind_group_layout,
         entries: &[
            wgpu::BindGroupEntry {
               binding: 0,
               resource: wgpu::BindingResource::TextureView(&view_2),
            },
            wgpu::BindGroupEntry {
               binding: 1,
               resource: wgpu::BindingResource::Sampler(&sampler),
            }
         ],
         label: Some("diffuse_bind_group"),
      });

      let bind_groups = PingPongData::new(bind_group_1, bind_group_2);

      Self {
         size,
         bind_group_layout,
         bind_groups,
      }
   }

   fn write_texture_data(setup: &Setup, texture: &Texture, size: Extent3d, data: &Vec<Texel> ) {
      let bytes_per_pixel = std::mem::size_of::<Texel>();
      let bytes_per_row = (size.width as usize * bytes_per_pixel) as u32;
      let rows_per_image = size.height;

      setup.queue.write_texture(
         wgpu::ImageCopyTexture {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
         },
         &cast_slice(data),
         ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(bytes_per_row),
            rows_per_image: Some(rows_per_image),
         },
         size,
      );

   }

   fn generate_random_data( size: Extent3d) -> Vec<Texel> {
      let mut rng = thread_rng();
      let mut test_data = vec![];
      for _ in 0..(size.width * size.height) {
         let vel = rng.gen();
         test_data.push(Texel(vel))
      }

      test_data
   }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Texel(f32);