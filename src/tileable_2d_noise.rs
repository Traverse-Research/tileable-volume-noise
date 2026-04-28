use crate::glm_functions::glm_perlin_vec4;
use glam::{Vec2, Vec4};

pub struct Tileable2dNoise;
impl Tileable2dNoise {
    /// FBM Perlin noise tiling on the unit square. Output is in `[0, 1]`,
    /// matching the convention of [`crate::Tileable3dNoise::perlin_noise`].
    pub fn perlin_noise(p: Vec2, frequency: f32, octave_count: u32) -> f32 {
        let noise = (Self::perlin_potential(p, frequency, octave_count)
            / Self::weight_sum(octave_count))
            * 0.5
            + 0.5;
        noise.clamp(0.0, 1.0)
    }

    /// Tileable 2D curl noise. Returns a divergence-free vector field
    /// `(∂ψ/∂y, −∂ψ/∂x)` of an FBM Perlin scalar potential ψ that tiles on
    /// the unit square.
    ///
    /// Magnitude scales with `frequency` and is not normalized; callers that
    /// bake this into a texture should rescale to fit the target range.
    pub fn curl_noise(p: Vec2, frequency: f32, octave_count: u32) -> Vec2 {
        // Central differences. `glm_perlin_vec4` already wraps integer lattice
        // coordinates by `rep`, so `p ± eps` straddling 0 or 1 stays tileable.
        let eps = 1.0 / 1024.0;
        let dx = Vec2::new(eps, 0.0);
        let dy = Vec2::new(0.0, eps);

        let psi_xp = Self::perlin_potential(p + dx, frequency, octave_count);
        let psi_xn = Self::perlin_potential(p - dx, frequency, octave_count);
        let psi_yp = Self::perlin_potential(p + dy, frequency, octave_count);
        let psi_yn = Self::perlin_potential(p - dy, frequency, octave_count);

        let inv_2eps = 1.0 / (2.0 * eps);
        let dpsi_dx = (psi_xp - psi_xn) * inv_2eps;
        let dpsi_dy = (psi_yp - psi_yn) * inv_2eps;

        Vec2::new(dpsi_dy, -dpsi_dx)
    }

    /// Signed FBM Perlin potential. Same octave/weight schedule as the 3D
    /// version, but evaluated in 2D by zeroing the z and w components of
    /// `glm_perlin_vec4` and tying the rep period to `frequency` only on the
    /// active axes.
    fn perlin_potential(p: Vec2, mut frequency: f32, octave_count: u32) -> f32 {
        let octaves_freq_factor = 2.0;

        let mut sum = 0.0;
        let mut weight = 0.5;

        for _ in 0..octave_count {
            let point = p * frequency;
            let val = glm_perlin_vec4(
                Vec4::new(point.x, point.y, 0.0, 0.0),
                Vec4::new(frequency, frequency, 1.0, 1.0),
            );

            sum += val * weight;

            weight *= weight;
            frequency *= octaves_freq_factor;
        }

        sum
    }

    fn weight_sum(octave_count: u32) -> f32 {
        let mut weight = 0.5;
        let mut sum = 0.0;
        for _ in 0..octave_count {
            sum += weight;
            weight *= weight;
        }
        sum
    }
}
