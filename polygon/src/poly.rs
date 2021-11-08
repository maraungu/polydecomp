//extern crate spade;

use smart_default::SmartDefault;
use eframe::egui::*;
use spade::delaunay::*;
use spade::TwoDimensional;
use spade::HasPosition;
use cgmath::Point2;
// use nalgebra::Point2;


#[derive(SmartDefault)]
pub struct Poly {
    pub vertices: Vec<[f32; 2]>,
    pub edges: Vec<[usize; 2]>,
    pub triangles: Vec<[Point2<f32>; 3]>,
    pub essential_diagonals: Vec<[usize; 2]>,
}

impl Poly {
    pub fn counter_clockwise_order(&mut self) {

    }

    pub fn triang(&mut self) {
        let mut delaunay = FloatDelaunayTriangulation::with_tree_locate();
        
        

        for v in self.vertices.iter() {
            delaunay.insert(Point2::new(v[0], v[1]));
        }
        for face in delaunay.triangles() {
            let triangle = face.as_triangle();
            println!("Found triangle: {:?} -> {:?} -> {:?}", *triangle[0], *triangle[1], *triangle[2]);
            self.triangles.push([*triangle[0], *triangle[1], *triangle[2]]);
        }

        

        
    }
}