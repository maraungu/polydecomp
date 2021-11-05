use eframe::egui::epaint::CircleShape;
use eframe::egui::*;
use eframe::{egui, epi};

use plot::{Line, Plot, Points, Value, Values};

use std::f64::consts::TAU;
use std::vec;

// we try here like in painting demo
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
    point: Pos2,
    points: Vec<Pos2>,
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            edges: Line::new(Values::default()),
            point: Pos2::new(0.0, 0.0),
            points: Vec::new(),
        }
    }
}

impl PolyDraw {
    fn options_ui(&mut self, ui: &mut Ui) {
        let Self {
            edges,
            // point,
            point,
            points,
        } = self;
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let mut point_shapes: Vec<Shape> = vec![];
        let mut points_shapes: Vec<Shape> = vec![];
        let mut lines_shapes: Vec<Shape> = vec![];

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            self.point = pointer_pos;
            dbg!(pointer_pos);
            self.points.push(pointer_pos);
            let canvas_pos = from_screen * pointer_pos;
            dbg!(canvas_pos);
            // self.point = canvas_pos;
            // point_shapes.push(Shape::Circle(CircleShape {
            //     center: pointer_pos,
            //     radius: 5.0,
            //     fill: Color32::BLACK,
            //     stroke: Default::default(),
            // }));
        }

        for (idx, point) in self.points.iter().enumerate() {
            lines_shapes.push(Shape::LineSegment {
                points: [*point, self.points[(idx + 1) % self.points.len()]],
                stroke: Stroke { width: 1.0, color: Color32::BLACK },
            })
        }

        for point in self.points.iter() {
            points_shapes.push(Shape::Circle(CircleShape {
                center: *point,
                radius: 5.0,
                fill: Color32::DARK_GRAY,
                stroke: Default::default(),
            }));
        }

        point_shapes.push(Shape::Circle(CircleShape {
            center: self.point,
            radius: 5.0,
            fill: Color32::LIGHT_GRAY,
            stroke: Default::default(),
        }));

        painter.extend(points_shapes);
        painter.extend(lines_shapes);

        response
    }
}

// impl Widget for &mut PolyDraw {
//     fn ui(self, ui: &mut Ui) -> Response {
//         self.options_ui(ui);

//         let mut  response = ui.interact(Rect::EVERYTHING, plot_id, Sense::click());
//         if let Some(pointer_pos) = response.interact_pointer_pos() {
//             dbg!(pointer_pos);

//             //let some_value = ScreenTransform::value_from_position(pointer_pos);
//         }

//         let marker = Points::new(Values::from_values(vec![Value::new(0.0, 0.0)]))
//             .shape(plot::MarkerShape::Diamond)
//             .color(Color32::BLACK)
//             .radius(5.0);

//         // dbg!(winit::MouseInput::press_origin());
//         // let (response, painter) =
//         //     ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

//         // let response = ui.allocate_response(egui::vec2(1.0, 2.0), egui::Sense::click());
//         // dbg!(&response);
//         // if let Some(pointer_pos) = response.interact_pointer_pos() {
//         //     self.point = pointer_pos;
//         //     dbg!(pointer_pos);
//         //     let some_points = Points::new(Values::from_values(vec![Value::new(
//         //         self.point.x,
//         //         self.point.y,
//         //     )])).shape(plot::MarkerShape::Circle);

//         //     //let some_points = &self.points;

//         //     actual_plot = actual_plot.points(some_points);
//         // }

//        response
//     }
// }
