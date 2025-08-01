use super::*;

pub fn use_element_center(identifier: &'static str) -> Signal<Point2D> {
    let coordinate = use_element_coordinate(identifier);
    let dimension = use_element_dimension(identifier);
    let coordinate_center = use_signal(|| point_2d::Point2D {
        x: coordinate().x + dimension().x() / 2.0,
        y: coordinate().y + dimension().y() / 2.0
    });

    coordinate_center
}