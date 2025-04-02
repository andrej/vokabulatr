use std::sync::RwLock;

pub struct VokabulatrApp {
    window: RwLock<Option<cacao::appkit::window::Window<MainWindow>>>
}

impl VokabulatrApp {
    pub fn new() -> Self {
        Self {
            window: RwLock::new(None)
        }
    }
}

impl cacao::appkit::AppDelegate for VokabulatrApp {
    fn did_finish_launching(&self) {
        cacao::appkit::App::activate();
        let window_config = cacao::appkit::window::WindowConfig::default();
        let window_delegate = MainWindow::new();
        let window = cacao::appkit::window::Window::with(window_config, window_delegate);
        {
            let mut locked_window = self.window.write().unwrap();
            *locked_window = Some(window);
        }
        {
            let locked_window = self.window.read().unwrap();
            locked_window.as_ref().unwrap().show();
        }
    }
}

pub struct MainWindow {
    content: cacao::view::ViewController<ContentView>
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            content: cacao::view::ViewController::new(ContentView::new())
        }
    }
}

impl cacao::appkit::window::WindowDelegate for MainWindow {
    const NAME: &'static str = "VokabulatrMainWindow";
    
    fn did_load(&mut self, window: cacao::appkit::window::Window) {
        window.set_minimum_content_size(400, 400);
        window.set_title("Vokabulatr");
        window.set_content_view_controller(&self.content);
    }

    fn will_close(&self) {
        cacao::appkit::App::terminate();
    }
}

pub struct ContentView {
    question_label: cacao::text::Label,
    answer_text_field: cacao::input::TextField,
    submit_button: cacao::button::Button
}

impl ContentView {
    pub fn new() -> Self {
        Self {
            question_label: cacao::text::Label::default(),
            answer_text_field: cacao::input::TextField::default(),
            submit_button: cacao::button::Button::new("Submit")
        }
    }
}

impl cacao::view::ViewDelegate for ContentView {
    const NAME: &'static str = "VokabulatrMainView";

    fn did_load(&mut self, view: cacao::view::View) {
        self.question_label.set_text("question");
        self.answer_text_field.set_text("answer");
        cacao::layout::Layout::add_subview(&view, &self.question_label);
        cacao::layout::Layout::add_subview(&view, &self.answer_text_field);
        cacao::layout::Layout::add_subview(&view, &self.submit_button);
        let pad = 5;
        cacao::layout::LayoutConstraint::activate(&[
            // everything stretches left and right
            self.question_label.leading.constraint_equal_to(&view.leading).offset(pad),
            self.question_label.trailing.constraint_equal_to(&view.trailing).offset(-pad),
            self.answer_text_field.leading.constraint_equal_to(&view.leading).offset(pad),
            self.answer_text_field.trailing.constraint_equal_to(&view.trailing).offset(-pad),
            self.submit_button.leading.constraint_equal_to(&view.leading).offset(pad),
            self.submit_button.trailing.constraint_equal_to(&view.trailing).offset(-pad),
            // question on top
            self.question_label.top.constraint_equal_to(&view.top).offset(32 + pad),
            self.question_label.bottom.constraint_equal_to(&self.answer_text_field.top).offset(-pad),
            // answer in middle
            self.answer_text_field.bottom.constraint_equal_to(&self.submit_button.top).offset(-pad),
            // button on bottom
            self.submit_button.bottom.constraint_equal_to(&view.bottom).offset(-pad)
        ]);
    }
}