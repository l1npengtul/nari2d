//! See [Mesh](crate::mesh::Mesh).

use crate::mesh::{ids::*, intersection::*, *};
use std::collections::{HashMap, HashSet};

/// # Split
impl<P: TwoDimensionalPoint> Mesh<P> {
    /// Clones a subset of this mesh defined by the is_included function.
    pub fn clone_subset(&self, is_included: &dyn Fn(&Mesh<P>, FaceID) -> bool) -> Mesh<P> {
        let mut clone = self.clone();
        for face_id in clone.face_iter() {
            if !is_included(&clone, face_id) {
                let edges: Vec<HalfEdgeID> = clone.face_halfedge_iter(face_id).collect();
                clone.remove_face_unsafe(face_id);
                for halfedge_id in edges {
                    let mut walker = clone.walker_from_halfedge(halfedge_id);
                    walker.as_twin();
                    if walker.face_id().is_none() {
                        clone
                            .connectivity_info
                            .remove_halfedge(walker.halfedge_id().unwrap());
                        clone.connectivity_info.remove_halfedge(halfedge_id);
                    }
                }
            }
        }

        for vertex_id in clone.vertex_iter() {
            clone.connectivity_info.set_vertex_halfedge(vertex_id, None);
        }

        for halfedge_id in clone.halfedge_iter() {
            let walker = clone.walker_from_halfedge(halfedge_id);
            clone
                .connectivity_info
                .set_vertex_halfedge(walker.vertex_id().unwrap(), walker.twin_id());
        }
        for vertex_id in clone.vertex_iter() {
            clone.remove_vertex_if_lonely(vertex_id);
        }
        clone
    }

    ///
    /// Splits the mesh into subsets bounded by the edges where the is_at_split function returns true.
    ///
    pub fn split(&self, is_at_split: &dyn Fn(&Mesh<P>, HalfEdgeID) -> bool) -> Vec<Mesh<P>> {
        let components =
            self.connected_components_with_limit(&|halfedge_id| is_at_split(self, halfedge_id));
        components
            .iter()
            .map(|cc| self.clone_subset(&|_, face_id| cc.contains(&face_id)))
            .collect()
    }

    ///
    /// Splits the two meshes into subsets bounded by the intersection between the two meshes.
    ///
    pub fn split_at_intersection(&mut self, other: &mut Mesh<P>) -> (Vec<Mesh<P>>, Vec<Mesh<P>>) {
        let stitches = self.split_primitives_at_intersection_internal(other);
        let mut map1 = HashMap::new();
        let mut map2 = HashMap::new();
        stitches.iter().for_each(|(v0, v1)| {
            map1.insert(*v0, *v1);
            map2.insert(*v1, *v0);
        });

        let meshes1 =
            self.split(&|_, halfedge_id| is_at_intersection(self, other, halfedge_id, &map1));
        let meshes2 =
            other.split(&|_, halfedge_id| is_at_intersection(other, self, halfedge_id, &map2));
        (meshes1, meshes2)
    }

    ///
    /// Splits the primitives of the two meshes at the intersection between the two meshes.
    ///
    pub fn split_primitives_at_intersection(&mut self, other: &mut Mesh<P>) {
        self.split_primitives_at_intersection_internal(other);
    }

    fn split_primitives_at_intersection_internal(
        &mut self,
        other: &mut Mesh<P>,
    ) -> Vec<(VertexID, VertexID)> {
        let mut intersections = find_intersections(self, other);
        let mut stitches = Vec::new();
        while let Some((ref new_edges1, ref new_edges2)) =
            split_at_intersections(self, other, &intersections, &mut stitches)
        {
            intersections =
                find_intersections_between_edge_face(self, new_edges1, other, new_edges2);
        }
        stitches
    }
}

