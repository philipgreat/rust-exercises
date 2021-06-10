use actix_web::{error, middleware, get, web, App, HttpServer, Responder,HttpResponse,HttpRequest};

use actix_web::http::header::ContentType;

use qrcode::{QrCode, Version, EcLevel};
use image::Luma;
use image::DynamicImage;
use image::ImageOutputFormat;
use qrcode::render::svg;
use qrcode::render::unicode;
use std::io::Cursor;


fn image(){

    let code = QrCode::new(b"https://www.google.com/").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();


}



#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/qrcode/")]
async fn genqrcode(_req: HttpRequest)  -> HttpResponse {

    let code = QrCode::new(b"https://www.google.com/").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    //let png: ImageBuffer<Luma<u8>, Vec<u8>> = qr.render::<Luma<u8>>().build();
    // Save the image.
    //image.save("/tmp/qrcode.png").unwrap();
    let mut w = Cursor::new(Vec::new());
    DynamicImage::ImageLuma8(image)
        .write_to(&mut w, ImageOutputFormat::Png)
        .unwrap();
    HttpResponse::Ok()
        
        .set(ContentType::png())
        //.plaintext()
        .body(w.into_inner())
        

    //format!("hello")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new().wrap(middleware::Logger::default())
            .service(index).service(genqrcode))
        .bind("127.0.0.1:9090")?
        .run()
        .await
}