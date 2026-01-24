use bevy::prelude::*;

#[derive(Resource, Default, Clone)]
pub struct LandMask {
    pub mask: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl LandMask {
    pub fn is_land(&self, lon: f32, lat: f32) -> bool {
        if self.width == 0 || self.height == 0 { return false; }
        
        let x_proj = lon.to_radians();
        let y_proj = (lat.to_radians().sin()).atanh();

        let u = ((x_proj + std::f32::consts::PI) / (2.0 * std::f32::consts::PI) * self.width as f32) as u32;
        let v = ((std::f32::consts::PI - y_proj) / (2.0 * std::f32::consts::PI) * self.height as f32) as u32;

        if u < self.width && v < self.height {
            self.mask[(v * self.width + u) as usize]
        } else {
            false
        }
    }
}

#[derive(Resource)]
pub struct MapImageHandle(pub Handle<Image>);

#[derive(Resource)]
pub struct LandMaskLoaded;

pub fn generate_land_mask_system(
    mut commands: Commands,
    handle: Res<MapImageHandle>,
    images: Res<Assets<Image>>,
    mut land_mask: ResMut<LandMask>,
) {
    if let Some(image) = images.get(&handle.0) {
        let width = image.texture_descriptor.size.width;
        let height = image.texture_descriptor.size.height;
        let mut mask = vec![false; (width * height) as usize];

        for y in 0..height {
            for x in 0..width {
                let i = ((y * width + x) * 4) as usize;
                if i + 3 < image.data.len() {
                    let a = image.data[i + 3];
                    if a > 128 {
                        mask[(y * width + x) as usize] = true;
                    }
                }
            }
        }

        land_mask.mask = mask;
        land_mask.width = width;
        land_mask.height = height;

        commands.insert_resource(LandMaskLoaded);
        commands.remove_resource::<MapImageHandle>();
        info!("Global Land mask generated: {}x{}", width, height);
    }
}
