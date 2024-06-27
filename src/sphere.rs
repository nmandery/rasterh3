use geo_types::{LineString, Polygon, Rect};

/// earth radius at the equator in meters
const EARTH_RADIUS_EQUATOR: f64 = 6_378_137_f64;

/// Calculate the approximate area of the given linestring ring (wgs84 coordinates) in square meters
///
/// Roughly taken from [stackoverflow](https://gis.stackexchange.com/questions/711/how-can-i-measure-area-from-geographic-coordinates).
///
/// Published in Chamberlain, R. and W. Duquette. “Some algorithms for polygons on a sphere.” (2007).
/// The full paper is available [here](https://www.semanticscholar.org/paper/Some-algorithms-for-polygons-on-a-sphere.-Chamberlain-Duquette/79668c0fe32788176758a2285dd674fa8e7b8fa8).
pub trait AreaOnSphere {
    fn area_on_sphere_m2(&self) -> f64;
}

impl AreaOnSphere for LineString<f64> {
    fn area_on_sphere_m2(&self) -> f64 {
        if !self.is_closed() {
            return 0.0;
        }
        self.0
            .windows(2)
            .map(|coords| {
                (coords[1].x - coords[0].x).to_radians()
                    * (2.0 + coords[0].y.to_radians().sin() + coords[1].y.to_radians().sin())
            })
            .sum::<f64>()
            .abs()
            * EARTH_RADIUS_EQUATOR.powi(2)
            / 2.0
    }
}

impl AreaOnSphere for Polygon<f64> {
    fn area_on_sphere_m2(&self) -> f64 {
        let mut area = self.exterior().area_on_sphere_m2();
        for hole in self.interiors().iter() {
            area -= hole.area_on_sphere_m2();
        }
        area.max(0.0)
    }
}

impl AreaOnSphere for Rect<f64> {
    fn area_on_sphere_m2(&self) -> f64 {
        self.to_polygon().area_on_sphere_m2()
    }
}
