use crate::{
    error::{NResult, Nari2DError},
    geometry::{point2d::float_cmp, Point2d},
};
use petgraph::stable_graph::StableGraph;
use rstar::RTree;

pub type Edge = (Point2d, Point2d);
pub type EdgeIdx = (usize, usize);

// temporary triangle structs
struct Triangle {
    vert0: usize,
    vert1: usize,
    vert2: usize,
    neighbour0: Option<usize>,
    neighbour1: Option<usize>,
    neighbour2: Option<usize>,
}

struct TriangleEdge {
    triangle: usize,
}

struct Bins {
    bins: Vec<Vec<Point2d>>,
    n_bins: usize,
    max: Point2d,
}

impl Bins {
    pub fn new(n_bins: usize, y_max: f32, x_max: f32) -> Self {
        Bins {
            bins: vec![vec![]; n_bins],
            n_bins,
            max: Point2d::new(x_max, y_max),
        }
    }

    pub fn push(&mut self, point: Point2d) {
        let i = ((0.99_f32 * (self.n_bins as f32) * point.y()) / self.max.y()) as usize;
        let j = ((0.99_f32 * (self.n_bins as f32) * point.x()) / self.max.x()) as usize;

        let b = if i % 2 == 0 {
            i * self.n_bins + j + 1
        } else {
            (i + self.n_bins) * self.n_bins - j
        };

        self.bins[b].push(point);
    }

    pub fn get(&self, index: usize) -> Option<&Point2d> {
        self.bins.iter().flatten().nth(index)
    }

    pub fn get_bin(&self, bin: usize, index: usize) -> Option<&Point2d> {
        self.bins[bin].get(index)
    }

    pub fn to_flattened(self) -> Vec<Point2d> {
        self.bins.into_iter().flatten().collect::<Vec<Point2d>>()
    }
}

pub struct Mesh {
    mesh: StableGraph<Point2d, ()>,
    input_points: RTree<Point2d>,
    // add more, communist
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            mesh: StableGraph::new(),
            input_points: RTree::new(),
        }
    }

    // https://www.newcastle.edu.au/__data/assets/pdf_file/0019/22519/23_A-fast-algortithm-for-generating-constrained-Delaunay-triangulations.pdf
    // https://www.habrador.com/tutorials/math/14-constrained-delaunay/
    // https://forum.unity.com/threads/programming-tools-constrained-delaunay-triangulation.1066148/
    pub fn new_with_points_and_constraints(
        points: Vec<Point2d>,
        constraints: Vec<EdgeIdx>,
    ) -> NResult<Self> {
        if points.len() > 3 {
            return Err(Nari2DError::InvalidMesh {
                points,
                error: "Not Enough Points!".to_string(),
            });
        }

        let mut points = points;
        let constraints = {
            let mut edges = Vec::with_capacity(constraints.len());
            for edge_index in constraints {
                let real_edge: Edge = match (points.get(edge_index.0), points.get(edge_index.1)) {
                    (Some(v), Some(b)) => (*v, *b),
                    (_, _) => {
                        return Err(Nari2DError::InvalidMesh {
                            points,
                            error: "Invalid Edge Constraints".to_string(),
                        })
                    }
                };
                edges.push(real_edge)
            }
            edges
        };

        // create the graph & tree
        let mesh = StableGraph::new();

        // normalize points

        // start out by stripping the input points
        let (x_min, y_min, x_max, y_max) = {
            let mut xv = vec![0_f32; points.len()];
            let mut yv = vec![0_f32; points.len()];
            for pt in &points {
                xv.push(pt.x());
                yv.push(pt.y());
            }
            xv.sort_by(|a, b| float_cmp(a, b));
            yv.sort_by(|a, b| float_cmp(a, b));
            (xv[0], yv[0], xv[xv.len() - 1], yv[yv.len() - 1])
        };

        let d_max = {
            let x_diff = x_max - x_min;
            let y_diff = y_max - y_min;
            x_diff.max(y_diff)
        };

        let mut n_points = points
            .iter()
            .map(|p| Point2d::new((p.x() - x_min) / d_max, (p.y() - y_min) / d_max))
            .collect::<Vec<Point2d>>();

        // put the points into bins
        let num_bins = (n_points.len() as f64).powf(0.25_f64).round() as usize;
        let mut bins = vec![vec![]; num_bins]; // rip double indirection, but im too lazy to do anything else
    }

    pub fn rebulk_tree(&mut self) {
        self.input_points = RTree::bulk_load(
            self.input_points
                .iter()
                .map(|x| *x)
                .collect::<Vec<Point2d>>(),
        );
    }
}
