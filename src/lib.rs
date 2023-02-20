use glam::Vec3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

mod glm_functions;
mod tileable_3d_noise;

pub use tileable_3d_noise::Tileable3dNoise;

pub struct TileableCloudNoise {
    pub data: Vec<u8>,
    pub resolution: u32,
    pub num_channels: u32,
    pub bytes_per_channel: u32,
}

// TODO: Add feature "image" that saves the textures to a folder for correctness checks.

impl TileableCloudNoise {
    fn remap(og_value: f32, og_min: f32, og_max: f32, new_min: f32, new_max: f32) -> f32 {
        new_min + (((og_value - og_min) / (og_max - og_min)) * (new_max - new_min))
    }

    // RGBA8 Unorm
    //
    // R: PerlinWorley noise
    // G: Worley0
    // B: Worley1
    // A: Worley2
    pub fn cloud_shape_and_erosion_texture() -> Self {
        // As SebH mentions in the reference material, frequency values should be reduced if using a smaller resolution.
        let frequence_mul = [2.0f32, 8.0f32, 14.0f32, 20.0f32, 26.0f32, 32.0f32]; // special weight for perlin-worley

        // Cloud base shape (will be used to generate Perlin-Worley noise in the shader)
        // Note: all channels could be combined once here to reduce memory bandwith requirements.

        // !!! If this is reduced, you should also reduce the number of frequencies in the fmb noise  !!!
        let resolution = 128u32;
        let num_channels = 4u32;
        let bytes_per_channel = 1u32;

        let norm_factor = 1.0 / resolution as f32;

        let cloud_base_shape_texels_unpadded = (0..resolution)
            .into_par_iter()
            .flat_map(|s| {
                let mut slice: Vec<u8> = Vec::with_capacity(
                    (resolution * resolution * num_channels * bytes_per_channel) as usize,
                );

                for t in 0..resolution {
                    for r in 0..resolution {
                        let coords = Vec3::new(s as f32, t as f32, r as f32) * norm_factor;

                        // Perlin FBM noise
                        let octave_count = 3u32;
                        let frequency = 8.0f32;

                        let perlin_noise =
                            Tileable3dNoise::perlin_noise(coords, frequency, octave_count);

                        let cell_count = 4f32;
                        let worley_noise_0 = 1.0f32
                            - Tileable3dNoise::worley_noise(coords, cell_count * frequence_mul[0]);
                        let worley_noise_1 = 1.0f32
                            - Tileable3dNoise::worley_noise(coords, cell_count * frequence_mul[1]);
                        let worley_noise_2 = 1.0f32
                            - Tileable3dNoise::worley_noise(coords, cell_count * frequence_mul[2]);

                        let worley_fbm = worley_noise_0 * 0.625f32
                            + worley_noise_1 * 0.25f32
                            + worley_noise_2 * 0.125f32;

                        // Perlin Worley is based on description in GPU Pro 7: Real Time Volumetric Cloudscapes.
                        // However, it is not clear the text and the image are matching: images does not seem to match what the result from the description in text would give.
                        // Also there are a lot of fudge factor in the code, e.g. * 0.2, so it is really up to you to fine the formula you like.

                        // mapping perlin noise in between worley as minimum and 1.0 as maximum (as described in text of p.101 of GPU Pro 7)
                        let perlin_worley = Self::remap(perlin_noise, 0.0, 1.0, worley_fbm, 1.0);

                        // Matches better what figure 4.7 (not the following up text description p.101). Maps worley between newMin as 0 and perlin as maximum.
                        // let perlin_worley = Self::remap(worleyFBM, 0.0, 1.0, 0.0, perlinNoise);

                        let cell_count = 4f32;
                        // let worley_noise_0 =
                        //     1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 1.0f32);
                        let worley_noise_1 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 2.0f32);
                        let worley_noise_2 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 4.0f32);
                        let worley_noise_3 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 8.0f32);
                        let worley_noise_4 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 16.0f32);
                        // cell_count=2 -> half the frequency of texel, we should not go further (with cellCount = 32 and texture size = 64)
                        // let worley_noise_5 =
                        //     1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 32.0f32);

                        // Three frequency of Worley FBM noise
                        let worley_fbm_0 = worley_noise_1 * 0.625f32
                            + worley_noise_2 * 0.25f32
                            + worley_noise_3 * 0.125f32;
                        let worley_fbm_1 = worley_noise_2 * 0.625f32
                            + worley_noise_3 * 0.25f32
                            + worley_noise_4 * 0.125f32;

                        // let worley_fbm_2 = worley_noise_3 * 0.625f32
                        // + worley_noise_4 * 0.25f32
                        // + worley_noise_5 * 0.125f32;

                        // cell_count=4 -> worleyNoise5 is just noise due to sampling frequency=texel frequency. So only take into account 2 frequencies for FBM
                        let worley_fbm_2 = worley_noise_3 * 0.75f32 + worley_noise_4 * 0.25f32;

                        slice.push((perlin_worley * 255.0) as u8);
                        slice.push((worley_fbm_0 * 255.0) as u8);
                        slice.push((worley_fbm_1 * 255.0) as u8);
                        slice.push((worley_fbm_2 * 255.0) as u8);
                    }
                }

                slice
            })
            .collect::<Vec<_>>();

        Self {
            data: cloud_base_shape_texels_unpadded,
            resolution,
            num_channels,
            bytes_per_channel,
        }
    }

    // RGBA8 Unorm
    //
    // R: Worley FBM 0
    // G: Worley FBM 1
    // B: Worley FBM 2
    // A: Unused - Set to 255
    pub fn details_texture() -> Self {
        // Detail texture behing different frequency of Worley noise
        // Note: all channels could be combined once here to reduce memory bandwith requirements.

        let resolution = 32u32;
        let num_channels = 4u32;
        let bytes_per_channel = 1u32;

        let norm_factor = 1.0 / resolution as f32;

        let cloud_detail_texels_unpadded = (0..resolution)
            .into_par_iter()
            .flat_map(|s| {
                let mut slice: Vec<u8> = Vec::with_capacity(
                    (resolution * resolution * num_channels * bytes_per_channel) as usize,
                );

                for t in 0..resolution {
                    for r in 0..resolution {
                        let coords = Vec3::new(s as f32, t as f32, r as f32) * norm_factor;

                        // 3 octaves
                        let cell_count = 2f32;
                        let worley_noise_0 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 1.0f32);
                        let worley_noise_1 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 2.0f32);
                        let worley_noise_2 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 4.0f32);
                        let worley_noise_3 =
                            1.0f32 - Tileable3dNoise::worley_noise(coords, cell_count * 8.0f32);

                        let worley_fbm_0 = worley_noise_0 * 0.625f32
                            + worley_noise_1 * 0.25f32
                            + worley_noise_2 * 0.125f32;
                        let worley_fbm_1 = worley_noise_1 * 0.625f32
                            + worley_noise_2 * 0.25f32
                            + worley_noise_3 * 0.125f32;
                        // cell_count=4 -> worleyNoise4 is just noise due to sampling frequency=texel frequency. So only take into account 2 frequencies for FBM
                        let worley_fbm_2 = worley_noise_2 * 0.75f32 + worley_noise_3 * 0.25f32;

                        // 2 octaves - unused
                        // let worley_noise_0 = 1.0f32 - Tileable3dNoise::worley_noise(coords, 4.0f32);
                        // let worley_noise_1 = 1.0f32 - Tileable3dNoise::worley_noise(coords, 7.0f32);
                        // let worley_noise_2 = 1.0f32 - Tileable3dNoise::worley_noise(coords, 10.0f32);
                        // let worley_noise_3 = 1.0f32 - Tileable3dNoise::worley_noise(coords, 13.0f32);
                        // let worley_fbm_0 = worley_noise_0 * 0.75f32 + worley_noise_1 * 0.25f32;
                        // let worley_fbm_1 = worley_noise_1 * 0.75f32 + worley_noise_2 * 0.25f32;
                        // let worley_fbm_2 = worley_noise_2 * 0.75f32 + worley_noise_3 * 0.25f32;

                        slice.push((worley_fbm_0 * 255.0) as u8);
                        slice.push((worley_fbm_1 * 255.0) as u8);
                        slice.push((worley_fbm_2 * 255.0) as u8);
                        slice.push(255u8);
                    }
                }

                slice
            })
            .collect::<Vec<_>>();

        Self {
            data: cloud_detail_texels_unpadded,
            resolution,
            num_channels,
            bytes_per_channel,
        }
    }
}
