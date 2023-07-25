use rand::Rng;
use rtw::aarect::{XYRect, YZRect, ZXRect};
use rtw::camera::Camera;
use rtw::color::paint;
use rtw::cube::Cube;
use rtw::hittable::{RotateY, Translate};
use rtw::hittable_list::HittableList;
use rtw::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use rtw::sphere::Sphere;
use rtw::texture::{CheckerTexture, ImageTexture};
use rtw::vec3::*;

use std::path::Path;
use std::rc::Rc;
use std::{
    fs::{self},
    sync::mpsc,
    thread,
};
const THREAD_NUM: usize = 16;

fn main() -> std::io::Result<()> {
    const IMAGE_WIDTH: usize = 920;
    const ASPECT_RATIO: f64 = 1f64;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const TOT_SIZE: usize = 12 * IMAGE_HEIGHT * IMAGE_WIDTH + 20;
    const SAMPLES_PER_PIXEL: usize = 10000;
    const MAX_DEPTH: usize = 50;
    const HPTHREAD: usize = IMAGE_HEIGHT / THREAD_NUM + 1;

    let background = Color::default();
    let mut image: String = String::with_capacity(TOT_SIZE);
    image.push_str(&format!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n"));
    let mut handles = vec![];
    let mut buffer = vec![];

    let (tx, rx) = mpsc::channel();
    for t in (0..THREAD_NUM).rev() {
        let tx = tx.clone();

        handles.push(thread::spawn(move || {
            // camera
            let lookfrom = Point3::new(278f64, 278f64, -800f64);
            let lookat = Point3::new(278f64, 278f64, 0f64);
            let mut camera = Camera::new(
                lookfrom,
                lookat,
                Vec3::new(0f64, 1f64, 0f64),
                40f64,
                ASPECT_RATIO,
                0.0f64,
                (lookfrom - lookat).modulus(),
                [0f64, 1f64],
            );

            // world
            let mut world = scene(2);

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
                        pixel_color += world.ray_color(r, &background, MAX_DEPTH);
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

fn scene(case: u8) -> HittableList {
    match case {
        1 => {
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
            let material = Rc::new(Lambertian::from_texture(Rc::new(ImageTexture::new(
                &Path::new("./assets/earthmap.jpg"),
            ))));
            world.push(Rc::new(Sphere::new(
                Point3::new(0f64, 2f64, 0f64),
                2f64,
                material,
            )));
            let difflight = Rc::new(DiffuseLight::from_color(Color::new(4f64, 4f64, 4f64)));
            world.push(Rc::new(XYRect::new(
                difflight,
                [3f64, 5f64],
                [1f64, 3f64],
                -2f64,
            )));
            world
        }
        2 => {
            let mut world = HittableList::new();
            let red = Rc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
            let white = Rc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
            let green = Rc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
            let light = Rc::new(DiffuseLight::from_color(Color::new(15f64, 15f64, 15f64)));

            world.push(Rc::new(YZRect::new(
                green.clone(),
                [0f64, 555f64],
                [0f64, 555f64],
                555f64,
            )));
            world.push(Rc::new(YZRect::new(
                red.clone(),
                [0f64, 555f64],
                [0f64, 555f64],
                0f64,
            )));
            world.push(Rc::new(ZXRect::new(
                light,
                [213f64, 343f64],
                [227f64, 332f64],
                554f64,
            )));
            world.push(Rc::new(ZXRect::new(
                white.clone(),
                [0f64, 555f64],
                [0f64, 555f64],
                0f64,
            )));
            world.push(Rc::new(ZXRect::new(
                white.clone(),
                [0f64, 555f64],
                [0f64, 555f64],
                555f64,
            )));
            world.push(Rc::new(XYRect::new(
                white.clone(),
                [0f64, 555f64],
                [0f64, 555f64],
                555f64,
            )));

            let box1 = Rc::new(Cube::new(
                Point3::new(0f64, 0f64, 0f64),
                Point3::new(165f64, 330f64, 165f64),
                white.clone(),
            ));
            let box1 = Rc::new(RotateY::new(15f64, box1));
            let box1 = Rc::new(Translate::new(Vec3::new(265f64, 0f64, 295f64), box1));
            world.push(box1);

            let box2 = Rc::new(Cube::new(
                Point3::new(0f64, 0f64, 0f64),
                Point3::new(165f64, 165f64, 165f64),
                white.clone(),
            ));
            let box2 = Rc::new(RotateY::new(-18f64, box2));
            let box2 = Rc::new(Translate::new(Vec3::new(130f64, 0f64, 65f64), box2));
            world.push(box2);

            world
        }
        _ => {
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
                Point3::new(0f64, 1f64, 0f64),
                1f64,
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
            let material = Rc::new(Lambertian::from_texture(Rc::new(ImageTexture::new(
                &Path::new("./assets/earthmap.jpg"),
            ))));
            world.push(Rc::new(Sphere::new(
                Point3::new(3.5f64, 0.7, 2.1),
                0.7,
                material.clone(),
            )));
            world
        }
    }
}
