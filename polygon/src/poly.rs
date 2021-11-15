use std::collections::{HashMap, HashSet};

use cgmath::Point2;
use nalgebra::Point;
use smart_default::SmartDefault;
use spade::delaunay::DelaunayWalkLocate;
use spade::delaunay::*;
use spade::kernels::FloatKernel;

#[derive(SmartDefault)]
pub struct Poly {
    pub vertices: Vec<[f32; 2]>,
    pub edges: Vec<[usize; 2]>,
    pub triangles: Vec<[Point2<f32>; 3]>,
    pub triangulation:
        ConstrainedDelaunayTriangulation<Point2<f32>, FloatKernel, DelaunayWalkLocate>,
    pub bad_edges: HashSet<usize>,
    pub essential_diagonals: Vec<[usize; 2]>,
    pub convex_parts: Vec<Vec<Point2<f32>>>,
}

impl Poly {
    pub fn triang(&mut self) {
        self.triangulation = FloatCDT::with_walk_locate();
        //self.triangulation = FloatCDT::with_tree_locate();

        for v in self.vertices.iter() {
            self.triangulation.insert(Point2::new(v[0], v[1]));
        }

        for (idx, v) in self.vertices.iter().enumerate() {
            let w = self.vertices[(idx + 1) % self.vertices.len()];
            self.triangulation
                .add_constraint_edge(Point2::new(v[0], v[1]), Point2::new(w[0], w[1]));
        }

        let mut convex_hull: Vec<usize> = vec![];
        let convex_hull_iter = self.triangulation.infinite_face().adjacent_edges();
        for edge in convex_hull_iter {
            let fixed_edge = edge.fix();
            dbg!(&self.triangulation.edge(fixed_edge));
            convex_hull.push(fixed_edge);
        }

        dbg!(&convex_hull);

        // let mut bad_edges: HashSet<usize> = HashSet::new();
        self.bad_edges = HashSet::new();

        // Here we add the most exterior bad edges
        for edge_idx in convex_hull.iter() {
            if !self.triangulation.is_constraint_edge(*edge_idx) {
                self.bad_edges.insert(*edge_idx);
            }
        }

        dbg!(&self.bad_edges);

        let mut to_be_visited = self.bad_edges.clone();

        for _ in 0..10 {
            let mut newer_bad_edges: HashSet<usize> = HashSet::new();

            for edge in to_be_visited.iter() {
                //dbg!(edge);
                for nb in self.triangulation.edge(*edge).o_next_iterator() {
                    //dbg!(nb.fix());
                    if !self.triangulation.is_constraint_edge(nb.fix())
                        && !self.triangulation.is_constraint_edge(nb.sym().fix())
                    {
                        newer_bad_edges.insert(nb.fix());
                        newer_bad_edges.insert(nb.sym().fix());
                    }
                }
            }
            self.bad_edges = self.bad_edges.union(&newer_bad_edges).cloned().collect();
            to_be_visited = newer_bad_edges;
        }

        for face in self.triangulation.triangles() {
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
                } else {
                    other_edge = fixed_edge - 1;
                }
                // ---------------------

                if self.bad_edges.contains(&fixed_edge) || self.bad_edges.contains(&other_edge) {
                    should_add = false;
                }
            }

