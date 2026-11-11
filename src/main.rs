use gpui::*;

struct HexApp {
    title: String,
}

impl Render for HexApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x090a0f))
            .text_color(rgb(0xf4f4f5))
            .child(format!("HEX Autonomous AI Workbench: {}", self.title))
    }
}

fn main() {
    Application::new().run(|cx| {
        let options = WindowOptions::default();
        cx.open_window(options, |_, cx| {
            cx.new(|_| HexApp {
                title: "Cockpit".into(),
            })
        })
        .unwrap();
    });
}
