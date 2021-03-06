use cgmath::Point2;
use smart_default::SmartDefault;
use spade::delaunay::DelaunayWalkLocate;
use spade::delaunay::*;
use spade::kernels::FloatKernel;
use std::collections::{HashMap, HashSet};

#[derive(SmartDefault)]
pub struct Poly {
    pub vertices: Vec<[f32; 2]>,
    pub changed_orientation: bool,
    pub triangles: Vec<[Point2<f32>; 3]>,
    pub triangulation:
        ConstrainedDelaunayTriangulation<Point2<f32>, FloatKernel, DelaunayWalkLocate>,
    pub bad_edges: HashSet<usize>,
    pub essential_diagonals: Vec<Vec<[f32; 2]>>,
    pub convex_parts: Vec<Vec<Point2<f32>>>,
}

impl Poly {
    /// Triangulates the Poly in place.  Uses constrained Delaunay 
    /// from the spade crate.
    /// The resulting triangles are stored in the triangles field of Poly
    pub fn triang(&mut self) {
        // ------ Triangulation -----------

        self.initialise_triangulation(); // this also yields the convex hull

        // if poly vertices not in ccw order, reverse 
        if !self.poly_vertices_ccw() && !self.changed_orientation {
            self.changed_orientation = true;
            self.vertices.reverse();
            self.initialise_triangulation();
        }

        // Need to collect and remove the "bad edges" of the triangulation
        // They occur because the triangulation is of the convex hull of
        // the poly vertices so there will be some triangulation edges 
        // outside the polygon.
        let mut convex_hull: Vec<usize> = vec![];
        let convex_hull_iter = self.triangulation.infinite_face().adjacent_edges();
        for edge in convex_hull_iter {
            let fixed_edge = edge.fix();
            convex_hull.push(fixed_edge);
        }

        self.bad_edges = HashSet::new();

        // Here we add the most exterior bad edges
        // They are defined as edges of the convex hull
        // that are not also polygon edges
        for edge_idx in convex_hull.iter() {
            if !self.triangulation.is_constraint_edge(*edge_idx) {
                self.bad_edges.insert(*edge_idx);
            }
        }

        let mut to_be_visited = self.bad_edges.clone();

        // loop through the neighbours of the bad edges
        // if they are outside the poly they are also inserted in the bad_edges vector
        for _ in 0..10 {
            let mut newer_bad_edges: HashSet<usize> = HashSet::new();

            for edge in to_be_visited.iter() {
                for nb in self.triangulation.edge(*edge).o_next_iterator() {
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

        // loop through the triangles and check if they contain bad edges
        // if yes, then ignore; if no, then add to the triangles field
        for face in self.triangulation.triangles() {
            let triangle = face.as_triangle();
            let mut should_add = true;

            for edge in face.adjacent_edges() {
                let fixed_edge = edge.fix();
                let other_edge = edge.sym().fix();

                if self.bad_edges.contains(&fixed_edge) || self.bad_edges.contains(&other_edge) {
                    should_add = false;
                }
            }

            if should_add {
                self.triangles
                    .push([*triangle[0], *triangle[1], *triangle[2]]);
            }
        }
    }

    /// Implementation of the Hertel-Mehlhorn convex decomposition
    /// of a polygon.  Starts from a triangulation and
    /// eliminates all triangle edges that are not essential, i.e.
    /// whose elimination does not make an angle concave.
    pub fn decomposition(&mut self) {
        // labels: 2 = poly edges, 1 = essential, 0 = non-essential
        let mut edge_labels: HashMap<FixedEdgeHandle, i32> = HashMap::new();

        // ignore the bad edges from the triangulation
        let triangulation_edges = self
            .triangulation
            .edges()
            .filter(|e| !self.bad_edges.contains(&e.fix()));

        // label the remaining edges
        for edge in triangulation_edges {
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

            // store all outgoing (non bad) edges from this vertex
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

                    let label = edge_labels.get(&fixed_edge).unwrap(); // ok to unwrap becaus of the filter used for outgoing_edges

                    // if edge is not known to be essential or poly edge
                    if label == &0 {
                        // if the vertex and its opposite wrt this diagonal are convex, then not essential
                        if self.convex_angle(vertex_coords, prev_poly_vertex, next_poly_vertex)
                            && self.convex_angle(opp_coords, opp_prev, opp_next)
                        {
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

                        // note the order switch!
                        if !self.convex_angle(vertex_coords, next_vert, prev_vert) {
                            // if essential label with 1
                            *edge_labels.get_mut(&fixed_edge).unwrap() = 1;
                            *edge_labels.get_mut(&mirror_edge).unwrap() = 1;
                            // add to essentials
                            self.essential_diagonals
                                .push(vec![[vertex.x, -vertex.y], [opp_coords.x, -opp_coords.y]]);
                            continue;
                        } else {
                            // else remove from outgoing edges and check again
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

        // --- Remove not really essential essentials ------
        // Comment this for loop out to see effect on polygon1
        // It is another traversal of the poly edges to
        // establish which essential diagonals are truly essential
        // --------------------------------------------------
        for vertex in self.triangulation.vertices() {
            let vertex_fixed = vertex.fix();
            let next_poly_vertex = *self.triangulation.vertex(self.next_vertex(vertex_fixed));
            let prev_poly_vertex = *self
                .triangulation
                .vertex(self.previous_vertex(vertex_fixed));
            let vertex_coords = *vertex;

            // only look at the essential and poly edges
            // so ignore the bad edges and those with label 0
            let mut outgoing_edges: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = vertex
                .ccw_out_edges()
                .filter(|e| {
                    let label = edge_labels.get(&e.fix());
                    label.is_some() && label != Some(&0)
                })
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

                    let label = edge_labels.get(&fixed_edge).unwrap(); // ok to unwrap becaus of the filter used for outgoing_edges

                    // if edge has been marked as essential (so not poly edge)
                    if label == &1 {
                       
                        // if the vertex is concave and its opposite wrt this diagonal is convex, then we check
                        if !self.convex_angle(vertex_coords, prev_poly_vertex, next_poly_vertex)
                            && self.convex_angle(opp_coords, opp_prev, opp_next)
                        {   
                            // check if essential diagonal
                            let prev_vert = *outgoing_edges
                                [(idx + outgoing_edges.len() - 1) % outgoing_edges.len()]
                            .to();
                            let next_vert = *outgoing_edges[(idx + 1) % outgoing_edges.len()].to();

                            // note the order switch!
                            if self.convex_angle(vertex_coords, next_vert, prev_vert) {
                                
                                // if not really essential label with 0
                                *edge_labels.get_mut(&fixed_edge).unwrap() = 0;
                                *edge_labels.get_mut(&mirror_edge).unwrap() = 0;
                                // remove from essentials
                                self.essential_diagonals.retain(|vector| {
                                    *vector
                                        != vec![
                                            [vertex.x, -vertex.y],
                                            [opp_coords.x, -opp_coords.y],
                                        ]
                                });
                                outgoing_edges.remove(idx);
                                check_again = true;
                                break;
                            }
                        }
                    }
                }

                if !check_again {
                    break;
                }
            }
        }

        

        // -------- Gluing together the triangles along the non-essential edges ----------
        // First add all triangles to the convex_polys vector
        let mut convex_polys: Vec<Vec<EdgeHandle<Point2<f32>, CdtEdge>>> = Vec::new();
        for triang in self.triangulation.triangles() {
            let mut triangle: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = Vec::new();
            let mut do_not_add: bool = false;
            for edge in triang.adjacent_edges() {
                if !self.bad_edges.contains(&edge.fix()) {
                    triangle.push(edge);
                } else {
                    do_not_add = true;
                }
            }
            if !do_not_add {
                convex_polys.push(triangle);
            }
        }

        for (edge, value) in edge_labels.iter() {
            // find the polygons containing this non-essential edge
            // collect their unique vertices minus the non-essential
            // edge in a new_poly
            if value == &0 {
                let edge_handle = self.triangulation.edge(*edge);
                let mut new_poly: Vec<EdgeHandle<Point2<f32>, CdtEdge>> = Vec::new();

                for poly in convex_polys.iter() {
                    if poly.contains(&edge_handle) {
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

                convex_polys = convex_polys
                    .into_iter()
                    .filter(|poly| {
                        !poly.contains(&edge_handle) && !poly.contains(&edge_handle.sym())
                    })
                    .collect();

                convex_polys.push(new_poly);
            }
        }

        for convex_part in convex_polys.iter() {
            let new_convex_part = self.vertex_ordering(convex_part);
            self.convex_parts.push(new_convex_part);
        }
    }

    /// Constrained Delaunay triangulation from the spade crate.
    /// Constraints are the polygon edges.
    fn initialise_triangulation(&mut self) {
        
        self.triangulation = FloatCDT::with_walk_locate();

       for v in self.vertices.iter() {
            self.triangulation.insert(Point2::new(v[0], v[1]));
        }

        // add polygon edges as constraints in the CDT
        for idx in 0..self.vertices.len() {
            let next_idx = (idx + 1) % self.vertices.len();
            if self.triangulation.can_add_constraint(idx, next_idx) {
                self.triangulation.add_constraint(idx, next_idx);
            }
            else {
                println!("Cannot have intersecting polygon edges. Press clear and start again");
                break;
               
            }
        }
    }
    
    /// Checks that the polygon vertices are in ccw order
    /// by looking at the convex hull
    fn poly_vertices_ccw(&self) -> bool {
        
        let mut convex_hull_iter = self.triangulation.infinite_face().adjacent_edges();
       
        let first_edge = convex_hull_iter.nth(0);
        
        let mut first_vertex =  first_edge.unwrap().from().fix();
        let mut second_vertex = first_edge.unwrap().to().fix();
        

        if first_vertex == 0 {
            first_vertex = self.vertices.len();
        }
        if second_vertex == 0 {
            second_vertex = self.vertices.len();
        }

        if first_vertex < second_vertex {
            false
        }
        else {
            true
        }
    }

    /// Ordering function that ensures that the vertices of the
    /// convex parts are ordered as follows:
    /// [point1, point2], [point2, point3], ...
    fn vertex_ordering(
        &self,
        convex_poly: &Vec<EdgeHandle<Point2<f32>, CdtEdge>>,
    ) -> Vec<Point2<f32>> {
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
        final_poly
    }

    /// Returns true if angle is convex
    fn convex_angle(
        &self,
        vertex: Point2<f32>,
        prev_poly_vertex: Point2<f32>,
        next_poly_vertex: Point2<f32>,
    ) -> bool {
        // direction is ccw

        let a_x = prev_poly_vertex.x;
        let a_y = prev_poly_vertex.y;
        let b_x = vertex.x;
        let b_y = vertex.y;
        let c_x = next_poly_vertex.x;
        let c_y = next_poly_vertex.y;

        let determinant = (b_x - a_x) * (c_y - b_y) - (c_x - b_x) * (b_y - a_y);

        if determinant > 0.0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccw_detection() {
        let mut polygon = Poly::default();
        polygon.vertices = vec![[30.0, 30.0], [40.0, 10.0], [10.0, 10.0]];
        polygon.initialise_triangulation();
        let orientation = polygon.poly_vertices_ccw();
        assert_eq!(orientation, false);
    }

    #[test]
    fn test_triangle() {
        let mut polygon = Poly::default();
        polygon.vertices = vec![[30.0, 30.0], [10.0, 10.0], [40.0, 10.0]];
        polygon.triang();
        polygon.decomposition();
        let triangle_number = polygon.triangles.len();
        let convex_part_number = polygon.convex_parts.len();
        assert_eq!(triangle_number, 1);
        assert_eq!(convex_part_number, 1);
    }

    #[test]
    fn test_beak_poly() {
        let mut polygon = Poly::default();
        polygon.vertices = vec![[10.0, 10.0], [10.0, 5.0], [20.0, 0.0], [0.0, 0.0]];
        polygon.triang();
        polygon.decomposition();
        let triangle_number = polygon.triangles.len();
        let convex_part_number = polygon.convex_parts.len();
        let essential_number = polygon.essential_diagonals.len();
        assert_eq!(triangle_number, 2);
        assert_eq!(convex_part_number, 2);
        assert_eq!(essential_number, 1);
    }
}
