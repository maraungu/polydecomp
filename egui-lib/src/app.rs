use eframe::{egui, epi};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DecompApp {
    load_poly: bool,
    draw_poly: bool,
    triangulate: bool,
    decompose: bool,
}

impl Default for DecompApp {
    fn default() -> Self {
        Self {
            load_poly: false,
            draw_poly: false,
            triangulate: false,
            decompose: false,
        }
    }
}

impl epi::App for DecompApp {
    fn name(&self) -> &str {
        "Convex decomposition for simple polygons"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default();
        }
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180).into()
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            load_poly,
            draw_poly,
            triangulate,
            decompose,
        } = self;


        
        egui::SidePanel::left("side_panel")
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(255, 255, 255)))
            .show(ctx, |ui| {
                ui.heading("POLYDECOMP");
                ui.separator();


                ui.horizontal(|ui| {
                    ui.heading("polygon");
                    
                    if ui.button("load").clicked() {

                    }
                    if ui.button("draw").clicked() {
                        
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("triangulation");
                    if ui.button("show").clicked() {

                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("decomposition");
                    if ui.button("show").clicked() {
                        
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.small("powered by ");
                        ui.add(egui::Hyperlink::new("https://github.com/emilk/egui")
                          .text("egui").small());
                        ui.small(" and ");
                        ui.add(egui::Hyperlink::new("https://github.com/emilk/egui/tree/master/eframe")
                          .text("eframe").small());
                    });
                });
            });

        
    }
}
