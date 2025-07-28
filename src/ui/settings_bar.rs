use imgui::Ui;

pub fn draw(ui: &Ui) {
    ui.window("Settings Bar").build(|| {
        ui.text("Placeholder for Settings Bar UI");
    });
}
