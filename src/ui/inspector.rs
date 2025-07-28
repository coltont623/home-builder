use imgui::Ui;

pub fn draw(ui: &Ui) {
    ui.window("Inspector").build(|| {
        ui.text("Placeholder for Inspector UI");
    });
}
