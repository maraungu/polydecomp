use eframe::egui::epaint::CircleShape;
use eframe::egui::*;
use std::vec;

// we try here like in painting demo

pub struct PolyDraw {
  pub points: Vec<Pos2>,
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

impl PolyDraw {
    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        let mut points_shapes: Vec<Shape> = vec![];
        let mut lines_shapes: Vec<Shape> = vec![];

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            
            dbg!(pointer_pos);
            if !self.points.contains(&pointer_pos) {
                self.points.push(pointer_pos);
            }

            // let canvas_pos = from_screen * pointer_pos;
            // dbg!(canvas_pos);
        }

        for (idx, point) in self.points.iter().enumerate() {
            lines_shapes.push(Shape::LineSegment {
                points: [*point, self.points[(idx + 1) % self.points.len()]],
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::DARK_GRAY,
                },
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

        painter.extend(points_shapes);
        painter.extend(lines_shapes);

        response
    }
}
