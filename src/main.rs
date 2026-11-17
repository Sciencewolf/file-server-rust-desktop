mod api;
mod app;
mod components;
mod progress;
mod state;
mod util;

use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

fn main() {
    dotenvy::dotenv().ok();

    let window = WindowBuilder::new()
        .with_title("File Server")
        .with_inner_size(LogicalSize::new(960.0, 720.0));

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window))
        .launch(app::App);
}
