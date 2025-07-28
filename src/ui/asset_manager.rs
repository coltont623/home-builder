use imgui::Ui;

pub fn draw(ui: &Ui) {
    ui.window("Asset Manager").build(|| {
        ui.text("Placeholder for Asset Manager UI");
    });
}
