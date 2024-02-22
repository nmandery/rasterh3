use geo_types::{coord, Rect};

/// Normalize a longitude to coordinate to ensure it's within [-180,180]
#[inline(always)]
fn normalize_longitude(longitude: f64) -> f64 {
    ((longitude + 540.0f64) % 360.0f64) - 180.0f64
}

pub(crate) struct SplittedRect {
    pub(crate) rect: Rect,
    pub(crate) difference_due_to_antimeridian_split: f64,
}

pub(crate) fn split_rect_at_antimeridian(rect: Rect) -> Vec<SplittedRect> {
    let min_x_normalized = normalize_longitude(rect.min().x);
    let max_x_normalized = normalize_longitude(rect.max().x);

    if min_x_normalized < max_x_normalized {
        vec![SplittedRect {
            rect,
            difference_due_to_antimeridian_split: 0.0,
        }]
    } else {
        vec![
            SplittedRect {
                rect: Rect::new(
                    coord! {x: -180.0, y: rect.min().y},
                    coord! {x:max_x_normalized, y:rect.max().y},
                ),
                difference_due_to_antimeridian_split: max_x_normalized - rect.max().x,
            },
            SplittedRect {
                rect: Rect::new(
                    coord! {x: min_x_normalized, y: rect.min().y},
                    coord! {x:180.0, y:rect.max().y},
                ),
                difference_due_to_antimeridian_split: min_x_normalized - rect.min().x,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::util::split_rect_at_antimeridian;
    use geo_types::{coord, Rect};

    #[test]
    fn test_split_rect_at_antimeridian_not_crossing() {
        let rect = Rect::new(coord! {x: 45.0, y:12.0}, coord! {x:67.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect.clone());
        assert_eq!(splitted.len(), 1);
        assert_eq!(splitted[0].rect, rect);
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, 0.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_lower() {
        let rect = Rect::new(coord! {x: -185.0, y:12.0}, coord! {x:-178.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect.clone());
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
        assert_eq!(splitted[1].difference_due_to_antimeridian_split, 360.0);
    }

    #[test]
    fn test_split_rect_at_antimeridian_upper() {
        let rect = Rect::new(coord! {x: 185.0, y:12.0}, coord! {x:178.0, y: 23.0});
        let splitted = split_rect_at_antimeridian(rect.clone());
        assert_eq!(splitted.len(), 2);
        assert_eq!(
            splitted[0].rect,
            Rect::new(coord! {x: -180.0, y:12.0}, coord! {x: -175.0, y: 23.0})
        );
        assert_eq!(splitted[0].difference_due_to_antimeridian_split, -360.0);
        assert_eq!(
            splitted[1].rect,
            Rect::new(coord! {x: 178.0, y:12.0}, coord! {x: 180.0, y: 23.0})
        );
        assert_eq!(splitted[1].difference_due_to_antimeridian_split, 0.0);
    }
}
