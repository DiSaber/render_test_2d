use render::{RenderApp, winit::window::WindowAttributes};

fn main() {
    let mut render_app = RenderApp::new(None::<fn(_)>, None::<fn(_)>).with_window_attributes(
        WindowAttributes::default()
            .with_title("Render Test 2d")
            .with_resizable(true),
    );

    render_app.run_app().unwrap();
}
