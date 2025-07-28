use bevy::prelude::*;
use bevy_egui::{EguiPlugin, egui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_ctx: bevy_egui::EguiContexts) {
    egui::Window::new("Hello egui").show(egui_ctx.ctx_mut(), |ui| {
        ui.label("This is a UI window");
    });
}
