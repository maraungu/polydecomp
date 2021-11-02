use eframe::{egui, epi};
use eframe::egui::*;
use plot::{Line, Values, Value, Plot};

use std::f64::consts::TAU;

pub struct PolyDraw {
    edges: Line,
    point: Pos2
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            edges: Line::new(Values::default()),
            point: Pos2::new(0.0,0.0),
        }
    }
}

impl PolyDraw {
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            edges,
            point,
        } = self;
    } 

    fn polygon(&self) -> Line {
        let n = 512;
        let circle = (0..=n).map(|i| {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
            let r = 1.5;
            Value::new(
                r * t.cos() + 0.0 as f64,
                r * t.sin() + 0.0 as f64,
            )
        });
        Line::new(Values::from_values_iter(circle))
            .color(Color32::from_rgb(100, 200, 100))
            .name("circle")
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        response
    }
    
}

impl Widget for &mut PolyDraw {
    fn ui(self, ui: &mut Ui) -> Response {
        self.options_ui(ui);

        let mut plot = Plot::new("poly_demo")
            .show_background(false);
            //.line(self.polygon());

        let mut actual_plot = plot.line(self.polygon());

        ui.add(actual_plot)

    }
}

