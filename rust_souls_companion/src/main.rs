use crate::companion::Companion;

mod companion;
mod character;
mod player;
mod game;

fn main() {
    let win_option = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: Some(eframe::IconData::try_from_png_bytes(&include_bytes!("../assets/icon.png")[..]).unwrap()),
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
        resizable: false,
        transparent: false,
        mouse_passthrough: false,
        active: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: Default::default(),
        follow_system_theme: cfg!(target_os = "macos") || cfg!(target_os = "windows"),
        default_theme: eframe::Theme::Dark,
        run_and_return: true,
        event_loop_builder: None,
        shader_version: None,
        centered: false,
        app_id: None,
    };

    eframe::run_native("Rust Souls Companion", win_option, Box::new(|cc| Box::new(Companion::new(cc)))).expect("Failed to open app.");
}