use eframe::egui::epaint::CircleShape;
use eframe::egui::*;
use polygon::poly::Poly;
use std::vec;

// we try here like in painting demo

pub struct PolyDraw {
    pub points: Vec<Pos2>,
    pub polygon: Poly,
    // triangles: Vec<Vec<Pos2>>
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            polygon: Poly::default(),
        }
    }
}

impl PolyDraw {
    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        // let to_screen = emath::RectTransform::from_to(
        //     Rect::from_center_size(Pos2::ZERO, response.rect.square_proportions()),
        //     response.rect,
        // );
        // let from_screen = to_screen.inverse();

        let mut points_shapes: Vec<Shape> = vec![];
        let mut lines_shapes: Vec<Shape> = vec![];
        let mut triangles_shapes: Vec<Shape> = vec![];
        let mut convex_shapes: Vec<Shape> = vec![];

        // poly vertices drawn by clicking on canvas
        if let Some(mut pointer_pos) = response.interact_pointer_pos() {
            //dbg!(pointer_pos);
            // truncating...otherwise get point repetition due to
            // too high precision
            pointer_pos = Pos2::from([
                f32::trunc(pointer_pos.x * 10.0) / 10.0,
                f32::trunc(pointer_pos.y * 10.0) / 10.0,
            ]);
            //dbg!(pointer_pos);

            if !self.points.contains(&pointer_pos) {
                self.points.push(pointer_pos);
                //dbg!(&self.points);

                // let canvas_pos = from_screen * pointer_pos;
                // dbg!(canvas_pos);
                // let new_canvas_pos = [canvas_pos.x, -canvas_pos.y];
                // dbg!(new_canvas_pos);
                // // TODO: truncate this too
                // self.polygon.vertices.push(new_canvas_pos);

                let transformed_pos: [f32; 2] = [pointer_pos.x, -pointer_pos.y];
                self.polygon.vertices.push(transformed_pos);
            }
        }

        // poly edges drawn by connecting the vertices in order
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

        for (idx, triangle) in self.polygon.triangles.iter().enumerate() {
            let points_for_shape: Vec<Pos2> = vec![
                Pos2::from([triangle[0].x, -triangle[0].y]),
                Pos2::from([triangle[1].x, -triangle[1].y]),
                Pos2::from([triangle[2].x, -triangle[2].y]),
            ];
            //dbg!(&points_for_shape);
            //dbg!(idx);

            let colour_triangles = |colour: Color32| {
                triangles_shapes.push(Shape::convex_polygon(
                    points_for_shape,
                    colour,
                    Stroke {
                        width: 1.0,
                        color: Color32::BLACK,
                    },
                ));
            };

            match idx % 4 {
                0 => colour_triangles(Color32::from_rgba_premultiplied(78, 91, 207, 255)),
                1 => colour_triangles(Color32::from_rgba_premultiplied(212, 158, 11, 255)),
                2 => colour_triangles(Color32::from_rgba_premultiplied(115, 23, 43, 255)),
                _ => colour_triangles(Color32::from_rgba_premultiplied(52, 133, 75, 255)),
            }

            // triangles_shapes.push(Shape::convex_polygon(
            //     points_for_shape,
            //     Color32::BLUE,
            //     Stroke::none(),)
            // );
        }

        painter.extend(points_shapes);
        painter.extend(lines_shapes);
        painter.extend(triangles_shapes);

        response
    }
}
