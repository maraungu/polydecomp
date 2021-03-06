use eframe::egui::{Color32, Stroke};
use eframe::{egui, epi};
use egui::math::Pos2;

use crate::draw::PolyDraw;
use polygon::poly::Poly;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DecompApp {
    loaded_poly: bool,
    selected_poly: String,
    poly_list: Vec<String>,
    drawing_app: PolyDraw,
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
            ],
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
            loaded_poly,
            selected_poly,
            poly_list,
            triangulate,
            decompose,
            ..
        } = self;

        let mut clear_poly = false;
        let drawing_stuff = &mut self.drawing_app;

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

                    ui.label("undo draw");
                    if ui.button("???").clicked() {
                        let length = drawing_stuff.points.len();

                        if length >= 1 {
                            drawing_stuff.points.remove(length - 1);
                            if drawing_stuff.polygon.changed_orientation {
                                drawing_stuff.polygon.vertices.remove(0);
                            }
                            else {
                                drawing_stuff.polygon.vertices.remove(length - 1);
                            }
                            drawing_stuff.polygon.triangles = vec![];
                            drawing_stuff.polygon.convex_parts = vec![];
                            drawing_stuff.polygon.essential_diagonals = vec![];
                            drawing_stuff.show_decomp = false;
                            drawing_stuff.show_essentials = false;
                            *decompose = false;
                            *triangulate = false;
                        }
                    }

                    ui.end_row();
                    ui.label("clear");
                    if ui.button("????").clicked() {
                        clear_poly = true;
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.heading("triangulation");
                    if ui.button("show").clicked() {
                        if drawing_stuff.points.len() > 2 {
                            *triangulate = true;
                            drawing_stuff.polygon.triang();
                        }
                        else {
                            println!("Need at least three points");
                        }
                    }
                });

                ui.separator();
                // Make sure this doesn't react if triangulation was not activated first
                egui::Grid::new("decomp").min_col_width(0.0).show(ui, |ui| {
                    ui.heading("decomposition");
                    ui.end_row();
                    ui.label("essential diagonals");
                    if ui.button("show").clicked() && *triangulate {
                        drawing_stuff.show_essentials = true;
                        if !*decompose {
                            drawing_stuff.polygon.decomposition();
                            *decompose = true;
                        }
                    }
                    ui.end_row();
                    ui.label("convex parts");
                    if ui.button("show").clicked() && *triangulate {
                        drawing_stuff.show_decomp = true;
                        if !*decompose {
                            drawing_stuff.polygon.decomposition();
                            *decompose = true;
                        }
                    }
                });

                ui.separator();
                egui::Grid::new("howto").min_col_width(0.0).show(ui, |ui| {
                    ui.heading("how to use");
                    ui.end_row();
                    ui.label("1. draw or load polygon");
                    ui.end_row();
                    ui.label("2. show triangulation");
                    ui.end_row();
                    ui.label("3. show essential edges or convex parts");
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
                            egui::Hyperlink::new("https://github.com/Stoeoef/spade")
                                .text("spade")
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
                        for point in drawing_stuff.points.iter() {
                            drawing_stuff.polygon.vertices.push([point.x, -point.y]);
                        }
                        drawing_stuff.ui_content(ui);
                    }
                    else {
                        drawing_stuff.ui_content(ui);
                    }
                    *loaded_poly = true;
                };
                
                match selected_poly.as_str() {
                    "polygon1" => {
                        draw_chosen_polygon(vec![Pos2::from([400.0, 150.0]), 
                        Pos2::from([380.0, 300.0]),
                        Pos2::from([360.0, 330.0]),
                        Pos2::from([360.0, 350.0]),
                        Pos2::from([400.0, 400.0]),
                        Pos2::from([450.0, 380.0]),
                        Pos2::from([500.0, 370.0]),
                        Pos2::from([550.0, 380.0]),
                        Pos2::from([600.0, 400.0]),
                        Pos2::from([640.0, 350.0]),
                        Pos2::from([640.0, 330.0]),
                        Pos2::from([620.0, 300.0]),
                        Pos2::from([600.0, 150.0]),
                        Pos2::from([520.0, 210.0]),
                        Pos2::from([500.0, 200.0]),
                        Pos2::from([480.0, 210.0]),],
                        );
                    },
                    "polygon2" => {
                        draw_chosen_polygon(vec![Pos2::from([500.0, 100.0]),
                        Pos2::from([450.0, 250.0]),
                        Pos2::from([400.0, 350.0]),
                        Pos2::from([450.0, 450.0]),
                        Pos2::from([450.0, 550.0]),
                        Pos2::from([500.0, 500.0]),
                        Pos2::from([650.0, 550.0]),
                        Pos2::from([550.0, 450.0]),
                        Pos2::from([600.0, 350.0]),
                        Pos2::from([550.0, 250.0]),
                        ]);
                    },
                    
                    _ => {
                        drawing_stuff.ui_content(ui);
                    },
                }
            });

        if clear_poly {
            drawing_stuff.points.clear();
            *triangulate = false;
            *decompose = false;
            drawing_stuff.polygon = Poly::default();
            drawing_stuff.show_decomp = false;
            drawing_stuff.show_essentials = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon1() {
        let mut app = DecompApp {
            selected_poly: "polygon1".to_string(),
            ..Default::default()
        };
        
        app.drawing_app.points = vec![Pos2::from([400.0, 150.0]), 
        Pos2::from([380.0, 300.0]),
        Pos2::from([360.0, 330.0]),
        Pos2::from([360.0, 350.0]),
        Pos2::from([400.0, 400.0]),
        Pos2::from([450.0, 380.0]),
        Pos2::from([500.0, 370.0]),
        Pos2::from([550.0, 380.0]),
        Pos2::from([600.0, 400.0]),
        Pos2::from([640.0, 350.0]),
        Pos2::from([640.0, 330.0]),
        Pos2::from([620.0, 300.0]),
        Pos2::from([600.0, 150.0]),
        Pos2::from([520.0, 210.0]),
        Pos2::from([500.0, 200.0]),
        Pos2::from([480.0, 210.0]),];

        for point in app.drawing_app.points.iter() {
            app.drawing_app.polygon.vertices.push([point.x, -point.y]);
        }

        app.drawing_app.polygon.triang();
        app.drawing_app.polygon.decomposition();
        
        let convex_part_number = app.drawing_app.polygon.convex_parts.len();
        assert_eq!(convex_part_number, 6);
        
    }

    #[test]
    fn test_polygon2() {
        let mut app = DecompApp {
            selected_poly: "polygon2".to_string(),
            ..Default::default()
        };
        
        app.drawing_app.points = vec![Pos2::from([500.0, 100.0]),
        Pos2::from([450.0, 250.0]),
        Pos2::from([400.0, 350.0]),
        Pos2::from([450.0, 450.0]),
        Pos2::from([450.0, 550.0]),
        Pos2::from([500.0, 500.0]),
        Pos2::from([650.0, 550.0]),
        Pos2::from([550.0, 450.0]),
        Pos2::from([600.0, 350.0]),
        Pos2::from([550.0, 250.0]),
        ];

        for point in app.drawing_app.points.iter() {
            app.drawing_app.polygon.vertices.push([point.x, -point.y]);
        }

        app.drawing_app.polygon.triang();
        app.drawing_app.polygon.decomposition();
        
        let convex_part_number = app.drawing_app.polygon.convex_parts.len();
        assert_eq!(convex_part_number, 4);
        
    }
}
