//! Status structure for the line sweep algorithm.
//!
//! Maintains an ordered set of active segments during the sweep,
//! supporting insert, remove, and neighbor queries.

use crate::event::Segment;

/// A simple status structure using a sorted vector of segments.
///
/// In a production implementation this would be a balanced BST (e.g., red-black tree),
/// but a sorted vector suffices for correctness and clarity.
pub struct Status {
    segments: Vec<Segment>,
}

impl Status {
    /// Create an empty status structure.
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    /// Insert a segment into the status structure.
    /// Maintains sorted order by the segment's y-position at the current sweep x.
    pub fn insert(&mut self, segment: Segment, sweep_x: f64) {
        let y = segment_y_at_x(&segment, sweep_x);
        let pos = self.segments.iter().position(|s| {
            segment_y_at_x(s, sweep_x) > y
        }).unwrap_or(self.segments.len());
        self.segments.insert(pos, segment);
    }

    /// Remove a segment by its ID.
    pub fn remove(&mut self, segment_id: usize) {
        self.segments.retain(|s| s.id != segment_id);
    }

    /// Get the segments currently in the status structure.
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    /// Find the neighbors of a given segment ID at the current sweep position.
    /// Returns (above, below) segment IDs.
    pub fn neighbors(&self, segment_id: usize) -> (Option<usize>, Option<usize>) {
        let idx = self.segments.iter().position(|s| s.id == segment_id);
        match idx {
            Some(i) => {
                let above = if i > 0 { Some(self.segments[i - 1].id) } else { None };
                let below = if i + 1 < self.segments.len() { Some(self.segments[i + 1].id) } else { None };
                (above, below)
            }
            None => (None, None),
        }
    }

    /// Number of active segments.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Whether the status is empty.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute the y-coordinate of a segment at a given x value.
fn segment_y_at_x(seg: &Segment, x: f64) -> f64 {
    if (seg.q.x - seg.p.x).abs() < 1e-12 {
        // Vertical segment: use the midpoint y
        (seg.p.y + seg.q.y) / 2.0
    } else {
        let t = (x - seg.p.x) / (seg.q.x - seg.p.x);
        seg.p.y + t * (seg.q.y - seg.p.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Point;

    #[test]
    fn test_insert_and_len() {
        let mut s = Status::new();
        s.insert(Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0), 0), 1.0);
        s.insert(Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0), 1), 1.0);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn test_remove() {
        let mut s = Status::new();
        s.insert(Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0), 0), 1.0);
        s.remove(0);
        assert!(s.is_empty());
    }

    #[test]
    fn test_neighbors() {
        let mut s = Status::new();
        s.insert(Segment::new(Point::new(0.0, 3.0), Point::new(2.0, 3.0), 0), 1.0);
        s.insert(Segment::new(Point::new(0.0, 1.0), Point::new(2.0, 1.0), 1), 1.0);
        s.insert(Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 2.0), 2), 1.0);
        let (above, below) = s.neighbors(2);
        assert!(above.is_some());
        assert!(below.is_some());
    }

    #[test]
    fn test_default() {
        let s = Status::default();
        assert!(s.is_empty());
    }
}
