
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(100, 100, 800, 600, "终端控制程序");
    let mut frame = Frame::new(0, 0, 400, 200, "123123");
    let mut but = Button::new(160, 210, 80, 40, "点击一下子");
    let mut but2 = Button::new(160, 260, 80, 40, "点击一下子");
    //let mut but3 = Butt
    wind.end();
    
    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!"));
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