fn is_at_intersection<P: TwoDimensionalPoint>(
    mesh1: &Mesh<P>,
    mesh2: &Mesh<P>,
    halfedge_id: HalfEdgeID,
    stitches: &HashMap<VertexID, VertexID>,
) -> bool {
    let (va, vb) = mesh1.ordered_edge_vertices(halfedge_id);
    if let (Some(vc), Some(vd)) = (stitches.get(&va), stitches.get(&vb)) {
        if let Some(halfedge_id2) = mesh2.connecting_edge(*vc, *vd) {
            if mesh1.is_edge_on_boundary(halfedge_id) || mesh2.is_edge_on_boundary(halfedge_id2) {
                return true;
            }
            let mut walker1 = mesh1.walker_from_halfedge(halfedge_id);
            let mut walker2 = mesh2.walker_from_halfedge(halfedge_id2);
            let face_id10 = walker1.face_id().unwrap();
            let face_id11 = walker1.as_twin().face_id().unwrap();
            let face_id20 = walker2.face_id().unwrap();
            let face_id21 = walker2.as_twin().face_id().unwrap();
            if (!face_and_face_overlaps(&mesh1, face_id10, mesh2, face_id20)
                && !face_and_face_overlaps(&mesh1, face_id10, mesh2, face_id21))
                || (!face_and_face_overlaps(&mesh1, face_id11, mesh2, face_id20)
                    && !face_and_face_overlaps(&mesh1, face_id11, mesh2, face_id21))
            {
                return true;
            }
        }
    }
    false
}

fn face_and_face_overlaps<P: TwoDimensionalPoint>(
    mesh1: &Mesh<P>,
    face_id1: FaceID,
    mesh2: &Mesh<P>,
    face_id2: FaceID,
) -> bool {
    let (v0, v1, v2) = mesh1.face_vertices(face_id1);
    let (p0, p1, p2) = mesh2.face_positions(face_id2);

    (mesh1.vertex_point_intersection(v0, &p0).is_some()
        || mesh1.vertex_point_intersection(v1, &p0).is_some()
        || mesh1.vertex_point_intersection(v2, &p0).is_some())
        && (mesh1.vertex_point_intersection(v0, &p1).is_some()
            || mesh1.vertex_point_intersection(v1, &p1).is_some()
            || mesh1.vertex_point_intersection(v2, &p1).is_some())
        && (mesh1.vertex_point_intersection(v0, &p2).is_some()
            || mesh1.vertex_point_intersection(v1, &p2).is_some()
            || mesh1.vertex_point_intersection(v2, &p2).is_some())
}

