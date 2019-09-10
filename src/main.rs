mod vec;
mod ray;
mod shape;
mod camera;
mod material;

extern crate rand;
extern crate image;

type Vec3 = vec::Vec3;
type Ray = ray::Ray;
type Shapes = shape::Shapes; 
type Sphere = shape::Sphere;
type Record = shape::Record;
type Camera = camera::Camera;

fn colour(r: &Ray, shapes: &Shapes) -> Vec3 {
    use shape::Hitable;
    use vec::random_unit_sphere;

    let mut rec: Record = Record::default();

    if shapes.hit(&mut rec, r, 0.0, 10000.0){
        let target: Vec3 = rec.p + rec.normal + random_unit_sphere();
        return colour(&Ray::new(&rec.p, &(target - rec.p)), shapes) * 0.5;
    }
    else {
        let unit_direction: Vec3 = vec::make_unit_vector(&r.direction());
        let t: f32 = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) 
                + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn render() {
    const WIDTH: u32 = 400; 
    const HEIGHT: u32 = 200; 
    const SMOOTH: u32 = 100; 

    let mut framebuffer = image::ImageBuffer::new(WIDTH, HEIGHT);

    let shapes: Shapes = Shapes::new(vec![
        Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0), 
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)], 2);

    let cam: Camera = Camera::default();

    for i in 0..WIDTH{
        for j in 0..HEIGHT{
            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _k in 0..SMOOTH{
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let u: f32 = (i as f32 + rng.gen_range(0.0, 1.0)) / WIDTH as f32;
                let v: f32 = (j as f32 + rng.gen_range(0.0, 1.0)) / HEIGHT as f32;
                let r: Ray = cam.get_ray(u, v);
                col = colour(&r, &shapes) + col;
            }
            col = col / SMOOTH as f32;
            framebuffer[(i,j)] = image::Rgb([(col.x() * 255.0) as u8, (col.y() * 255.0) as u8, (col.z() * 255.0) as u8]);
        }
    }
    framebuffer.save("output.png").unwrap();
}

fn main() {
    render();
}