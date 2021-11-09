//extern crate spade;

use std::collections::HashSet;

use cgmath::Point2;
use eframe::egui::*;
use smart_default::SmartDefault;
use spade::delaunay::*;
use spade::HasPosition;
use spade::TwoDimensional;
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
        let mut new_bad_edges: HashSet<usize> = HashSet::new(); 

        // Here we add the most exterior bad edges
        for edge_idx in convex_hull.iter() {
            if !cdt_delaunay.is_constraint_edge(*edge_idx) {
                bad_edges.insert(*edge_idx);
            }
        }

        dbg!(&bad_edges);

        // from each exterior bad edge we work inwards 
        for edge_idx in bad_edges.iter() {
            let mut still_have_undiscovered_baddies = true;

            let mut reference_edge = edge_idx - 1;

            let mut flag = 0;
            while flag < 5 { //still_have_undiscovered_baddies {
                let mut found_some_nonconstraint_edge = false;
                
                let face = cdt_delaunay.edge(reference_edge).face();
                //dbg!(face.fix());
                for face_edge in face.adjacent_edges() {
                    //dbg!(face_edge.fix());
                    if !cdt_delaunay.is_constraint_edge(face_edge.fix()) {
                        new_bad_edges.insert(face_edge.fix());
                        found_some_nonconstraint_edge = true;
                        reference_edge = face_edge.sym().fix();
                        //dbg!(reference_edge);
                    }
                if !found_some_nonconstraint_edge {
                    still_have_undiscovered_baddies = false;
                }
                flag += 1;
                
            }

            }
            
            
            // let edge = edge_idx - 1;
            // let face = cdt_delaunay.edge(edge).face();
            // for face_edge in face.adjacent_edges() {
            //     if !cdt_delaunay.is_constraint_edge(face_edge.fix()) {
            //         new_bad_edges.insert(face_edge.fix());
            //     }
            // }
            
            // let prev = cdt_delaunay.edge(edge).o_prev();
            // let next = cdt_delaunay.edge(edge).o_next();
            // if !cdt_delaunay.is_constraint_edge(prev.fix()) {
            //     dummy_bad_edges.insert(prev.fix());
            // }
            // if !cdt_delaunay.is_constraint_edge(next.fix()) {
            //     dummy_bad_edges.insert(next.fix());
            // }
        }

        let mut newer_bad_edges: HashSet<usize> = HashSet::new();
        
        for edge in new_bad_edges.iter() {
            dbg!(edge);
            for nb in cdt_delaunay.edge(*edge).o_next_iterator() {
                dbg!(nb.fix());
                if !cdt_delaunay.is_constraint_edge(nb.fix()) && !cdt_delaunay.is_constraint_edge(nb.sym().fix()) {
                    newer_bad_edges.insert(nb.fix());
                    newer_bad_edges.insert(nb.sym().fix());
                }
            }
        }

        let mut newest_bad_edges: HashSet<usize> = HashSet::new();
        
        for edge in newer_bad_edges.iter() {
            dbg!(edge);
            for nb in cdt_delaunay.edge(*edge).o_next_iterator() {
                dbg!(nb.fix());
                if !cdt_delaunay.is_constraint_edge(nb.fix()) && !cdt_delaunay.is_constraint_edge(nb.sym().fix()) {
                    newest_bad_edges.insert(nb.fix());
                    newest_bad_edges.insert(nb.sym().fix());
                }
            }
        }

        let mut more_bad_edges: HashSet<usize> = HashSet::new();
        
        for edge in newest_bad_edges.iter() {
            dbg!(edge);
            for nb in cdt_delaunay.edge(*edge).o_next_iterator() {
                dbg!(nb.fix());
                if !cdt_delaunay.is_constraint_edge(nb.fix()) && !cdt_delaunay.is_constraint_edge(nb.sym().fix()) {
                    more_bad_edges.insert(nb.fix());
                    more_bad_edges.insert(nb.sym().fix());
                }
            }
        }

        bad_edges = bad_edges.union(&new_bad_edges).cloned().collect();
        bad_edges = bad_edges.union(&newer_bad_edges).cloned().collect();
        bad_edges = bad_edges.union(&newest_bad_edges).cloned().collect();
        bad_edges = bad_edges.union(&more_bad_edges).cloned().collect();
        dbg!(&bad_edges);

        for face in cdt_delaunay.triangles() {
            let triangle = face.as_triangle();
            let mut should_add = true;
            println!(
                "Found triangle: {:?} -> {:?} -> {:?}",
                *triangle[0], *triangle[1], *triangle[2]
            );

            let mut number_not_constraints = 0;
            let mut convex_guy = false;

            for edge in face.adjacent_edges() {
                let fixed_edge = edge.fix();
                dbg!(fixed_edge);
                
                // let start = [edge.from().x, edge.from().y];
                // let end = [edge.to().x, edge.to().y];
                
                // let middle = Point2::new((start[0] + end[0]) / 2.0,
                //                     (start[1] + end[1]) / 2.0);
                
                // if !cdt_delaunay.is_constraint_edge(fixed_edge) && 
                // cdt_delaunay.locate(middle) ==  {
                //     dbg!("bad guy");
                //     should_add = false;
                // }
                
                // order matters! -----
                let other_edge;
                if fixed_edge % 2 == 0 {
                    other_edge = fixed_edge + 1;
                }
                else {
                    other_edge = fixed_edge - 1;
                }
                // ---------------------
                dbg!(&cdt_delaunay.edge(fixed_edge));
                if convex_hull.contains(&fixed_edge) || convex_hull.contains(&other_edge)
                 && !cdt_delaunay.is_constraint_edge(fixed_edge)
                {
                    dbg!("bad guy");
                    bad_edges.insert(fixed_edge);
                    bad_edges.insert(other_edge);
                    should_add = false;
                    convex_guy = true;
                }
                if bad_edges.contains(&fixed_edge) || bad_edges.contains(&other_edge) {
                    should_add = false;
                }

                if !cdt_delaunay.is_constraint_edge(fixed_edge) {
                    number_not_constraints += 1;
                }
                    
                
            }

            if convex_guy && number_not_constraints >= 2 {
                for edge in face.adjacent_edges() {
                    let fixed_edge = edge.fix();
                    if !cdt_delaunay.is_constraint_edge(fixed_edge) {
                        // order matters! -----
                        let other_edge;
                        if fixed_edge % 2 == 0 {
                            other_edge = fixed_edge + 1;
                        }
                        else {
                            other_edge = fixed_edge - 1;
                        }
                        bad_edges.insert(fixed_edge);
                        bad_edges.insert(other_edge);
                    }
                }

            }
            dbg!(&bad_edges);
            if should_add {
                self.triangles
                    .push([*triangle[0], *triangle[1], *triangle[2]]);
            }
        }

        
    }
}
