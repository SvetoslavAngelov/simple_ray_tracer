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

fn colour(r: &Ray, shapes: &Shapes, depth: i32) -> Vec3 {
    use shape::Hitable;
    use material::Material;

    let mut rec: Record = Record::default();

    if shapes.hit(&mut rec, r, 0.0, 10000.0){
        let mut scattered: Ray = Default::default();
        let mut attenuation: Vec3 = Default::default(); 
        let diffuse: material::Diffuse = Default::default();
        let specular: material::Specular = Default::default();

        if rec.material == shape::MaterialType::Diffuse && depth < 50 {
            diffuse.scatter(&rec, &r, &mut scattered, &mut attenuation);
            return Vec3::new(0.5, 0.5, 0.5) * colour(&scattered, shapes, depth + 1);
        }
        else if rec.material == shape::MaterialType::Specular && depth < 50{
            specular.scatter(&rec, &r, &mut scattered, &mut attenuation);
            return Vec3::new(0.8, 0.0, 0.8) * colour(&scattered, shapes, depth + 1);
        }
        else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
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
        Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, shape::MaterialType::Diffuse), 
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, shape::MaterialType::Specular),
        Sphere::new(Vec3::new(0.5, 0.0, -1.0), 0.2, shape::MaterialType::Diffuse)], 3);

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
                col = colour(&r, &shapes, 0) + col;
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