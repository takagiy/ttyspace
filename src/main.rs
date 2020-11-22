use term_size;
use std::ops::Add;
use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;

struct Point {
  x: f64,
  y: f64,
  z: f64
}

impl Point {
  fn at(x: f64, y: f64, z: f64) -> Point {
    Point {x: x, y: y, z: z}
  }

  fn polar_at(r: f64, phi: f64, theta: f64) -> Point {
    Point{
      x: r * phi.sin() * theta.cos(),
      y: r * phi.sin() * theta.sin(),
      z: r * phi.cos()
    }
  }

  fn rot_x(self, theta: f64) -> Point {
    Point {
      x: self.x,
      y: self.y * theta.cos() - self.z * theta.sin(),
      z: self.y * theta.sin() + self.z * theta.cos()
    }
  }

  fn rot_y(self, theta: f64) -> Point {
    Point {
      x: self.x * theta.cos() + self.z * theta.sin(),
      y: self.y,
      z: -self.x * theta.sin() + self.z * theta.cos()
    }
  }

  fn rot_z(self, theta: f64) -> Point {
    Point {
      x: self.x * theta.cos() - self.y * theta.sin(),
      y: self.x * theta.sin() + self.y * theta.cos(),
      z: self.z
    }
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
    for i in 0..150 {
      for j in 0..150 {
        let phi = PI / 150. * i as f64;
        let theta = 2. * PI / 150. * j as f64;
        let norm = Point::polar_at(self.radius, phi, theta);
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
  fn show(self) -> String {
    let mut buffer = String::with_capacity((self.width + 1) * self.height);
    for row in self.pixels {
      for p in row {
        buffer.push(p.to_char());
      }
      buffer.push('\n');
    }
    buffer
  }
}

fn main() {
  let frames = {
    let n = 120;
    let mut frames = Vec::with_capacity(n);
    for i in 0..n {
      let rot = 2. * PI / n as f64 * i as f64;
      frames.push(draw_rotated_donut(rot));
    }
    frames
  };
  animate(frames);
}

fn draw_rotated_donut(rot: f64) -> String {
  let mut canvas = Canvas::with_site_from(Point::at(0., 0., -5.));
  for i in 0..8 {
    let theta = 2. * PI / 8. * i as f64;
    let p = Point::polar_at(1.5, theta, 0.).rot_z(1.).rot_y(rot);
    let s = Sphere {radius: 0.8, pos: p };
    s.light(&mut canvas);
  }
  canvas.show()
}

fn animate(frames: Vec<String>) {
  let rest = Duration::from_millis(16);
  loop {
    for f in &frames {
      println!("{}", f);
      sleep(rest);
    }
  }
}
