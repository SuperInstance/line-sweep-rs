//! Sweep event types for the line sweep algorithm.
//!
//! Defines the events that drive the sweep: segment start, end, and intersection.

/// A 2D point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// x-coordinate.
    pub x: f64,
    /// y-coordinate.
    pub y: f64,
}

impl Point {
    /// Create a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Squared distance to another point.
    pub fn dist_sq(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Euclidean distance to another point.
    pub fn distance(&self, other: &Point) -> f64 {
        self.dist_sq(other).sqrt()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}

impl Ord for Point {
    /// Lexicographic ordering: x first, then y.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.partial_cmp(&other.x) {
            Some(std::cmp::Ordering::Equal) | None => {}
            Some(ord) => return ord,
        }
        self.y.partial_cmp(&other.y).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// A line segment defined by two endpoints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// Start point (leftmost, or bottommost if vertical).
    pub p: Point,
    /// End point.
    pub q: Point,
    /// Segment ID for tracking.
    pub id: usize,
}

impl Segment {
    /// Create a new segment, automatically ordering endpoints.
    pub fn new(p: Point, q: Point, id: usize) -> Self {
        if (p.x, p.y) <= (q.x, q.y) {
            Self { p, q, id }
        } else {
            Self { p: q, q: p, id }
        }
    }

    /// Check if this segment intersects another using parametric intersection.
    pub fn intersects(&self, other: &Segment) -> bool {
        self.intersection_point(other).is_some()
    }

    /// Compute the intersection point of two segments, if any.
    pub fn intersection_point(&self, other: &Segment) -> Option<Point> {
        let d1x = self.q.x - self.p.x;
        let d1y = self.q.y - self.p.y;
        let d2x = other.q.x - other.p.x;
        let d2y = other.q.y - other.p.y;

        let cross = d1x * d2y - d1y * d2x;
        if cross.abs() < 1e-12 {
            // Parallel or collinear
            return None;
        }

        let dx = other.p.x - self.p.x;
        let dy = other.p.y - self.p.y;

        let t = (dx * d2y - dy * d2x) / cross;
        let u = (dx * d1y - dy * d1x) / cross;

        if (-1e-12..=1.0 + 1e-12).contains(&t) && (-1e-12..=1.0 + 1e-12).contains(&u) {
            Some(Point::new(self.p.x + t * d1x, self.p.y + t * d1y))
        } else {
            None
        }
    }
}

/// Types of sweep events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// Segment begins at this point.
    Start,
    /// Segment ends at this point.
    End,
    /// Two segments intersect at this point.
    Intersection,
}

/// A sweep event at a given point.
#[derive(Debug, Clone)]
pub struct SweepEvent {
    /// Location of the event.
    pub point: Point,
    /// Type of event.
    pub event_type: EventType,
    /// Segment ID(s) involved.
    pub segment_ids: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_ordering() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(2.0, 1.0);
        assert!(a < b);
    }

    #[test]
    fn test_segment_creation_orders_endpoints() {
        let s = Segment::new(Point::new(5.0, 5.0), Point::new(1.0, 1.0), 0);
        assert!(s.p.x <= s.q.x);
    }

    #[test]
    fn test_segment_intersect_cross() {
        let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0), 0);
        let s2 = Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0), 1);
        assert!(s1.intersects(&s2));
    }

    #[test]
    fn test_segment_no_intersect() {
        let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0), 0);
        let s2 = Segment::new(Point::new(3.0, 3.0), Point::new(4.0, 4.0), 1);
        assert!(!s1.intersects(&s2));
    }

    #[test]
    fn test_intersection_point() {
        let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0), 0);
        let s2 = Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0), 1);
        let pt = s1.intersection_point(&s2).unwrap();
        assert!((pt.x - 1.0).abs() < 1e-9);
        assert!((pt.y - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_parallel_no_intersect() {
        let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), 0);
        let s2 = Segment::new(Point::new(0.0, 1.0), Point::new(1.0, 1.0), 1);
        assert!(!s1.intersects(&s2));
    }

    #[test]
    fn test_shared_endpoint() {
        let s1 = Segment::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0), 0);
        let s2 = Segment::new(Point::new(1.0, 1.0), Point::new(2.0, 0.0), 1);
        assert!(s1.intersects(&s2));
    }
}
