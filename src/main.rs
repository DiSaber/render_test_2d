use render::{RenderApp, winit::window::WindowAttributes};

fn main() {
    let mut before = |_| {};
    let mut after = |_| {};

    let mut render_app = RenderApp::new(&mut before, &mut after).with_window_attributes(
        WindowAttributes::default()
            .with_title("Render Test 2d")
            .with_resizable(true),
    );

    render_app.run_app().unwrap();
}
