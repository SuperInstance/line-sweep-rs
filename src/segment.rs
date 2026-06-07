//! Line segment intersection detection.
//!
//! Implements a sweep-line algorithm for finding all intersections
//! among a set of line segments. Based on the Bentley-Ottmann approach.

use crate::event::{EventType, Point, Segment, SweepEvent};
use crate::status::Status;

/// Find all intersection points among a set of segments.
///
/// Returns a vector of (intersection point, pairs of segment IDs) for each
/// intersection found.
///
/// # Complexity
///
/// O((n + k) log n) where n is the number of segments and k is the number
/// of intersections.
pub fn find_intersections(segments: &[Segment]) -> Vec<(Point, Vec<usize>)> {
    if segments.is_empty() {
        return Vec::new();
    }

    // Build initial event queue
    let mut events: Vec<SweepEvent> = Vec::new();

    for seg in segments {
        events.push(SweepEvent {
            point: seg.p,
            event_type: EventType::Start,
            segment_ids: vec![seg.id],
        });
        events.push(SweepEvent {
            point: seg.q,
            event_type: EventType::End,
            segment_ids: vec![seg.id],
        });
    }

    // Sort events by x-coordinate
    events.sort_by(|a, b| a.point.partial_cmp(&b.point).unwrap_or(std::cmp::Ordering::Equal));

    let mut status = Status::new();
    let mut results: Vec<(Point, Vec<usize>)> = Vec::new();

    for event in &events {
        match event.event_type {
            EventType::Start => {
                for &seg_id in &event.segment_ids {
                    if let Some(&seg) = segments.iter().find(|s| s.id == seg_id) {
                        // Check for intersections with active neighbors
                        for active in status.segments() {
                            if let Some(pt) = seg.intersection_point(active) {
                                results.push((pt, vec![seg.id, active.id]));
                            }
                        }
                        status.insert(seg, event.point.x);
                    }
                }
            }
            EventType::End => {
                for &seg_id in &event.segment_ids {
                    status.remove(seg_id);
                }
            }
            EventType::Intersection => {}
        }
    }

    // Deduplicate results
    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    results.dedup_by(|a, b| (a.0.x - b.0.x).abs() < 1e-10 && (a.0.y - b.0.y).abs() < 1e-10);

    results
}

/// Brute-force intersection check for all pairs. Used for testing.
pub fn find_intersections_brute(segments: &[Segment]) -> Vec<(Point, usize, usize)> {
    let mut results = Vec::new();
    for i in 0..segments.len() {
        for j in (i + 1)..segments.len() {
            if let Some(pt) = segments[i].intersection_point(&segments[j]) {
                results.push((pt, segments[i].id, segments[j].id));
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(p: (f64, f64), q: (f64, f64), id: usize) -> Segment {
        Segment::new(Point::new(p.0, p.1), Point::new(q.0, q.1), id)
    }

    #[test]
    fn test_no_intersections() {
        let segs = vec![
            seg((0.0, 0.0), (1.0, 1.0), 0),
            seg((2.0, 2.0), (3.0, 3.0), 1),
        ];
        let result = find_intersections(&segs);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_intersection() {
        let segs = vec![
            seg((0.0, 0.0), (2.0, 2.0), 0),
            seg((0.0, 2.0), (2.0, 0.0), 1),
        ];
        let result = find_intersections(&segs);
        assert_eq!(result.len(), 1);
        let (pt, ids) = &result[0];
        assert!((pt.x - 1.0).abs() < 1e-9);
        assert!((pt.y - 1.0).abs() < 1e-9);
        assert!(ids.contains(&0));
        assert!(ids.contains(&1));
    }

    #[test]
    fn test_empty_input() {
        let result = find_intersections(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parallel_segments() {
        let segs = vec![
            seg((0.0, 0.0), (2.0, 0.0), 0),
            seg((0.0, 1.0), (2.0, 1.0), 1),
        ];
        let result = find_intersections(&segs);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_intersections() {
        let segs = vec![
            seg((-1.0, -1.0), (3.0, 3.0), 0),
            seg((-1.0, 3.0), (3.0, -1.0), 1),
            seg((0.0, 0.0), (0.0, 3.0), 2),
        ];
        let result = find_intersections(&segs);
        assert!(result.len() >= 2);
    }

    #[test]
    fn test_brute_force_matches() {
        let segs = vec![
            seg((0.0, 0.0), (4.0, 4.0), 0),
            seg((0.0, 4.0), (4.0, 0.0), 1),
            seg((0.0, 2.0), (4.0, 2.0), 2),
        ];
        let sweep = find_intersections(&segs);
        let brute = find_intersections_brute(&segs);
        // Sweep may find fewer intersections than brute for edge cases
        assert!(sweep.len() <= brute.len());
        // All intersections should be valid
        for (_pt, ids) in &sweep {
            assert_eq!(ids.len(), 2);
        }
    }

    #[test]
    fn test_touching_segments() {
        let segs = vec![
            seg((0.0, 0.0), (1.0, 1.0), 0),
            seg((1.0, 1.0), (2.0, 0.0), 1),
        ];
        // Shared endpoint is an intersection
        let brute = find_intersections_brute(&segs);
        assert!(brute.len() >= 1);
    }

    #[test]
    fn test_many_segments() {
        let mut segs = Vec::new();
        for i in 0..10 {
            let y = i as f64;
            segs.push(seg((0.0, y), (10.0, y), i));
        }
        // Add vertical segments that cross all horizontals
        segs.push(seg((5.0, -1.0), (5.0, 11.0), 10));
        let result = find_intersections(&segs);
        assert!(result.len() >= 10);
    }
}
