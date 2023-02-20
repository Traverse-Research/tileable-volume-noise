use glam::{DVec4, Vec2, Vec3, Vec4};

pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

pub(crate) fn glm_mod_3(x: Vec3, mod_val: Vec3) -> Vec3 {
    x - mod_val * (x / mod_val).floor()
}

fn glm_mod_4(x: Vec4, mod_val: Vec4) -> Vec4 {
    x - mod_val * (x / mod_val).floor()
}

fn glm_mod_289(x: Vec4) -> Vec4 {
    x - (x * Vec4::splat(1.0 / 289.0)).floor() * Vec4::splat(289.0)
}

fn glm_permute(x: Vec4) -> Vec4 {
    glm_mod_289((x * Vec4::splat(34.0) + Vec4::ONE) * x)
}

fn glm_step(edge: Vec4, x: Vec4) -> Vec4 {
    Vec4::select(x.cmplt(edge), Vec4::ZERO, Vec4::ONE)
}

fn taylor_inv_sqrt(x: Vec4) -> Vec4 {
    let double_vec = DVec4::splat(1.79284291400159f64) - 0.85373472095314f64 * x.as_dvec4();
    double_vec.as_vec4()
}

fn glm_fade(x: Vec4) -> Vec4 {
    (x * x * x) * (x * (x * Vec4::splat(6.0) - Vec4::splat(15.0)) + Vec4::splat(10.0))
}

