use eframe::egui::{Color32, Stroke};
use eframe::{egui, epi};

use crate::draw::PolyDraw;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DecompApp {
    load_poly: bool,
    selected_poly: String,
    poly_list: Vec<String>,
    draw_poly: bool,
    drawing_app: PolyDraw,
    triangulate: bool,
    decompose: bool,
}

impl Default for DecompApp {
    fn default() -> Self {
        Self {
            load_poly: false,
            selected_poly: "select a polygon".to_string(),
            poly_list: vec![
                "select a polygon".to_string(),
                "polygon1".to_string(),
                "polygon2".to_string(),
            ],
            draw_poly: false,
            drawing_app: PolyDraw::default(),
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
            selected_poly,
            poly_list,
            draw_poly,
            // drawing_app,
            triangulate,
            decompose,
            ..
        } = self;

        let mut reset_poly = false;

        egui::SidePanel::left("side_panel")
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .margin(egui::Vec2 { x: 10.0, y: 10.0 })
                    .stroke(Stroke::new(1.0, Color32::BLACK)),
            )
            .show(ctx, |ui| {
                ui.heading("POLYDECOMP");
                ui.separator();

                egui::Grid::new("polys").min_col_width(0.0).show(ui, |ui| {
                    ui.heading("polygon");
                    ui.end_row();
                    ui.label("load polygon");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:}", selected_poly))
                        .show_ui(ui, |ui| {
                            for poly in poly_list.iter() {
                                ui.selectable_value(
                                    selected_poly,
                                    poly.to_string(),
                                    poly.to_string(),
                                );
                            }
                        });
                    ui.end_row();
                    ui.label("draw");
                    if ui.button("🖊").clicked() {
                        *draw_poly = true;
                        *load_poly = false;
                    }
                    ui.end_row();
                    ui.label("reset");
                    if ui.button("🔃").clicked() {
                        reset_poly = true;
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("triangulation");
                    if ui.button("show").clicked() {
                        *triangulate = true;
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("decomposition");
                    if ui.button("show").clicked() {
                        *decompose = true;
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.small("powered by ");
                        ui.add(
                            egui::Hyperlink::new("https://github.com/emilk/egui")
                                .text("egui")
                                .small(),
                        );
                        ui.small(" and ");
                        ui.add(
                            egui::Hyperlink::new(
                                "https://github.com/emilk/egui/tree/master/eframe",
                            )
                            .text("eframe")
                            .small(),
                        );
                    });
                });
            });
        
        let drawing_stuff = &mut self.drawing_app;
        
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(Stroke::new(1.0, Color32::BLACK)),
            )
            .show(ctx, |ui| {
                if *draw_poly {
                    dbg!(draw_poly);
                    // ui.add(drawing_app);
                    drawing_stuff.ui_content(ui);
                }
            });
        if reset_poly {
            *self = DecompApp::default();
        }
    }
}
