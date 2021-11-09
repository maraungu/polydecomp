//extern crate spade;

use std::collections::HashSet;

use cgmath::Point2;
//use eframe::egui::*;
use smart_default::SmartDefault;
use spade::delaunay::*;
//use spade::HasPosition;
//use spade::TwoDimensional;
// use nalgebra::Point2;

#[derive(SmartDefault)]
pub struct Poly {
    pub vertices: Vec<[f32; 2]>,
    pub edges: Vec<[usize; 2]>,
    pub triangles: Vec<[Point2<f32>; 3]>,
    pub essential_diagonals: Vec<[usize; 2]>,
}

impl Poly {
    pub fn counter_clockwise_order(&mut self) {}

    pub fn triang(&mut self) {
        // let mut delaunay = FloatDelaunayTriangulation::with_tree_locate();
        let mut cdt_delaunay = FloatCDT::with_tree_locate();

        for v in self.vertices.iter() {
            cdt_delaunay.insert(Point2::new(v[0], v[1]));
        }

        for (idx, v) in self.vertices.iter().enumerate() {
            let w = self.vertices[(idx + 1) % self.vertices.len()];
            cdt_delaunay.add_constraint_edge(Point2::new(v[0], v[1]), Point2::new(w[0], w[1]));
        }

        let mut convex_hull: Vec<usize> = vec![];
        let convex_hull_iter = cdt_delaunay.infinite_face().adjacent_edges();
        for edge in convex_hull_iter {
            let fixed_edge = edge.fix();
            dbg!(&cdt_delaunay.edge(fixed_edge));
            convex_hull.push(fixed_edge);
        }

        dbg!(&convex_hull);
        
        let mut bad_edges: HashSet<usize> = HashSet::new(); 
       

        // Here we add the most exterior bad edges
        for edge_idx in convex_hull.iter() {
            if !cdt_delaunay.is_constraint_edge(*edge_idx) {
                bad_edges.insert(*edge_idx);
            }
        }

        dbg!(&bad_edges);

        let mut to_be_visited = bad_edges.clone();

        for _ in 0..10 {
            
            let mut newer_bad_edges: HashSet<usize> = HashSet::new();

            for edge in to_be_visited.iter() {
                //dbg!(edge);
                for nb in cdt_delaunay.edge(*edge).o_next_iterator() {
                    //dbg!(nb.fix());
                    if !cdt_delaunay.is_constraint_edge(nb.fix()) && !cdt_delaunay.is_constraint_edge(nb.sym().fix()) {
                        newer_bad_edges.insert(nb.fix());
                        newer_bad_edges.insert(nb.sym().fix());
                    }
                }
            }
            bad_edges = bad_edges.union(&newer_bad_edges).cloned().collect();
            to_be_visited = newer_bad_edges;
        }

        

        for face in cdt_delaunay.triangles() {
            let triangle = face.as_triangle();
            let mut should_add = true;
            println!(
                "Found triangle: {:?} -> {:?} -> {:?}",
                *triangle[0], *triangle[1], *triangle[2]
            );

            

            for edge in face.adjacent_edges() {
                let fixed_edge = edge.fix();
                //dbg!(fixed_edge);
                
                // order matters! -----
                let other_edge;
                if fixed_edge % 2 == 0 {
                    other_edge = fixed_edge + 1;
                }
                else {
                    other_edge = fixed_edge - 1;
                }
                // ---------------------
               
                if bad_edges.contains(&fixed_edge) || bad_edges.contains(&other_edge) {
                    should_add = false;
                }
            }

            
            if should_add {
                self.triangles
                    .push([*triangle[0], *triangle[1], *triangle[2]]);
            }
        }

        for vertex in cdt_delaunay.vertices() {
            dbg!(vertex);
            for e in vertex.ccw_out_edges() {
                if !bad_edges.contains(&e.fix()) {
                    dbg!(e);
                }
            }
        }
        
    }
}
