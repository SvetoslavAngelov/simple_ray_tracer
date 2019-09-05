mod vec;
mod ray;
mod shape;
extern crate image;

type Vec3 = vec::Vec3;
type Ray = ray::Ray;
type Shapes = shape::Shapes; 
type Sphere = shape::Sphere;
type Record = shape::Record;

fn colour(r: &Ray, shapes: &Shapes) -> Vec3 {
    use shape::Hitable;
    let mut rec: Record = Record::default();

    if shapes.hit(&mut rec, r, 0.0, 10000.0){
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5; 
    }
    else {
        let unit_direction: Vec3 = vec::make_unit_vector(&r.direction());
        let t: f32 = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) 
                + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn render() {
    const WIDTH: u32 = 200; 
    const HEIGHT: u32 = 100; 

    let mut framebuffer = image::ImageBuffer::new(WIDTH, HEIGHT);
    let upper_left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let shapes: Shapes = Shapes::new(vec![
        Sphere::new(Vec3::new(0.0, 101.0, -1.0), 100.0), 
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)], 2);

    for i in 0..WIDTH{
        for j in 0..HEIGHT{
            let u: f32 = i as f32 / WIDTH as f32;
            let v: f32 = j as f32 / HEIGHT as f32;
            let r: Ray = Ray::new(&origin, &(upper_left_corner + horizontal * u + vertical * v));
            let colour: Vec3 = colour(&r, &shapes);
            framebuffer[(i,j)] = image::Rgb([(colour.x() * 255.0) as u8, (colour.y() * 255.0) as u8, (colour.z() * 255.0) as u8]);
        }
    }
    framebuffer.save("output.png").unwrap();
}

fn main() {
    render();
}