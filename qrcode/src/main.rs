

use qrcode::{QrCode, Version, EcLevel};
use image::Luma;
use qrcode::render::svg;
use qrcode::render::unicode;

fn main4() {
    let code = QrCode::with_version(b"01234567", Version::Micro(2), EcLevel::L).unwrap();
    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();
    println!("{}", image);
}

fn main5(){

    let code = QrCode::new(b"https://www.google.com/").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();


}
fn main() {
    let code = QrCode::new("https://baidu.com").unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
    
}
