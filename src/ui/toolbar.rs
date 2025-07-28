use imgui::Ui;

pub fn draw(ui: &Ui) {
    ui.window("Toolbar").build(|| {
        ui.text("Placeholder for Toolbar UI");
    });
}
