use imgui::Ui;

pub fn draw(ui: &Ui) {
    ui.window("Preferences").build(|| {
        ui.text("Placeholder for Preferences UI");
    });
}
