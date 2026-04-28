//! Run with `cargo test --features images` to write the generated textures
//! to `./example_images/`. Without the `images` feature this is a no-op.

#[cfg(feature = "images")]
#[test]
fn generate_textures() {
    use tileable_volume_noise::TileableCloudNoise;

    let _ = TileableCloudNoise::cloud_shape_and_erosion_texture();
    let _ = TileableCloudNoise::details_texture();
    let _ = TileableCloudNoise::curl_noise_texture();
}
