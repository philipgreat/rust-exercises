//use actix_web::{error, middleware, get, web, App, HttpServer, Responder,HttpResponse,HttpRequest};
use actix_web::http::header::ContentType;
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use urlencoding::decode;

//use qrcode::{QrCode, Version, EcLevel};
use qrcode::QrCode;

use image::DynamicImage;
use image::ImageFormat;
use image::Luma;
use qrcode::render::svg;
//use qrcode::render::unicode;
use std::io::Cursor;

fn serve_svg(content: String) -> HttpResponse {
    //let code = QrCode::with_version(content, Version::Micro(2), EcLevel::H).unwrap();
    //let code = QrCode::with_version(content.as_bytes(), Version::Micro(2), EcLevel::L).unwrap();
    let code = QrCode::new(content).unwrap();
    //let code = QrCode::with_version(b"099999", Version::Micro(2), EcLevel::L).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();

    HttpResponse::Ok()
        //.set(ContentType::svg())
        .content_type("image/svg+xml")
        //.plaintext()
        .body(image)
}

fn serve_png(content: String) -> HttpResponse {
    let code = QrCode::new(content).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    //let png: ImageBuffer<Luma<u8>, Vec<u8>> = qr.render::<Luma<u8>>().build();
    // Save the image.
    //image.save("/tmp/qrcode.png").unwrap();
    let mut w = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut w, ImageFormat::Png)
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::png())
        //.plaintext()
        .body(w.into_inner())
}

#[get("/qrsvc/{imagetype}/{content}/")]
async fn qrsvc(path: web::Path<(String, String)>) -> HttpResponse {
    let (imagetype, content) = path.into_inner();
    let decodedcontent = decode(&content);
    if imagetype == "svg" {
        return serve_svg(decodedcontent.unwrap().into_owned());
    }
    return serve_png(decodedcontent.unwrap().into_owned());

    //format!("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(qrsvc)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
