//! Axis-aligned rectangle intersection detection.
//!
//! Uses a sweep-line algorithm to efficiently find all overlapping
//! pairs of axis-aligned rectangles.

use crate::event::Point;

/// An axis-aligned rectangle defined by its bottom-left and top-right corners.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// Bottom-left corner (min x, min y).
    pub min: Point,
    /// Top-right corner (max x, max y).
    pub max: Point,
    /// Rectangle ID.
    pub id: usize,
}

impl Rect {
    /// Create a new axis-aligned rectangle.
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, id: usize) -> Self {
        Self {
            min: Point::new(x1.min(x2), y1.min(y2)),
            max: Point::new(x1.max(x2), y1.max(y2)),
            id,
        }
    }

    /// Check if this rectangle overlaps with another.
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    /// Area of this rectangle.
    pub fn area(&self) -> f64 {
        (self.max.x - self.min.x) * (self.max.y - self.min.y)
    }

    /// Check if this rectangle contains a point.
    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Intersection of two rectangles (if they overlap).
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.overlaps(other) {
            return None;
        }
        Some(Rect::new(
            self.min.x.max(other.min.x),
            self.min.y.max(other.min.y),
            self.max.x.min(other.max.x),
            self.max.y.min(other.max.y),
            0,
        ))
    }
}

/// Find all pairs of overlapping rectangles using sweep-line.
///
/// Returns pairs of rectangle IDs that overlap.
pub fn find_overlapping(rects: &[Rect]) -> Vec<(usize, usize)> {
    if rects.is_empty() {
        return Vec::new();
    }

    // Create events: rectangle start (left edge) and end (right edge)
    let mut events: Vec<(f64, bool, usize)> = Vec::new(); // (x, is_start, rect_id)
    for r in rects {
        events.push((r.min.x, true, r.id));
        events.push((r.max.x, false, r.id));
    }
    events.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut active: Vec<usize> = Vec::new();
    let mut results: Vec<(usize, usize)> = Vec::new();

    for (_x, is_start, rect_id) in events {
        if is_start {
            // Check for y-overlap with all active rectangles
            let current = rects.iter().find(|r| r.id == rect_id).unwrap();
            for &active_id in &active {
                let active_rect = rects.iter().find(|r| r.id == active_id).unwrap();
                if current.overlaps(active_rect) {
                    let pair = if rect_id < active_id {
                        (rect_id, active_id)
                    } else {
                        (active_id, rect_id)
                    };
                    if !results.contains(&pair) {
                        results.push(pair);
                    }
                }
            }
            active.push(rect_id);
        } else {
            active.retain(|&id| id != rect_id);
        }
    }

    results.sort();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_creation() {
        let r = Rect::new(1.0, 2.0, 3.0, 4.0, 0);
        assert_eq!(r.min.x, 1.0);
        assert_eq!(r.max.x, 3.0);
    }

    #[test]
    fn test_rect_creation_unordered() {
        let r = Rect::new(3.0, 4.0, 1.0, 2.0, 0);
        assert_eq!(r.min.x, 1.0);
        assert_eq!(r.max.x, 3.0);
    }

    #[test]
    fn test_overlapping() {
        let r1 = Rect::new(0.0, 0.0, 2.0, 2.0, 0);
        let r2 = Rect::new(1.0, 1.0, 3.0, 3.0, 1);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn test_non_overlapping() {
        let r1 = Rect::new(0.0, 0.0, 1.0, 1.0, 0);
        let r2 = Rect::new(2.0, 2.0, 3.0, 3.0, 1);
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn test_touching_edges() {
        let r1 = Rect::new(0.0, 0.0, 1.0, 1.0, 0);
        let r2 = Rect::new(1.0, 0.0, 2.0, 1.0, 1);
        // Touching edges don't count as overlap
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn test_area() {
        let r = Rect::new(0.0, 0.0, 5.0, 3.0, 0);
        assert!((r.area() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_contains_point() {
        let r = Rect::new(0.0, 0.0, 5.0, 5.0, 0);
        assert!(r.contains_point(&Point::new(2.0, 3.0)));
        assert!(!r.contains_point(&Point::new(6.0, 3.0)));
    }

    #[test]
    fn test_intersection() {
        let r1 = Rect::new(0.0, 0.0, 3.0, 3.0, 0);
        let r2 = Rect::new(1.0, 1.0, 4.0, 4.0, 1);
        let inter = r1.intersection(&r2).unwrap();
        assert!((inter.min.x - 1.0).abs() < 1e-10);
        assert!((inter.max.x - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_find_overlapping_basic() {
        let rects = vec![
            Rect::new(0.0, 0.0, 2.0, 2.0, 0),
            Rect::new(1.0, 1.0, 3.0, 3.0, 1),
            Rect::new(5.0, 5.0, 6.0, 6.0, 2),
        ];
        let overlaps = find_overlapping(&rects);
        assert_eq!(overlaps.len(), 1);
        assert_eq!(overlaps[0], (0, 1));
    }

    #[test]
    fn test_find_overlapping_none() {
        let rects = vec![
            Rect::new(0.0, 0.0, 1.0, 1.0, 0),
            Rect::new(2.0, 2.0, 3.0, 3.0, 1),
        ];
        assert!(find_overlapping(&rects).is_empty());
    }

    #[test]
    fn test_find_overlapping_all() {
        let rects = vec![
            Rect::new(0.0, 0.0, 3.0, 3.0, 0),
            Rect::new(1.0, 1.0, 4.0, 4.0, 1),
            Rect::new(2.0, 2.0, 5.0, 5.0, 2),
        ];
        let overlaps = find_overlapping(&rects);
        assert_eq!(overlaps.len(), 3);
    }

    #[test]
    fn test_empty_rects() {
        assert!(find_overlapping(&[]).is_empty());
    }
}
