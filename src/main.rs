use term_size;
use std::ops::Add;
use std::f64::consts::PI;

struct Point {
  x: f64,
  y: f64,
  z: f64
}

impl Point {
  fn at(x: f64, y: f64, z: f64) -> Point {
    Point {x: x, y: y, z: z}
  }
  fn dot(self, other: Point) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Add for &Point {
  type Output = Point;
  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}

struct Sphere {
  radius: f64,
  pos: Point,
}

impl Sphere {
  fn light(self, canvas: &mut Canvas) -> () {
    for i in 0..300 {
      for j in 0..300 {
        let phi = PI / 300. * i as f64;
        let theta = 2. * PI / 300. * j as f64;
        let norm = Point{
          x: self.radius * phi.sin() * theta.cos(),
          y: self.radius * phi.sin() * theta.sin(),
          z: self.radius * phi.cos()
        };
        let p = &self.pos + &norm;
        let dz = p.z - canvas.site.z;
        let x = p.x / dz * canvas.scale;
        let y = p.y / dz * canvas.scale;
        let cx = x.round() as i32 + (canvas.width / 2) as i32;
        let cy = (y / 2.).round() as i32 + (canvas.height / 2) as i32;
        if cx >= 0 && cx < canvas.width as i32
            && cy >= 0 && cy < canvas.height as i32
            && p.z < canvas.pixels[cy as usize][cx as usize].z
        {
          let light = norm.dot(Point::at(-1., -1., -1.)) * canvas.light;
          canvas.pixels[cy as usize][cx as usize] = Pixel{ light: light, z: p.z };
        }
      }
    }
  }
}

#[derive(Copy, Clone)]
struct Pixel {
  light: f64,
  z: f64
}

impl Pixel {
  fn to_char(&self) -> char {
    let lights = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".chars().rev().collect::<String>();
    let l = ((self.light * lights.len() as f64).max(0.).round() as usize).min(lights.len() - 1);
    lights.as_bytes()[l] as char
  }
}

struct Canvas {
  site: Point,
  scale: f64,
  light: f64,
  width: usize,
  height: usize,
  pixels: Vec<Vec<Pixel>>
}

impl Canvas {
  fn with_site_from(at: Point) -> Canvas {
    let (w, h) = term_size::dimensions().unwrap();
    Canvas{
      site: at,
      scale: 150.,
      light: 2.,
      width: w,
      height: h,
      pixels: vec![
        vec![Pixel{ light: 0., z: f64::MAX }; w]; h
      ]
    }
  }
  fn show(self) -> () {
    println!("Hello, World!");
    for row in self.pixels {
      for p in row {
        print!("{}", p.to_char());
      }
      println!("")
    }
  }
}

fn main() {
    let earth = Sphere {radius: 1., pos: Point::at(0., 0., 0.) };
    let mut canvas = Canvas::with_site_from(Point::at(0., 0., -5.));
    earth.light(&mut canvas);
    canvas.show();
}