            if should_add {
                self.triangles
                    .push([*triangle[0], *triangle[1], *triangle[2]]);
            }
        }

        for vertex in self.triangulation.vertices() {
            dbg!(vertex);
            for e in vertex.ccw_out_edges() {
                if !self.bad_edges.contains(&e.fix()) {
                    dbg!(e);
                }
            }
        }
    }

    pub fn decomposition(&mut self) {
        // label constraints by 2, 1 = essential, 0 = non-essential, not known =-1
        let mut edge_labels: HashMap<FixedEdgeHandle, i32> = HashMap::new();

        let triangulation_edges = self
            .triangulation
            .edges()
            .filter(|e| !self.bad_edges.contains(&e.fix()));

        for edge in triangulation_edges
        // for edge in self
        //     .triangulation
        //     .edges()
        //     .filter(|e| !self.bad_edges.contains(&e.fix()))
        {
            let fixed_edge = edge.fix();
            let mirror_edge = edge.sym().fix();

            let mut add_label = |label: i32| {
                edge_labels.insert(fixed_edge, label);
                edge_labels.insert(mirror_edge, label);
            };

            if self.triangulation.is_constraint_edge(fixed_edge) {
                add_label(2);
            } else {
                add_label(0);
            }
        }

        // loop over vertices
        for vertex in self.triangulation.vertices() {
            let vertex_fixed = vertex.fix();
            let next_poly_vertex = *self.triangulation.vertex(self.next_vertex(vertex_fixed));
            let prev_poly_vertex = *self
                .triangulation
                .vertex(self.previous_vertex(vertex_fixed));
            let vertex_coords = *vertex;
            

            let mut outgoing_edges: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = vertex
            .ccw_out_edges()
            .filter(|e| edge_labels.get(&e.fix()).is_some())
            .collect();

            loop {
                let mut check_again = false;

                // loop through the outgoing edges of the vertex
                for idx in 0..outgoing_edges.len() {
                    let edge = outgoing_edges[idx];
                    let fixed_edge = edge.fix();
                    let mirror_edge = edge.sym().fix();
                    let opposite_vertex = edge.to().fix();
                    let opp_prev = *self
                        .triangulation
                        .vertex(self.previous_vertex(opposite_vertex));
                    let opp_next = *self.triangulation.vertex(self.next_vertex(opposite_vertex));
                    let opp_coords = *edge.to();

                    let label = edge_labels.get(&fixed_edge).unwrap();
                    dbg!(label);

                    // if edge is not known to be essential or poly edge
                    if label == &0 {
                        dbg!(vertex_fixed);
                        dbg!(opposite_vertex);
                        // if the vertex and its opposite wrt this diagonal are convex, then not essential
                        if self.convex_angle(vertex_coords, prev_poly_vertex, next_poly_vertex)
                            && self.convex_angle(opp_coords, opp_prev, opp_next)
                        {
                            dbg!("convex");
                            outgoing_edges.remove(idx);
                            check_again = true;
                            break;
                        }

                        // check if essential diagonal
                        // this means check if angle between its preceeding and subsequent edges
                        // in outgoing_edges is concave
                        let prev_vert = *outgoing_edges
                            [(idx + outgoing_edges.len() - 1) % outgoing_edges.len()]
                        .to();
                        let next_vert = *outgoing_edges[(idx + 1) % outgoing_edges.len()].to();
                        let previous = outgoing_edges
                        [(idx + outgoing_edges.len() - 1) % outgoing_edges.len()]
                    .to().fix();
                    let next = outgoing_edges[(idx + 1) % outgoing_edges.len()].to().fix();
                        dbg!(previous);
                        dbg!(next);

                        if !self.convex_angle(vertex_coords, next_vert, prev_vert) {

                            dbg!("essential");
                            *edge_labels.get_mut(&fixed_edge).unwrap() = 1;
                            *edge_labels.get_mut(&mirror_edge).unwrap() = 1;
                            continue;
                        }
                        else //if self.convex_angle(opp_coords, opp_prev, opp_next) 
                        {
                            dbg!("can remove");
                            outgoing_edges.remove(idx);
                            check_again = true;
                            break;
                        }

                        
                        
                    }
                }

                if !check_again {
                    break;
                }
            }
        }
        dbg!("checking");
        for edge in self.triangulation.edges() {
            if let Some(value) = edge_labels.get(&edge.fix()) {
                if value == &1 {
                    dbg!(edge);
                }
            }
        }
        // -------- Gluing together the triangles along the non-essential edges ----------
        let mut convex_polys: Vec<Vec<EdgeHandle<Point2<f32>, CdtEdge>>> = Vec::new();
        for triang in self.triangulation.triangles() {
            let mut triangle: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = Vec::new();
            let mut do_not_add: bool = false;
            for edge in triang.adjacent_edges() {
                if !self.bad_edges.contains(&edge.fix()) {
                    triangle.push(edge);
                }
                else {
                    do_not_add = true;
                }
            }
            if !do_not_add {convex_polys.push(triangle);}
        }

        for (edge, value) in edge_labels.iter() {
            if value == &0 {
                let edge_handle = self.triangulation.edge(*edge);
                dbg!(edge_handle);
                let mut new_poly: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = Vec::new();
                

                //dbg!(&convex_polys);
                for poly in convex_polys.iter() {
                    if poly.contains(&edge_handle)   {
                        for e in poly.iter().filter(|e| **e != edge_handle) {
                            if !new_poly.contains(e) && !new_poly.contains(&e.sym()) {
                                new_poly.push(*e);
                            }
                        }
                        
                        
                    }
                    if poly.contains(&edge_handle.sym()) {
                        for e in poly.iter().filter(|e| **e != edge_handle) {
                            if !new_poly.contains(e) && !new_poly.contains(&e.sym()) {
                                new_poly.push(*e);
                            }
                        }
                        
                    }
                }

                convex_polys = convex_polys.into_iter().filter(|poly| !poly.contains(&edge_handle) &&
            !poly.contains(&edge_handle.sym())).collect();

                convex_polys.push(new_poly);
            }
        }
        dbg!("check convex parts");
        for convex_part in convex_polys.iter() {
            dbg!(convex_part);
            let new_convex_part = self.vertex_ordering(convex_part);
            dbg!(&new_convex_part);
            self.convex_parts.push(new_convex_part);
        }


    }

    fn vertex_ordering(&self, convex_poly: &Vec<EdgeHandle<Point2<f32>, CdtEdge>>) 
    -> Vec<Point2<f32>> {
        let mut ordered_poly: Vec<[usize; 2]> = Vec::new();
        let mut final_poly: Vec<Point2<f32>> = Vec::new();
        // first order by vertex index
        for edge in convex_poly.iter() {
            ordered_poly.push([edge.from().fix(), edge.to().fix()]);
        }
        ordered_poly.sort_by_key(|tuple| tuple[0]);

        // then convert to coordinates
        for vertex in ordered_poly.iter() {
            final_poly.push(*self.triangulation.vertex(vertex[0]));
            final_poly.push(*self.triangulation.vertex(vertex[1]));
        }

        // dbg!(&ordered_poly);
        final_poly
        //ordered_poly
    }

    fn convex_angle(
        &self,
        vertex: Point2<f32>,
        prev_poly_vertex: Point2<f32>,
        next_poly_vertex: Point2<f32>,
    ) -> bool {
        // direction is ccw

        let ax = prev_poly_vertex.x;
        let ay = prev_poly_vertex.y;
        let bx = vertex.x;
        let by = vertex.y;
        let cx = next_poly_vertex.x;
        let cy = next_poly_vertex.y;

        let det = (bx - ax) * (cy - by) - (cx - bx) * (by - ay);

        if det > 0.0 {
            true
        } else {
            false
        }
    }

    fn previous_vertex(&self, vertex: usize) -> usize {
        (vertex + self.vertices.len() - 1) % self.vertices.len()
    }

    fn next_vertex(&self, vertex: usize) -> usize {
        (vertex + 1) % self.vertices.len()
    }
}
