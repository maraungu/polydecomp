use eframe::egui::{Color32, Stroke};
use eframe::{egui, epi};
use egui::math::Pos2;

use crate::draw::{self, PolyDraw};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DecompApp {
    loaded_poly: bool,
    selected_poly: String,
    poly_list: Vec<String>,
    drawing_app: PolyDraw,
    //save_your_poly: bool,
    your_poly: Vec<Pos2>,
    triangulate: bool,
    decompose: bool,
}

impl Default for DecompApp {
    fn default() -> Self {
        Self {
            loaded_poly: false,
            selected_poly: "select a polygon".to_string(),
            poly_list: vec![
                "select a polygon".to_string(),
                "polygon1".to_string(),
                "polygon2".to_string(),
                "your polygon".to_string(),
            ],
            drawing_app: PolyDraw::default(),
            //save_your_poly: false,
            your_poly: Vec::new(),
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
           
            loaded_poly,
            selected_poly,
            poly_list,
            //save_your_poly,
            triangulate,
            decompose,
            ..
        } = self;

        let mut save_your_poly = false;
        let mut clear_poly = false;
        let drawing_stuff = &mut self.drawing_app;
        let the_saved_poly = self.your_poly.clone();

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
                                let response = ui.selectable_value(
                                    selected_poly,
                                    poly.to_string(),
                                    poly.to_string(),
                                );
                                if response.changed() {
                                    *loaded_poly = false;
                                }
                            }
                        });
                    ui.end_row();
                    
                    ui.label("undo");
                    if ui.button("⟲").clicked() {
                        let length = drawing_stuff.points.len();
                        dbg!(length);
                        drawing_stuff.points.remove(length - 1);
                    }
                    ui.end_row();
                    ui.label("save");
                    if ui.button("💾").clicked() {
                        save_your_poly = true;
                    }
                    ui.end_row();
                    ui.label("clear");
                    if ui.button("🔃").clicked() {
                        clear_poly = true;
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

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(255, 255, 255))
                    .stroke(Stroke::new(1.0, Color32::BLACK)),
            )
            .show(ctx, |ui| {
                ui.label("to draw a polygon add the vertices by clicking on the canvas or load one of the default ones");
                
                let mut draw_chosen_polygon = |poly: Vec<Pos2>| {
                    if !*loaded_poly {
                        drawing_stuff.points = poly;
                        drawing_stuff.ui_content(ui);
                    }
                    else {
                        drawing_stuff.ui_content(ui);
                    }
                    *loaded_poly = true;
                };
                
                match selected_poly.as_str() {
                    "polygon1" => {
                        draw_chosen_polygon(vec![Pos2::from([600.0, 350.0]), Pos2::from([500.0, 450.0])]);
                    }
                    "polygon2" => {
                        draw_chosen_polygon(vec![Pos2::from([330.0, 170.0]), 
                                    Pos2::from([340.0, 390.0]),
                                    Pos2::from([555.0, 390.0]),
                                    Pos2::from([465.0, 330.0]),
                                    Pos2::from([460.0, 255.0]),
                                    Pos2::from([515.0, 167.0])]);
                    }
                    "your polygon" => {
                        draw_chosen_polygon(the_saved_poly);
                    }
                    _ => {
                        drawing_stuff.ui_content(ui);
                    }
                }
            });
        if save_your_poly {
            self.your_poly = drawing_stuff.points.clone();
        }
        if clear_poly {
            //*self = DecompApp::default();
            drawing_stuff.points.clear();
        }
    }
}
