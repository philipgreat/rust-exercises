
use fltk::{app,enums::Font, button::Button, input::MultilineInput, frame::Frame, prelude::*, window::Window};
//For examples
//http://seriss.com/people/erco/fltk/#Fl_JPG_Image
//https://github.com/fltk-rs/fltk-rs/blob/master/fltk/examples
fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    //let font = app.load_font("yahei.ttf").unwrap();
    //println!("Found font {:?}", font);
    //let font_sim_hei = app.load_font("SimHei.ttf").unwrap();
    //println!("Found font {:?}", font_sim_hei);
    
    let mut wind = Window::new(100, 100, 800, 600, "终端控制程序");
    let mut frame = Frame::new(0, 0, 400, 200, "点击一下子");
    let mut frame2 = MultilineInput::new(300, 200, 400, 300, "123中文123");
    //frame.set_label_font(Font::by_name(&font));
    //frame2.set_text_font(Font::Helvetica);
    frame2.set_text_size(100);
    let mut but = Button::new(160, 210, 80, 40, "点击一下子");
    let mut but2 = Button::new(160, 260, 80, 40, "点击一下子");
    
    //let mut but3 = Butt
    wind.end();
    //but.set_label_font(Font::by_name(&font));
    //but2.set_label_font(Font::by_name(&font_sim_hei));


    wind.show();
    but.set_callback(move |_| frame.set_label("Hello World!"));
    but2.set_callback(move |_| frame2.set_value("ddd你好"));
    //but2.set_callback(move |_| frame.set_label("你好"));
    
    app.run().unwrap();
}

/*


序号


批次号码


产品名称


物料编码


批次日期


批次数量


操作


1


2021072502


液态氮(99.999%.150L杜瓦罐)


31.03.1245


2021/07/25


8


2


2021072502


工业氧(99.5%.10L.150bar)


31.03.1001


2021/07/25


16


3


2021072502


纯氩(99.99%.40L.150bar)


31.03.1011


2021/07/25


14


4


2021072502


医用氧(99.5% 40L 150bar)


31.03.1060


2021/07/25


5


5


2021072502


医用氧(99.5% 10L 150bar)


31.03.1054


2021/07/25


5


6


2021072502


高纯氩(99.999%.40L.150bar)


31.03.1017


2021/07/25


9


7


2021072502


工业氧瓶组(99.5%.40L*15瓶.150bar)


31.03.1004


2021/07/25


7



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

序号


客户编号


客户名称


盘点人


盘点人工号


审核处理


盘点时间


操作


1


000001.00


广州眼科医院


周廷志


00235


是


2021/08/07


2


000016.00


爱普生技术(深圳)有限公司


周廷志


00235


是


2021/08/03


3


000022.00


奥林巴斯(深圳)有限公司


周廷志


00235


否


2021/08/01


4


000040.00


本田汽车(中国)有限公司


周廷志


00235


是


2021/07/26


5


000044.00


深圳博爱医院


周廷志


00235


是


2021/06/21


6


000047.00


博思格建筑结构有限公司


周廷志


00235


是


2019/08/23


7


000062.00


戴德科技(东莞)有限公司


周廷志


00235


是


2021/07/25


8


000235.00


广东省口腔医院


周廷志


00235


是


2021/08/02


9


000235.01


广东省口腔医院-总务科


周廷志


00235


是


2021/08/05



*/


