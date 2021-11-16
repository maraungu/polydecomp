use eframe::egui::epaint::CircleShape;
use eframe::egui::*;
use polygon::poly::Poly;
use std::vec;

// we follow the egui painting demo

pub struct PolyDraw {
    pub points: Vec<Pos2>,
    pub polygon: Poly,
    pub show_essentials: bool,
    pub show_decomp: bool,
}

impl Default for PolyDraw {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            polygon: Poly::default(),
            show_essentials: false,
            show_decomp: false,
        }
    }
}

impl PolyDraw {
    pub fn ui_content(&mut self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::click());

        // ------ Shape vectors to be added to the painter -------
        let mut points_shapes: Vec<Shape> = vec![];
        let mut lines_shapes: Vec<Shape> = vec![];
        let mut triangles_shapes: Vec<Shape> = vec![];
        let mut convex_shapes: Vec<Shape> = vec![];
        let mut essential_lines: Vec<Shape> = vec![];
        // --------------------------------------------------------

        // poly vertices drawn by clicking on canvas
        if let Some(mut pointer_pos) = response.interact_pointer_pos() {
            // truncating...otherwise get point repetition due to
            // too high precision
            pointer_pos = Pos2::from([
                f32::trunc(pointer_pos.x * 10.0) / 10.0,
                f32::trunc(pointer_pos.y * 10.0) / 10.0,
            ]);

            if !self.points.contains(&pointer_pos) {
                self.points.push(pointer_pos);
                let transformed_pos: [f32; 2] = [pointer_pos.x, -pointer_pos.y];
                self.polygon.vertices.push(transformed_pos);
            }
        }

        for point in self.points.iter() {
            points_shapes.push(Shape::Circle(CircleShape {
                center: *point,
                radius: 5.0,
                fill: Color32::DARK_GRAY,
                stroke: Default::default(),
            }));
        }

        // poly edges drawn by connecting the points in order
        for (idx, point) in self.points.iter().enumerate() {
            lines_shapes.push(Shape::LineSegment {
                points: [*point, self.points[(idx + 1) % self.points.len()]],
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::GRAY,
                },
            })
        }

        // essential diagonals
        for essential in self.polygon.essential_diagonals.iter() {
            essential_lines.push(Shape::LineSegment {
                points: [Pos2::from(essential[0]), Pos2::from(essential[1])],
                stroke: Stroke {
                    width: 2.0,
                    color: Color32::RED,
                },
            })
        }

        // triangles obtained from Delaunay triangulation
        for (idx, triangle) in self.polygon.triangles.iter().enumerate() {
            let points_for_shape: Vec<Pos2> = vec![
                // need - y coordinates because I used a right-handed coord sys in poly.rs
                Pos2::from([triangle[0].x, -triangle[0].y]),
                Pos2::from([triangle[1].x, -triangle[1].y]),
                Pos2::from([triangle[2].x, -triangle[2].y]),
            ];

            let colour_triangles = |colour: Color32| {
                triangles_shapes.push(Shape::convex_polygon(
                    points_for_shape,
                    colour,
                    Stroke {
                        width: 2.0,
                        color: Color32::GRAY,
                    },
                ));
            };

            match idx % 6 {
                0 => colour_triangles(Color32::from_rgb(78, 91, 207)),
                1 => colour_triangles(Color32::from_rgb(212, 158, 11)),
                2 => colour_triangles(Color32::from_rgb(115, 23, 43)),
                3 => colour_triangles(Color32::from_rgb(52, 133, 75)),
                4 => colour_triangles(Color32::from_rgb(237, 137, 7)),
                _ => colour_triangles(Color32::from_rgb(17, 143, 130)),
            }
        }

        // convex part of the polygon
        for (idx, convex_part) in self.polygon.convex_parts.iter().enumerate() {
            let mut points_for_shape: Vec<Pos2> = Vec::new();
            for vertex in convex_part {
                points_for_shape.push(Pos2::from([vertex.x, -vertex.y]));
            }

            let colour_parts = |colour: Color32| {
                convex_shapes.push(Shape::convex_polygon(
                    points_for_shape,
                    colour,
                    Stroke::none(),
                ));
            };

            match idx % 6 {
                0 => colour_parts(Color32::from_rgb(99, 164, 186)),
                1 => colour_parts(Color32::from_rgb(191, 128, 189)),
                2 => colour_parts(Color32::from_rgb(143, 191, 128)),
                3 => colour_parts(Color32::from_rgb(235, 233, 117)),
                4 => colour_parts(Color32::from_rgb(227, 176, 132)),
                _ => colour_parts(Color32::from_rgb(137, 124, 222)),
            }
        }

        // adding to the painter

        painter.extend(triangles_shapes);
        if self.show_decomp {
            painter.extend(convex_shapes);
        }
        if self.show_essentials {
            painter.extend(essential_lines);
        }

        painter.extend(lines_shapes);
        painter.extend(points_shapes);

        response
    }
}
