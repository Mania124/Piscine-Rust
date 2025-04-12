pub mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area = x * y;

    let shape_area = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    (area as f64) >= (shape_area * times as f64)
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume = x * y * z;

    let shape_volume = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };

    (volume as f64) >= (shape_volume * times as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1));
        assert_eq!(result, false);
        let result1 = area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3));
        assert_eq!(result1, true);
        let result2 = volume_fit((5, 5, 5), GeometricalVolumes::Sphere, 3, (2, 0, 0));
        assert_eq!(result2, true);
        let result4 = volume_fit((5, 7, 5), GeometricalVolumes::Parallelepiped, 1, (6, 7, 4));
        assert_eq!(result4, true);
    }
}
