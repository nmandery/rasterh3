use geo_types::{Rect, coord};

/// Normalize a longitude to coordinate to ensure it's within [-180,180]
#[inline(always)]
fn normalize_longitude(longitude: f64) -> f64 {
    ((longitude + 540.0f64) % 360.0f64) - 180.0f64
}

#[derive(Debug)]
pub(crate) struct SplittedRect {
    pub(crate) rect: Rect,
    pub(crate) difference_due_to_antimeridian_split: f64,
}

pub(crate) fn split_rect_at_antimeridian(rect: Rect) -> Vec<SplittedRect> {
    let min_x = rect.min().x;
    let max_x = rect.max().x;
    let min_x_normalized = normalize_longitude(min_x);
    let max_x_normalized = normalize_longitude(max_x);

    let mut splits = Vec::new();

    if min_x_normalized <= max_x_normalized {
        // The rect does not wrap around the antimeridian. Normalize it into
        // [-180, 180] as a single piece. This also covers rects lying entirely
        // outside [-180, 180] (e.g. a tile fully east of +180), which are simply
        // shifted back into the valid range.
        splits.push(SplittedRect {
            rect: Rect::new(
                coord! {x: min_x_normalized, y: rect.min().y},
                coord! {x: max_x_normalized, y: rect.max().y},
            ),
            difference_due_to_antimeridian_split: min_x - min_x_normalized,
        });
    } else {
        // The rect crosses the antimeridian; split it into two normalized pieces.
        //
        // `difference_due_to_antimeridian_split` converts a normalized cell
        // centroid back to the original (un-normalized) longitude expected by the
        // inverse affine transform, i.e. `original - normalized`.
        //
        // Western piece: the part originally east of +180 (normalized near -180).
        if max_x_normalized > -180.0 {
            splits.push(SplittedRect {
                rect: Rect::new(
                    coord! {x: -180.0, y: rect.min().y},
                    coord! {x: max_x_normalized, y: rect.max().y},
                ),
                difference_due_to_antimeridian_split: max_x - max_x_normalized,
            });
        }
        // Eastern piece: the part originally west of -180 (normalized near +180).
        if min_x_normalized < 180.0 {
            splits.push(SplittedRect {
                rect: Rect::new(
                    coord! {x: min_x_normalized, y: rect.min().y},
                    coord! {x: 180.0, y: rect.max().y},
                ),
                difference_due_to_antimeridian_split: min_x - min_x_normalized,
            });
        }
    }

    splits
}

#[cfg(test)]
mod tests {
    use crate::util::split_rect_at_antimeridian;
    use geo_types::{Rect, coord};

