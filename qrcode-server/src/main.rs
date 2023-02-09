//use actix_web::{error, middleware, get, web, App, HttpServer, Responder,HttpResponse,HttpRequest};
use actix_web::{middleware, get, web, App, HttpServer, Responder,HttpResponse,HttpRequest};
use urlencoding::decode;
use actix_web::http::header::ContentType;

//use qrcode::{QrCode, Version, EcLevel};
use qrcode::{QrCode};

use image::Luma;
use image::DynamicImage;
use image::ImageOutputFormat;
use qrcode::render::svg;
//use qrcode::render::unicode;
use std::io::Cursor;



fn serve_svg(content: String)  -> HttpResponse {

    //let code = QrCode::with_version(content, Version::Micro(2), EcLevel::H).unwrap();
    //let code = QrCode::with_version(content.as_bytes(), Version::Micro(2), EcLevel::L).unwrap();
    let code = QrCode::new(content).unwrap();
    //let code = QrCode::with_version(b"099999", Version::Micro(2), EcLevel::L).unwrap();
    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        
        .build();

    
    HttpResponse::Ok()
        
        //.set(ContentType::svg())
        .header("Content-Type", "image/svg+xml")
        //.plaintext()
        .body(image)
    


}


 fn serve_png(content: String)  -> HttpResponse {

    let code = QrCode::new(content).unwrap();

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
    


}





#[get("/qrsvc/{imagetype}/{content}/")]
async fn qrsvc(web::Path((imagetype, content)): web::Path<(String, String)>)  -> HttpResponse {


    let decodedcontent = decode(&content);
    
    if imagetype=="svg" {
        return serve_svg(decodedcontent.unwrap().into_owned());
    }
    return serve_png(decodedcontent.unwrap().into_owned());

    //format!("hello")
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new().wrap(middleware::Logger::default())
            .service(index).service(genqrcode).service(qrsvc))
        .bind("0.0.0.0:9090")?
        .run()
        .await
}

/*




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



fn image(){

    let code = QrCode::new(b"https://www.google.com/").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();


}


*/