use rand::Rng;
use rtw::camera::Camera;
use rtw::color::paint;
use rtw::hittable_list::HittableList;
use rtw::material::{Dielectric, Lambertian, Metal};
use rtw::perlin::Perlin;
use rtw::sphere::Sphere;
use rtw::texture::{CheckerTexture, NoiseTexture};
use rtw::vec3::*;

use std::rc::Rc;
use std::{
    fs::{self},
    sync::mpsc,
    thread,
};
const THREAD_NUM: usize = 12;

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: usize = 2560;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const TOT_SIZE: usize = 12 * IMAGE_HEIGHT * IMAGE_WIDTH + 20;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: usize = 50;
    const HPTHREAD: usize = IMAGE_HEIGHT / THREAD_NUM + 1;

    let mut image: String = String::with_capacity(TOT_SIZE);
    image.push_str(&format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n"));
    let mut handles = vec![];
    let mut buffer = vec![];

    let (tx, rx) = mpsc::channel();
    let perlin = Perlin::new();
    for t in (0..THREAD_NUM).rev() {
        let tx = tx.clone();
        let perlin = perlin.clone();

        handles.push(thread::spawn(move || {
            // camera
            let lookfrom = Point3::new(13f64, 3f64, 6f64);
            let lookat = Point3::new(0f64, 0f64, -1f64);
            let mut camera = Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0f64, 1f64, 0f64),
                20f64,
                ASPECT_RATIO,
                0.01f64,
                (lookfrom - lookat).modulus(),
                [0f64, 1f64],
            );

            // world
            let mut world = scene();
            world.push(Rc::new(Sphere::new(
                Point3::new(1.3, 0.5, 2.6),
                0.5,
                Rc::new(Lambertian::from_texture(Rc::new(NoiseTexture::new(
                    perlin, 4f64,
                )))),
            )));

            let mut buffer = String::with_capacity(TOT_SIZE / THREAD_NUM);
            for i in (t * HPTHREAD..((t + 1) * HPTHREAD).min(IMAGE_HEIGHT)).rev() {
                for j in 0..IMAGE_WIDTH {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u =
                            (j as f64 + world.rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                        let v =
                            (i as f64 + world.rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                        let r = camera.get_ray(u, v);
                        pixel_color += world.ray_color(r, MAX_DEPTH);
                    }
                    buffer.push_str(&paint(pixel_color, SAMPLES_PER_PIXEL));
                }
            }
            tx.send((t, buffer)).unwrap();
        }));
    }

    for _ in 0..THREAD_NUM {
        let product = rx.recv().unwrap();
        println!("Thread {} completed", product.0);
        buffer.push(product);
    }
    buffer.sort_by_key(|x| x.0);
    for (_, st) in buffer.iter().rev() {
        image.push_str(st);
    }

    fs::write("rtw.ppm", &image)?;
    print!("\nDone.\n");
    Ok(())
}

fn scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Rc::new(Lambertian::from_texture(checker));
    world.push(Rc::new(Sphere::new(
        Point3::new(0f64, -1000f64, 0f64),
        1000f64,
        ground_material.clone(),
    )));

    let material = Rc::new(Dielectric::new(1.5));
    world.push(Rc::new(Sphere::new(
        Point3::new(0f64, 0.8f64, 0f64),
        0.8f64,
        material.clone(),
    )));
    let material = Rc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.push(Rc::new(Sphere::new(
        Point3::new(-4f64, 1f64, 0f64),
        1f64,
        material.clone(),
    )));
    let material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0f64));
    world.push(Rc::new(Sphere::new(
        Point3::new(4f64, 1f64, 0f64),
        1f64,
        material.clone(),
    )));
    let material = Rc::new(Metal::new(Color::new(1f64, 1f64, 10f64 / 255f64), 0.03));
    world.push(Rc::new(Sphere::new(
        Point3::new(2f64, 0.5, 1.5),
        0.5,
        material.clone(),
    )));
    world
}
