use crate::glm_functions::{glm_mod_3, glm_perlin_vec4, lerp};
use glam::{Vec3, Vec4};

pub struct Tileable3dNoise;
impl Tileable3dNoise {
    fn hash(n: f32) -> f32 {
        let x = (n + 1.951f32).sin() * 43758.547f32;
        // Original implementation uses `glm::fract`, which is implemented as `x - floor(x)`
        // On the contrary, `fract` on the `f32` type is implemented as `x - x.trunc()`
        // This is vital to achieve correct results.
        x - x.floor()
    }

    fn noise(x: Vec3) -> f32 {
        let p = x.floor();
        let f = x - x.floor();

        let f = f * f * (Vec3::splat(3.0) - Vec3::splat(2.0) * f);
        let n = p.x + p.y * 57.0f32 + 113.0f32 * p.z;

        lerp(
            lerp(
                lerp(Self::hash(n + 0.0f32), Self::hash(n + 1.0f32), f.x),
                lerp(Self::hash(n + 57.0f32), Self::hash(n + 58.0f32), f.x),
                f.y,
            ),
            lerp(
                lerp(Self::hash(n + 113.0f32), Self::hash(n + 114.0f32), f.x),
                lerp(Self::hash(n + 170.0f32), Self::hash(n + 171.0f32), f.x),
                f.y,
            ),
            f.z,
        )
    }

    fn cells(p: Vec3, cell_count: f32) -> f32 {
        let p_cell = p * cell_count;
        let mut d = 1.0e10f32;

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let tp = p_cell.floor() + Vec3::new(x as f32, y as f32, z as f32);
                    let tp = p_cell - tp - Self::noise(glm_mod_3(tp, Vec3::splat(cell_count)));

                    d = d.min(tp.dot(tp));
                }
            }
        }

        d.clamp(0.0, 1.0)
    }

    pub fn worley_noise(p: Vec3, cell_count: f32) -> f32 {
        Self::cells(p, cell_count)
    }

    pub fn perlin_noise(p: Vec3, mut frequency: f32, octave_count: u32) -> f32 {
        let octaves_freq_factor = 2.0; // noise frequency factor between octave, forced to 2

        // Compute the sum for each octave
        let mut sum = 0.0;
        let mut weight_sum = 0.0;
        let mut weight = 0.5;

        // TODO: Consider implementing `glm_perlin` to remove dependency on noise crate?
        for _ in 0..octave_count {
            let point = p * frequency;
            let val = glm_perlin_vec4(
                Vec4::new(point.x, point.y, point.z, 0.0),
                Vec4::splat(frequency),
            );

            sum += val * weight;
            weight_sum += weight;

            weight *= weight;
            frequency *= octaves_freq_factor;
        }

        let noise = (sum / weight_sum) * 0.5 + 0.5;
        noise.clamp(0.0, 1.0)
    }
}
