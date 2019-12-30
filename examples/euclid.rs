use euclid::Box3D;
use euclid::Point2D;
use euclid::Point3D;
use euclid::Rect;
use euclid::Size2D;
use euclid::Trig;

fn main() {
    let (x0, y0, z0) = (3.0, 5.0, 4.0);
    let (x1, y1, z1) = (3.0, 5.0, 4.0);
    let r1 = Box3D::<f64, f64>::new(Point3D::new(x0, y0, z0), Point3D::new(x1, y1, z1));
    let r2 = r1.clone().inflate(2.0, 0.5, 1.0);
    assert!(r1.intersects(&r2), true);
}