// From https://github.com/g-truc/glm/blob/master/glm/gtc/noise.inl
pub(crate) fn glm_perlin_vec4(p: Vec4, rep: Vec4) -> f32 {
    let pi0 = glm_mod_4(p.floor(), rep); // Integer part modulo rep
    let pi1 = glm_mod_4(pi0 + Vec4::ONE, rep); // Integer part + 1 mod rep
    let pf0 = p.fract(); // Fractional part for interpolation
    let pf1 = pf0 - Vec4::ONE; // Fractional part - 1.0
    let ix = Vec4::new(pi0.x, pi1.x, pi0.x, pi1.x);
    let iy = Vec4::new(pi0.y, pi0.y, pi1.y, pi1.y);
    let iz0 = Vec4::splat(pi0.z);
    let iz1 = Vec4::splat(pi1.z);
    let iw0 = Vec4::splat(pi0.w);
    let iw1 = Vec4::splat(pi1.w);

    let ixy = glm_permute(glm_permute(ix) + iy);
    let ixy0 = glm_permute(ixy + iz0);
    let ixy1 = glm_permute(ixy + iz1);
    let ixy00 = glm_permute(ixy0 + iw0);
    let ixy01 = glm_permute(ixy0 + iw1);
    let ixy10 = glm_permute(ixy1 + iw0);
    let ixy11 = glm_permute(ixy1 + iw1);

    let mut gx00 = ixy00 / Vec4::splat(7.0);
    let mut gy00 = gx00.floor() / Vec4::splat(7.0);
    let mut gz00 = gy00.floor() / Vec4::splat(6.0);
    gx00 = gx00.fract() - Vec4::splat(0.5);
    gy00 = gy00.fract() - Vec4::splat(0.5);
    gz00 = gz00.fract() - Vec4::splat(0.5);
    let gw00 = Vec4::splat(0.75) - gx00.abs() - gy00.abs() - gz00.abs();
    let sw00 = glm_step(gw00, Vec4::ZERO);
    gx00 -= sw00 * (glm_step(Vec4::ZERO, gx00) - Vec4::splat(0.5));
    gy00 -= sw00 * (glm_step(Vec4::ZERO, gy00) - Vec4::splat(0.5));

    let mut gx01 = ixy01 / Vec4::splat(7.0);
    let mut gy01 = gx01.floor() / Vec4::splat(7.0);
    let mut gz01 = gy01.floor() / Vec4::splat(6.0);
    gx01 = gx01.fract() - Vec4::splat(0.5);
    gy01 = gy01.fract() - Vec4::splat(0.5);
    gz01 = gz01.fract() - Vec4::splat(0.5);
    let gw01 = Vec4::splat(0.75) - gx01.abs() - gy01.abs() - gz01.abs();
    let sw01 = glm_step(gw01, Vec4::ZERO);
    gx01 -= sw01 * (glm_step(Vec4::ZERO, gx01) - Vec4::splat(0.5));
    gy01 -= sw01 * (glm_step(Vec4::ZERO, gy01) - Vec4::splat(0.5));

    let mut gx10 = ixy10 / Vec4::splat(7.0);
    let mut gy10 = gx10.floor() / Vec4::splat(7.0);
    let mut gz10 = gy10.floor() / Vec4::splat(6.0);
    gx10 = gx10.fract() - Vec4::splat(0.5);
    gy10 = gy10.fract() - Vec4::splat(0.5);
    gz10 = gz10.fract() - Vec4::splat(0.5);
    let gw10 = Vec4::splat(0.75) - gx10.abs() - gy10.abs() - gz10.abs();
    let sw10 = glm_step(gw10, Vec4::ZERO);
    gx10 -= sw10 * (glm_step(Vec4::ZERO, gx10) - Vec4::splat(0.5));
    gy10 -= sw10 * (glm_step(Vec4::ZERO, gy10) - Vec4::splat(0.5));

    let mut gx11 = ixy11 / Vec4::splat(7.0);
    let mut gy11 = gx11.floor() / Vec4::splat(7.0);
    let mut gz11 = gy11.floor() / Vec4::splat(6.0);
    gx11 = gx11.fract() - Vec4::splat(0.5);
    gy11 = gy11.fract() - Vec4::splat(0.5);
    gz11 = gz11.fract() - Vec4::splat(0.5);
    let gw11 = Vec4::splat(0.75) - gx11.abs() - gy11.abs() - gz11.abs();
    let sw11 = glm_step(gw11, Vec4::ZERO);
    gx11 -= sw11 * (glm_step(Vec4::ZERO, gx11) - Vec4::splat(0.5));
    gy11 -= sw11 * (glm_step(Vec4::ZERO, gy11) - Vec4::splat(0.5));

    let mut g0000 = Vec4::new(gx00.x, gy00.x, gz00.x, gw00.x);
    let mut g1000 = Vec4::new(gx00.y, gy00.y, gz00.y, gw00.y);
    let mut g0100 = Vec4::new(gx00.z, gy00.z, gz00.z, gw00.z);
    let mut g1100 = Vec4::new(gx00.w, gy00.w, gz00.w, gw00.w);
    let mut g0010 = Vec4::new(gx10.x, gy10.x, gz10.x, gw10.x);
    let mut g1010 = Vec4::new(gx10.y, gy10.y, gz10.y, gw10.y);
    let mut g0110 = Vec4::new(gx10.z, gy10.z, gz10.z, gw10.z);
    let mut g1110 = Vec4::new(gx10.w, gy10.w, gz10.w, gw10.w);
    let mut g0001 = Vec4::new(gx01.x, gy01.x, gz01.x, gw01.x);
    let mut g1001 = Vec4::new(gx01.y, gy01.y, gz01.y, gw01.y);
    let mut g0101 = Vec4::new(gx01.z, gy01.z, gz01.z, gw01.z);
    let mut g1101 = Vec4::new(gx01.w, gy01.w, gz01.w, gw01.w);
    let mut g0011 = Vec4::new(gx11.x, gy11.x, gz11.x, gw11.x);
    let mut g1011 = Vec4::new(gx11.y, gy11.y, gz11.y, gw11.y);
    let mut g0111 = Vec4::new(gx11.z, gy11.z, gz11.z, gw11.z);
    let mut g1111 = Vec4::new(gx11.w, gy11.w, gz11.w, gw11.w);

    let norm00 = taylor_inv_sqrt(Vec4::new(
        g0000.dot(g0000),
        g0100.dot(g0100),
        g1000.dot(g1000),
        g1100.dot(g1100),
    ));
    g0000 *= norm00.x;
    g0100 *= norm00.y;
    g1000 *= norm00.z;
    g1100 *= norm00.w;

    let norm01 = taylor_inv_sqrt(Vec4::new(
        g0001.dot(g0001),
        g0101.dot(g0101),
        g1001.dot(g1001),
        g1101.dot(g1101),
    ));
    g0001 *= norm01.x;
    g0101 *= norm01.y;
    g1001 *= norm01.z;
    g1101 *= norm01.w;

    let norm10 = taylor_inv_sqrt(Vec4::new(
        g0010.dot(g0010),
        g0110.dot(g0110),
        g1010.dot(g1010),
        g1110.dot(g1110),
    ));
    g0010 *= norm10.x;
    g0110 *= norm10.y;
    g1010 *= norm10.z;
    g1110 *= norm10.w;

    let norm11 = taylor_inv_sqrt(Vec4::new(
        g0011.dot(g0011),
        g0111.dot(g0111),
        g1011.dot(g1011),
        g1111.dot(g1111),
    ));
    g0011 *= norm11.x;
    g0111 *= norm11.y;
    g1011 *= norm11.z;
    g1111 *= norm11.w;

    let n0000 = g0000.dot(pf0);
    let n1000 = g1000.dot(Vec4::new(pf1.x, pf0.y, pf0.z, pf0.w));
    let n0100 = g0100.dot(Vec4::new(pf0.x, pf1.y, pf0.z, pf0.w));
    let n1100 = g1100.dot(Vec4::new(pf1.x, pf1.y, pf0.z, pf0.w));
    let n0010 = g0010.dot(Vec4::new(pf0.x, pf0.y, pf1.z, pf0.w));
    let n1010 = g1010.dot(Vec4::new(pf1.x, pf0.y, pf1.z, pf0.w));
    let n0110 = g0110.dot(Vec4::new(pf0.x, pf1.y, pf1.z, pf0.w));
    let n1110 = g1110.dot(Vec4::new(pf1.x, pf1.y, pf1.z, pf0.w));
    let n0001 = g0001.dot(Vec4::new(pf0.x, pf0.y, pf0.z, pf1.w));
    let n1001 = g1001.dot(Vec4::new(pf1.x, pf0.y, pf0.z, pf1.w));
    let n0101 = g0101.dot(Vec4::new(pf0.x, pf1.y, pf0.z, pf1.w));
    let n1101 = g1101.dot(Vec4::new(pf1.x, pf1.y, pf0.z, pf1.w));
    let n0011 = g0011.dot(Vec4::new(pf0.x, pf0.y, pf1.z, pf1.w));
    let n1011 = g1011.dot(Vec4::new(pf1.x, pf0.y, pf1.z, pf1.w));
    let n0111 = g0111.dot(Vec4::new(pf0.x, pf1.y, pf1.z, pf1.w));
    let n1111 = g1111.dot(pf1);

    let fade_xyzw = glm_fade(pf0);
    let n_0w = Vec4::new(n0000, n1000, n0100, n1100)
        .lerp(Vec4::new(n0001, n1001, n0101, n1101), fade_xyzw.w);
    let n_1w = Vec4::new(n0010, n1010, n0110, n1110)
        .lerp(Vec4::new(n0011, n1011, n0111, n1111), fade_xyzw.w);

    let n_zw = n_0w.lerp(n_1w, fade_xyzw.z);
    let n_yzw = Vec2::new(n_zw.x, n_zw.y).lerp(Vec2::new(n_zw.z, n_zw.w), fade_xyzw.y);
    let n_xyzw = lerp(n_yzw.x, n_yzw.y, fade_xyzw.x);

    2.2 * n_xyzw
}
