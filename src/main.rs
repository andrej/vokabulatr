mod app;

fn main() {
    cacao::appkit::App::new(
        "com.andre.vokabulatr", 
        app::VokabulatrApp::new()
    )
        .run();
}