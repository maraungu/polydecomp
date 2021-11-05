use eframe::egui::*;
use eframe::{egui, epi};

use plot::{Line, Plot, Points, Value, Values};


use std::f64::consts::TAU;

pub struct DrawPoint {
    point: Pos2,
}

impl Default for DrawPoint {
    fn default() -> Self {
        Self {
            point: Pos2::new(0.0, 0.0),
        }
    }
}

pub struct PolyDraw {
    edges: Line,
    //point: Pos2,
    points: Points,
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            edges: Line::new(Values::default()),
            // point: Pos2::new(0.0, 0.0),
            points: Points::new(Values::default()),
        }
    }
}

impl PolyDraw {
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            edges,
            // point,
            points,
        } = self;
    }

    fn polygon(&self) -> Line {
        let n = 512;
        let circle = (0..=n).map(|i| {
            let t = remap(i as f64, 0.0..=(n as f64), 0.0..=TAU);
            let r = 1.5;
            Value::new(r * t.cos() + 0.0 as f64, r * t.sin() + 0.0 as f64)
        });
        Line::new(Values::from_values_iter(circle))
            .color(Color32::from_rgb(100, 200, 100))
            .name("circle")
    }

    // pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
    //     let (mut response, painter) =
    //         ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

    //     if let Some(pointer_pos) = response.interact_pointer_pos() {
    //         self.point = pointer_pos;
    //     }

    //     response
    // }
}

impl Widget for &mut PolyDraw {
    fn ui(self, ui: &mut Ui) -> Response {
        self.options_ui(ui);

        let mut plot = Plot::new("poly_demo").show_background(false);
        //.line(self.polygon());

        let mut actual_plot = plot.line(self.polygon());

        let plot_id = ui.make_persistent_id("poly_demo");

        let response = ui.interact(Rect::EVERYTHING, plot_id, Sense::click());
        if let Some(pointer_pos) = response.interact_pointer_pos() {
            dbg!(pointer_pos);

            //let some_value = ScreenTransform::value_from_position(pointer_pos);
        }

        

        let marker = Points::new(Values::from_values(vec![Value::new(0.0, 0.0)]))
            .shape(plot::MarkerShape::Diamond)
            .color(Color32::BLACK)
            .radius(5.0);

        let actual_plot = actual_plot.points(marker).include_x(0.0);

        // dbg!(winit::MouseInput::press_origin());
        // let (response, painter) =
        //     ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        // let response = ui.allocate_response(egui::vec2(1.0, 2.0), egui::Sense::click());
        // dbg!(&response);
        // if let Some(pointer_pos) = response.interact_pointer_pos() {
        //     self.point = pointer_pos;
        //     dbg!(pointer_pos);
        //     let some_points = Points::new(Values::from_values(vec![Value::new(
        //         self.point.x,
        //         self.point.y,
        //     )])).shape(plot::MarkerShape::Circle);

        //     //let some_points = &self.points;

        //     actual_plot = actual_plot.points(some_points);
        // }

        ui.add(actual_plot)
    }
}
