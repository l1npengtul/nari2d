use crate::error::mesh::{MResult, MeshError};
use crate::error::util::{IndexOrValue, IndexType};
use nari2d_traits::index::NumIndex;
use staticvec::{staticvec, StaticVec};
use std::cmp::Ordering;
use std::collections::BTreeSet;

pub type PointRef = u32;

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Ord, Eq)]
#[repr(C)]
pub struct Triangle {
    p1: u32,
    p2: u32,
    p3: u32,
}

impl Triangle {
    #[inline]
    pub fn new(p1: u32, p2: u32, p3: u32) -> Self {
        let mut unsorted: StaticVec<u32, 3> = staticvec![p1, p2, p3];
        let sorted = unsorted.sorted();

        Triangle {
            p1: sorted[0],
            p2: sorted[1],
            p3: sorted[2],
        }
    }

    #[inline]
    pub fn p1(&self) -> PointRef {
        self.p1
    }

    #[inline]
    pub fn set_p1(&mut self, p1: u32) {
        self.p1 = p1;
        self.sort();
    }

    #[inline]
    pub fn p2(&self) -> PointRef {
        self.p2
    }

    #[inline]
    pub fn set_p2(&mut self, p2: u32) {
        self.p2 = p2;
        self.sort();
    }

    #[inline]
    pub fn p3(&self) -> PointRef {
        self.p3
    }

    #[inline]
    pub fn set_p3(&mut self, p3: u32) {
        self.p3 = p3;
        self.sort();
    }

    #[inline]
    pub fn sort(&mut self) {
        let mut unsorted: StaticVec<u32, 3> = staticvec![p1, p2, p3];
        let sorted = unsorted.sorted();

        self.set_p1(sorted[0]);
        self.set_p2(sorted[1]);
        self.set_p3(sorted[2]);
    }

    #[inline]
    pub fn edges(&self) -> [Edge; 3] {
        [
            Edge::new(self.p1, self.p2),
            Edge::new(self.p2, self.p3),
            Edge::new(self.p1, self.p3),
        ]
    }

    #[inline]
    pub fn points(&self) -> [PointRef; 3] {
        [self.p1, self.p2, self.p3]
    }

    #[inline]
    pub fn non_edge_point(&self, edge: &Edge) -> MResult<u32> {
        let points: StaticVec<u32, 3> = staticvec![p1, p2, p3];
        let mut filtered = points
            .into_iter()
            .filter(|x| x != edge.start() && x != edge.end());
        return if filtered.count() != 1 {
            Err(MeshError::NonEdgeImproperEdge {
                triangle: *self,
                edge: *edge,
            })
        } else {
            // SAFETY: We already do a bounds check (filtered.len() != 1), so this operation is garunteed to be safe.
            Ok(unsafe { filtered.nth(0).unwrap_unchecked() })
        };
    }

    #[inline]
    pub fn opposite_edge(&self, point: PointRef) -> MResult<Edge> {
        let mut pts = BTreeSet::new();
        pts.insert(self.p1);
        pts.insert(self.p2);
        pts.insert(self.p3);

        pts.remove(&point);

        if pts.len() != 2 {
            return Err(MeshError::PointNotFound {
                idx: IndexOrValue::Index(IndexType::U32(point)),
            });
        }
        let mut iterator = pts.into_iter();
        Ok(
            // SAFETY: We did a bounds check (if pts.len != 2)
            unsafe {
                Edge {
                    start: iterator.nth(0).unwrap_unchecked(),
                    end: iterator.nth(0).unwrap_unchecked(),
                }
            },
        )
    }
}

#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Ord, Eq)]
#[repr(C)]
pub struct Edge {
    start: u32,
    end: u32,
}

impl Edge {
    #[inline]
    pub fn new(start: u32, end: u32) -> Self {
        match start.cmp(&end) {
            Ordering::Less | Ordering::Equal => Edge { start, end },
            Ordering::Greater => Edge {
                start: end,
                end: start,
            },
        }
    }

    #[inline]
    pub fn start(&self) -> u32 {
        self.start
    }

    #[inline]
    pub fn set_start(&mut self, start: u32) {
        self.start = start;
        self.sort();
    }

    #[inline]
    pub fn end(&self) -> u32 {
        self.end
    }

    #[inline]
    pub fn set_end(&mut self, end: u32) {
        self.end = end;
        self.sort();
    }

    #[inline]
    pub fn sort(&mut self) {
        if self.start.cmp(&self.end) == Ordering::Greater {
            let tmp = self.start;
            self.set_start(self.end);
            self.set_end(tmp);
        }
    }
}

pub type EdgeRef = u32;

pub type TriangleRef = u32;
