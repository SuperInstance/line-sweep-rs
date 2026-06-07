//! Closest pair of points using divide-and-conquer.
//!
//! Runs in O(n log n) time, optimal for this problem.

use crate::event::Point;

/// Find the closest pair of points and return their indices and distance.
///
/// Returns `None` if fewer than 2 points are provided.
pub fn closest_pair(points: &[Point]) -> Option<(usize, usize, f64)> {
    if points.len() < 2 {
        return None;
    }

    // Sort by x-coordinate
    let mut indices: Vec<usize> = (0..points.len()).collect();
    indices.sort_by(|&a, &b| points[a].x.partial_cmp(&points[b].x).unwrap_or(std::cmp::Ordering::Equal));

    let (i, j, d) = closest_rec(points, &indices);
    Some((i, j, d))
}

fn closest_rec(points: &[Point], sorted_x: &[usize]) -> (usize, usize, f64) {
    if sorted_x.len() <= 3 {
        return brute_force(points, sorted_x);
    }

    let mid = sorted_x.len() / 2;
    let left = &sorted_x[..mid];
    let right = &sorted_x[mid..];

    let (li, lj, ld) = closest_rec(points, left);
    let (ri, rj, rd) = closest_rec(points, right);

    let (best_i, best_j, best_d) = if ld < rd { (li, lj, ld) } else { (ri, rj, rd) };

    // Strip of points within best_d of the dividing line
    let mid_x = points[sorted_x[mid]].x;
    let strip: Vec<usize> = sorted_x.iter().copied()
        .filter(|&idx| (points[idx].x - mid_x).abs() < best_d)
        .collect();

    // Check strip (sorted by y in inner loop)
    let mut result_i = best_i;
    let mut result_j = best_j;
    let mut result_d = best_d;

    for i in 0..strip.len() {
        for j in (i + 1)..strip.len() {
            let dy = points[strip[j]].y - points[strip[i]].y;
            if dy >= result_d {
                break;
            }
            let dist = points[strip[i]].dist_sq(&points[strip[j]]).sqrt();
            if dist < result_d {
                result_d = dist;
                result_i = strip[i];
                result_j = strip[j];
            }
        }
    }

    (result_i, result_j, result_d)
}

/// Brute force for small inputs.
fn brute_force(points: &[Point], indices: &[usize]) -> (usize, usize, f64) {
    let mut best_i = indices[0];
    let mut best_j = indices[1];
    let mut best_d = points[indices[0]].distance(&points[indices[1]]);

    for a in 0..indices.len() {
        for b in (a + 1)..indices.len() {
            let d = points[indices[a]].distance(&points[indices[b]]);
            if d < best_d {
                best_d = d;
                best_i = indices[a];
                best_j = indices[b];
            }
        }
    }

    (best_i, best_j, best_d)
}

/// Brute-force closest pair for testing comparison.
pub fn closest_pair_brute(points: &[Point]) -> Option<(usize, usize, f64)> {
    if points.len() < 2 {
        return None;
    }
    let mut best_i = 0;
    let mut best_j = 1;
    let mut best_d = points[0].distance(&points[1]);
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = points[i].distance(&points[j]);
            if d < best_d {
                best_d = d;
                best_i = i;
                best_j = j;
            }
        }
    }
    Some((best_i, best_j, best_d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_closest_pair() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(3.0, 4.0),
            Point::new(1.0, 0.0),
            Point::new(10.0, 10.0),
        ];
        let (i, j, d) = closest_pair(&pts).unwrap();
        assert_eq!(i, 0);
        assert_eq!(j, 2);
        assert!((d - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_two_points() {
        let pts = vec![Point::new(0.0, 0.0), Point::new(1.0, 0.0)];
        let (i, j, d) = closest_pair(&pts).unwrap();
        assert_eq!(i, 0);
        assert_eq!(j, 1);
        assert!((d - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_single_point() {
        assert!(closest_pair(&[Point::new(0.0, 0.0)]).is_none());
    }

    #[test]
    fn test_empty() {
        assert!(closest_pair(&[]).is_none());
    }

    #[test]
    fn test_matches_brute_force() {
        let pts = vec![
            Point::new(2.0, 3.0),
            Point::new(12.0, 30.0),
            Point::new(40.0, 50.0),
            Point::new(5.0, 1.0),
            Point::new(12.0, 10.0),
            Point::new(3.0, 4.0),
        ];
        let result = closest_pair(&pts).unwrap();
        let brute = closest_pair_brute(&pts).unwrap();
        assert!((result.2 - brute.2).abs() < 1e-9);
    }

    #[test]
    fn test_collinear_points() {
        let pts = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(5.0, 0.0),
            Point::new(10.0, 0.0),
        ];
        let (i, j, d) = closest_pair(&pts).unwrap();
        assert!((d - 1.0).abs() < 1e-9);
        assert_eq!((i, j), (0, 1));
    }

    #[test]
    fn test_grid_points() {
        let mut pts = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                pts.push(Point::new(i as f64 * 10.0, j as f64 * 10.0));
            }
        }
        let result = closest_pair(&pts).unwrap();
        let brute = closest_pair_brute(&pts).unwrap();
        assert!((result.2 - brute.2).abs() < 1e-6);
    }

    #[test]
    fn test_identical_points() {
        let pts = vec![
            Point::new(5.0, 5.0),
            Point::new(5.0, 5.0),
            Point::new(10.0, 10.0),
        ];
        let result = closest_pair(&pts).unwrap();
        assert!((result.2).abs() < 1e-9);
    }

    #[test]
    fn test_large_random() {
        let mut pts = Vec::new();
        for i in 0..500 {
            let x = ((i * 7 + 13) % 1000) as f64;
            let y = ((i * 11 + 37) % 1000) as f64;
            pts.push(Point::new(x, y));
        }
        let result = closest_pair(&pts).unwrap();
        let brute = closest_pair_brute(&pts).unwrap();
        assert!((result.2 - brute.2).abs() < 1e-3);
    }
}
