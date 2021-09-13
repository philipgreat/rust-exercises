
use fltk::{app,enums::Font, button::Button, input::MultilineInput, frame::Frame, prelude::*, window::Window};
//For examples
//http://seriss.com/people/erco/fltk/#Fl_JPG_Image

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let font = app.load_font("yahei.ttf").unwrap();
    println!("Found font {:?}", font);
    let font_sim_hei = app.load_font("SimHei.ttf").unwrap();
    println!("Found font {:?}", font_sim_hei);
    
    let mut wind = Window::new(100, 100, 800, 600, "终端控制程序");
    let mut frame = Frame::new(0, 0, 400, 200, "点击一下子");
    let mut frame2 = MultilineInput::new(300, 200, 400, 300, "123123");
    frame.set_label_font(Font::by_name(&font));
    frame2.set_text_font(Font::by_name(&font_sim_hei));
    frame2.set_text_size(100);
    let mut but = Button::new(160, 210, 80, 40, "点击一下子");
    let mut but2 = Button::new(160, 260, 80, 40, "点击一下子");
    
    //let mut but3 = Butt
    wind.end();
    but.set_label_font(Font::by_name(&font));
    but2.set_label_font(Font::by_name(&font_sim_hei));


    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!"));
    but2.set_callback(move |_| frame2.set_value("ddd你好"));
    //but2.set_callback(move |_| frame.set_label("你好"));
    
    app.run().unwrap();
}

/*




use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(800, 600)
        .with_label("Webview");
    let mut wv_win = window::Window::default()
        .with_size(790, 590)
        .center_of_parent();
    win.end();
    win.show();
    
    let mut wv = fltk_webview::Webview::from(false, &mut wv_win);
    wv.navigate("http://wikipedia.com");
    
    wv.run();
}

*/