    #[test]
    fn test_split_rect_at_antimeridian_not_crossing() {
        let rect = Rect::new(coord! {x: 45.0, y:12.0}, coord! {x:67.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect);
        assert_eq!(splitted.len(), 1);
        assert_eq!(splitted[0].rect, rect);
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 0.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_lower() {
        let rect = Rect::new(coord! {x: -185.0, y:12.0}, coord! {x:-178.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect);
        assert_eq!(splitted.len(), 2);
        assert_eq!(
            splitted[0].rect,
            Rect::new(coord! {x: -180.0, y:12.0}, coord! {x: -178.0, y: 23.0})
        );
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 0.0);
        assert_eq!(
            splitted[1].rect,
            Rect::new(coord! {x: 175.0, y:12.0}, coord! {x: 180.0, y: 23.0})
        );
        assert_eq!(splitted[1].difference_due_to_antimeridian_split, -360.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_upper() {
        let rect = Rect::new(coord! {x: 185.0, y:12.0}, coord! {x:178.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect);
        assert_eq!(splitted.len(), 2);
        assert_eq!(
            splitted[0].rect,
            Rect::new(coord! {x: -180.0, y:12.0}, coord! {x: -175.0, y: 23.0})
        );
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 360.0);
        assert_eq!(
            splitted[1].rect,
            Rect::new(coord! {x: 178.0, y:12.0}, coord! {x: 180.0, y: 23.0})
        );
        assert_eq!(splitted[1].difference_due_to_antimeridian_split, 0.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_fully_east() {
        // a tile lying entirely east of +180 must be normalized into a single
        // piece within [-180, 180] instead of being passed through unchanged
        let rect = Rect::new(coord! {x: 180.5, y: 12.0}, coord! {x: 181.5, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect);
        assert_eq!(splitted.len(), 1);
        assert_eq!(
            splitted[0].rect,
            Rect::new(coord! {x: -179.5, y: 12.0}, coord! {x: -178.5, y: 23.0})
        );
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 360.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_exact_180() {
        // a rect ending exactly at +180 must not produce a zero-width split
        // (normalize_longitude maps +180 to -180)
        let rect = Rect::new(coord! {x: 178.0, y: 12.0}, coord! {x: 180.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect);
        assert_eq!(splitted.len(), 1);
        assert_eq!(
            splitted[0].rect,
            Rect::new(coord! {x: 178.0, y: 12.0}, coord! {x: 180.0, y: 23.0})
        );
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 0.0);
    }

    fn assert_split_roundtrips(rect: Rect) {
        let min_x = rect.min().x;
        let max_x = rect.max().x;
        let splitted = split_rect_at_antimeridian(rect);
        assert!(!splitted.is_empty(), "no splits produced for {rect:?}");
        for s in &splitted {
            // every split must lie within the valid longitude range ...
            assert!(s.rect.min().x >= -180.0, "min x {} < -180", s.rect.min().x);
            assert!(s.rect.min().x <= 180.0, "min x {} > 180", s.rect.min().x);
            assert!(s.rect.max().x >= -180.0, "max x {} < -180", s.rect.max().x);
            assert!(s.rect.max().x <= 180.0, "max x {} > 180", s.rect.max().x);
            // ... and must have a non-zero width (no degenerate polygons)
            assert!(s.rect.min().x < s.rect.max().x, "zero-width split {s:?}");
            // every endpoint must map back into the original rect's x-range
            for &x_norm in &[s.rect.min().x, s.rect.max().x] {
                let x_orig = x_norm + s.difference_due_to_antimeridian_split;
                assert!(
                    x_orig >= min_x - 1e-9 && x_orig <= max_x + 1e-9,
                    "roundtrip failed: {x_norm} + {} = {x_orig} not in [{min_x}, {max_x}]",
                    s.difference_due_to_antimeridian_split
                );
            }
        }
    }

    #[test]
    fn test_split_rect_at_antimeridian_roundtrip() {
        // not crossing
        assert_split_roundtrips(Rect::new(
            coord! {x: 45.0, y: 12.0},
            coord! {x: 67.0, y: 23.0},
        ));
        // crossing the antimeridian (west side)
        assert_split_roundtrips(Rect::new(
            coord! {x: -185.0, y: 12.0},
            coord! {x: -178.0, y: 23.0},
        ));
        // crossing the antimeridian (east side)
        assert_split_roundtrips(Rect::new(
            coord! {x: 178.0, y: 12.0},
            coord! {x: 185.0, y: 23.0},
        ));
        // entirely east of +180
        assert_split_roundtrips(Rect::new(
            coord! {x: 180.5, y: 12.0},
            coord! {x: 181.5, y: 23.0},
        ));
        // entirely west of -180
        assert_split_roundtrips(Rect::new(
            coord! {x: -181.5, y: 12.0},
            coord! {x: -180.5, y: 23.0},
        ));
        // ending exactly at +180
        assert_split_roundtrips(Rect::new(
            coord! {x: 178.0, y: 12.0},
            coord! {x: 180.0, y: 23.0},
        ));
        // starting exactly at -180
        assert_split_roundtrips(Rect::new(
            coord! {x: -180.0, y: 12.0},
            coord! {x: -178.0, y: 23.0},
        ));
    }
}