fn split_at_intersections<P: TwoDimensionalPoint>(
    mesh1: &mut Mesh<P>,
    mesh2: &mut Mesh<P>,
    intersections: &HashMap<(Primitive, Primitive), P>,
    stitches: &mut Vec<(VertexID, VertexID)>,
) -> Option<(Vec<HalfEdgeID>, Vec<HalfEdgeID>)> {
    let mut new_edges1 = Vec::new();
    let mut new_edges2 = Vec::new();

    // Split faces
    let mut new_intersections: HashMap<(Primitive, Primitive), P> = HashMap::new();
    let mut face_splits1 = HashMap::new();
    let mut face_splits2 = HashMap::new();
    for ((id1, id2), point) in intersections.iter() {
        if let Primitive::Face(face_id) = id1 {
            match find_face_primitive_to_split(&face_splits1, mesh1, *face_id, point) {
                Primitive::Vertex(vertex_id) => {
                    new_intersections.insert((Primitive::Vertex(vertex_id), *id2), *point);
                }
                Primitive::Edge(edge) => {
                    new_intersections.insert((Primitive::Edge(edge), *id2), *point);
                }
                Primitive::Face(split_face_id) => {
                    let vertex_id = mesh1.split_face(split_face_id, point.clone());
                    insert_faces(&mut face_splits1, mesh1, *face_id, vertex_id);
                    for halfedge_id in mesh1.vertex_halfedge_iter(vertex_id) {
                        new_edges1.push(halfedge_id);
                    }
                    new_intersections.insert((Primitive::Vertex(vertex_id), *id2), *point);
                }
            }
        } else if let Primitive::Face(face_id) = id2 {
            match find_face_primitive_to_split(&face_splits2, mesh2, *face_id, point) {
                Primitive::Vertex(vertex_id) => {
                    new_intersections.insert((*id1, Primitive::Vertex(vertex_id)), *point);
                }
                Primitive::Edge(edge) => {
                    new_intersections.insert((*id1, Primitive::Edge(edge)), *point);
                }
                Primitive::Face(split_face_id) => {
                    let vertex_id = mesh2.split_face(split_face_id, point.clone());
                    insert_faces(&mut face_splits2, mesh2, *face_id, vertex_id);
                    for halfedge_id in mesh2.vertex_halfedge_iter(vertex_id) {
                        new_edges2.push(halfedge_id);
                    }
                    new_intersections.insert((*id1, Primitive::Vertex(vertex_id)), *point);
                }
            }
        } else {
            new_intersections.insert((*id1, *id2), *point);
        }
    }

    // Split edges
    let mut edge_splits1 = HashMap::new();
    let mut edge_splits2 = HashMap::new();
    for ((id1, id2), point) in new_intersections.drain() {
        let v0 = match id1 {
            Primitive::Vertex(vertex_id) => vertex_id,
            Primitive::Edge(edge) => {
                match find_edge_primitive_to_split(&edge_splits1, mesh1, edge, &point) {
                    Primitive::Vertex(vertex_id) => vertex_id,
                    Primitive::Edge(split_edge) => {
                        let (v0, v1) = mesh1.edge_vertices(split_edge);
                        let vertex_id = mesh1.split_edge(split_edge, point);

                        if !edge_splits1.contains_key(&edge) {
                            edge_splits1.insert(edge, HashSet::new());
                        }
                        let list = edge_splits1.get_mut(&edge).unwrap();

                        list.remove(&split_edge);
                        for halfedge_id in mesh1.vertex_halfedge_iter(vertex_id) {
                            let vid = mesh1.walker_from_halfedge(halfedge_id).vertex_id().unwrap();
                            if vid != v0 && vid != v1 {
                                new_edges1.push(halfedge_id);
                            } else {
                                list.insert(halfedge_id);
                            }
                        }
                        vertex_id
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        };
        let v1 = match id2 {
            Primitive::Vertex(vertex_id) => vertex_id,
            Primitive::Edge(edge) => {
                match find_edge_primitive_to_split(&edge_splits2, mesh2, edge, &point) {
                    Primitive::Vertex(vertex_id) => vertex_id,
                    Primitive::Edge(split_edge) => {
                        let (v0, v1) = mesh2.edge_vertices(split_edge);
                        let vertex_id = mesh2.split_edge(split_edge, point);

                        if !edge_splits2.contains_key(&edge) {
                            edge_splits2.insert(edge, HashSet::new());
                        }
                        let list = edge_splits2.get_mut(&edge).unwrap();

                        list.remove(&split_edge);
                        for halfedge_id in mesh2.vertex_halfedge_iter(vertex_id) {
                            let vid = mesh2.walker_from_halfedge(halfedge_id).vertex_id().unwrap();
                            if vid != v0 && vid != v1 {
                                new_edges2.push(halfedge_id);
                            } else {
                                list.insert(halfedge_id);
                            }
                        }
                        vertex_id
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        };
        stitches.push((v0, v1));
    }
    if new_edges1.len() > 0 && new_edges2.len() > 0 {
        Some((new_edges1, new_edges2))
    } else {
        None
    }
}

fn find_face_primitive_to_split<P: TwoDimensionalPoint>(
    face_splits: &HashMap<FaceID, HashSet<FaceID>>,
    mesh: &Mesh<P>,
    face_id: FaceID,
    point: &P,
) -> Primitive {
    if let Some(new_faces) = face_splits.get(&face_id) {
        for new_face_id in new_faces {
            if let Some(Intersection::Point { primitive, .. }) =
                mesh.face_point_intersection(*new_face_id, point)
            {
                return primitive;
            }
        }
        unreachable!()
    }
    Primitive::Face(face_id)
}

fn find_edge_primitive_to_split<P: TwoDimensionalPoint>(
    edge_splits: &HashMap<HalfEdgeID, HashSet<HalfEdgeID>>,
    mesh: &Mesh<P>,
    edge: HalfEdgeID,
    point: &P,
) -> Primitive {
    if let Some(new_edges) = edge_splits.get(&edge) {
        for new_edge in new_edges {
            if let Some(Intersection::Point { primitive, .. }) =
                mesh.edge_point_intersection(*new_edge, point)
            {
                return primitive;
            }
        }
        unreachable!()
    }
    Primitive::Edge(edge)
}

fn insert_faces<P: TwoDimensionalPoint>(
    face_list: &mut HashMap<FaceID, HashSet<FaceID>>,
    mesh: &Mesh<P>,
    face_id: FaceID,
    vertex_id: VertexID,
) {
    if !face_list.contains_key(&face_id) {
        face_list.insert(face_id, HashSet::new());
    }
    let list = face_list.get_mut(&face_id).unwrap();

    let mut iter = mesh.vertex_halfedge_iter(vertex_id);
    list.insert(
        mesh.walker_from_halfedge(iter.next().unwrap())
            .face_id()
            .unwrap(),
    );
    list.insert(
        mesh.walker_from_halfedge(iter.next().unwrap())
            .face_id()
            .unwrap(),
    );
    list.insert(
        mesh.walker_from_halfedge(iter.next().unwrap())
            .face_id()
            .unwrap(),
    );
}

fn find_intersections<P: TwoDimensionalPoint>(
    mesh1: &Mesh<P>,
    mesh2: &Mesh<P>,
) -> HashMap<(Primitive, Primitive), P> {
    let edges1: Vec<HalfEdgeID> = mesh1.edge_iter().collect();
    let edges2: Vec<HalfEdgeID> = mesh2.edge_iter().collect();
    find_intersections_between_edge_face(mesh1, &edges1, mesh2, &edges2)
}

fn find_intersections_between_edge_face<P: TwoDimensionalPoint>(
    mesh1: &Mesh<P>,
    edges1: &Vec<HalfEdgeID>,
    mesh2: &Mesh<P>,
    edges2: &Vec<HalfEdgeID>,
) -> HashMap<(Primitive, Primitive), P> {
    let mut intersections: HashMap<(Primitive, Primitive), P> = HashMap::new();
    for edge1 in edges1 {
        for face_id2 in mesh2.face_iter() {
            let (p0, p1) = mesh1.edge_positions(*edge1);
            if let Some(intersection) = mesh2.face_line_piece_intersection(face_id2, &p0, &p1) {
                match intersection {
                    Intersection::Point {
                        primitive: primitive2,
                        point,
                    } => {
                        if let Some(Intersection::Point {
                            primitive: primitive1,
                            ..
                        }) = mesh1.edge_point_intersection(*edge1, &point)
                        {
                            intersections.insert((primitive1, primitive2), point);
                        } else {
                            unreachable!()
                        }
                    }
                    Intersection::LinePiece {
                        primitive0: primitive20,
                        primitive1: primitive21,
                        point0,
                        point1,
                    } => {
                        if let Some(Intersection::Point {
                            primitive: primitive1,
                            ..
                        }) = mesh1.edge_point_intersection(*edge1, &point0)
                        {
                            intersections.insert((primitive1, primitive20), point0);
                        } else {
                            unreachable!()
                        }

                        if let Some(Intersection::Point {
                            primitive: primitive1,
                            ..
                        }) = mesh1.edge_point_intersection(*edge1, &point1)
                        {
                            intersections.insert((primitive1, primitive21), point1);
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }
    for edge2 in edges2 {
        for face_id1 in mesh1.face_iter() {
            let (p0, p1) = mesh2.edge_positions(*edge2);
            if let Some(intersection) = mesh1.face_line_piece_intersection(face_id1, &p0, &p1) {
                match intersection {
                    Intersection::Point {
                        primitive: primitive1,
                        point,
                    } => {
                        if let Some(Intersection::Point {
                            primitive: primitive2,
                            ..
                        }) = mesh2.edge_point_intersection(*edge2, &point)
                        {
                            intersections.insert((primitive1, primitive2), point);
                        } else {
                            unreachable!()
                        }
                    }
                    Intersection::LinePiece {
                        primitive0: primitive10,
                        primitive1: primitive11,
                        point0,
                        point1,
                    } => {
                        if let Some(Intersection::Point {
                            primitive: primitive2,
                            ..
                        }) = mesh2.edge_point_intersection(*edge2, &point0)
                        {
                            intersections.insert((primitive10, primitive2), point0);
                        } else {
                            unreachable!()
                        }

                        if let Some(Intersection::Point {
                            primitive: primitive2,
                            ..
                        }) = mesh2.edge_point_intersection(*edge2, &point1)
                        {
                            intersections.insert((primitive11, primitive2), point1);
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
        }
    }
    intersections
}
