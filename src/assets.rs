use macroquad::prelude::*;
use miniquad::conf::Icon;

pub async fn load_fonts() -> (Font, Font) {
    let magic_font = load_ttf_font("src/assets/daydream.otf").await.unwrap();
    let font = load_ttf_font("src/assets/upheavtt.ttf").await.unwrap();
    (magic_font, font)
}

pub fn window_conf() -> Conf {
    let bytes = include_bytes!("assets/icon.png");

    let dyn_img = image::load_from_memory(bytes).expect("Nie udało się zdekodować assets/icon.png");

    use image::imageops::FilterType;
    let small_img = image::imageops::resize(&dyn_img, 16, 16, FilterType::Lanczos3);
    let medium_img = image::imageops::resize(&dyn_img, 32, 32, FilterType::Lanczos3);
    let big_img = image::imageops::resize(&dyn_img, 64, 64, FilterType::Lanczos3);

    let mut small = [0u8; 1024];
    let mut medium = [0u8; 4096];
    let mut big = [0u8; 16384];

    small.copy_from_slice(&small_img.into_raw()[..]);
    medium.copy_from_slice(&medium_img.into_raw()[..]);
    big.copy_from_slice(&big_img.into_raw()[..]);

    let icon = Icon { small, medium, big };

    Conf {
        window_title: "Snake".to_owned(),
        window_width: 1350,
        window_height: 800,
        icon: Some(icon),
        ..Default::default()
    }
}
