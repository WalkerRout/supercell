
use rand::Rng;
use raylib::prelude::*;

use lib_simulation as sm;
use sm::Cell;

use std::thread;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 800;
const Y_OFFSET: f32 = 3.0;
const UPDATE_TICKS: usize = 9;
const DIMENSIONS: u16 = 40;
const ALIVE_COLOR: Color = Color::new(0x7b, 0xa0, 0x56, 0xFF);

#[derive(Debug, Clone, PartialEq)]
struct World {
  sm_world: sm::World<sm::CubeCell>,
  sm_previous: sm::World<sm::CubeCell>,
  ticks: usize,
  update_ticks: usize,
  draw_wireframe: bool,
}

impl World {
  fn new((sm_previous, sm_world): (sm::World<sm::CubeCell>, sm::World<sm::CubeCell>)) -> Self {
    Self {
      sm_world,
      sm_previous,
      ticks: 0,
      update_ticks: UPDATE_TICKS,
      draw_wireframe: false,
    }
  }

  fn update(&mut self) {
    self.sm_world.update(&mut self.sm_previous);
  }

  fn draw(&mut self, d: &mut RaylibDrawHandle<'_>, camera: &Camera) {
    let y_offset = Y_OFFSET;
    let dimension = self.sm_world.rules.dims;
    let index_to_vector3 = |(i, j, k): (u16, u16, u16)| -> Vector3 {
      let off = dimension as f32 / 2.0;
      let i = i as f32 - off;
      let j = j as f32 + y_offset;
      let k = k as f32 - off;
      Vector3::new(i, j, k)
    };

    let mut rng = rand::thread_rng();
    let mut d2 = d.begin_mode3D(camera);
    for cell in &self.sm_world.cells {
      let pos = index_to_vector3(cell.index);
      let size = 1.0;
      let (st, hp) = cell.status();
      match st {
        sm::CellStatus::Alive => {
          let alive_color = {
            let Color { r, g, b, ..} = ALIVE_COLOR;
            let r = rng.gen_range(r-4..=r+4);
            let g = rng.gen_range(g-4..=g+4);
            let b = rng.gen_range(b-4..=b+4);
            Color::new(r, g, b, 0xFF)
          };
          d2.draw_cube(pos, size, size, size, alive_color);
        },
        sm::CellStatus::Decaying => {
          let decaying_color = {
            let intensity = (1.0 + hp.curr_health as f32) / (hp.max_health as f32 + 2.0);
            let brightness = (intensity * 255.0) as u8;
            Color::new(brightness, brightness, brightness, 0xFF)
          };
          d2.draw_cube(pos, size, size, size, decaying_color);
        },
        _ => (),
      }
    }

    // kind of unstable
    if self.draw_wireframe {
      let fdim = dimension as f32;
      let pos = Vector3::new(0.0, Y_OFFSET + fdim / 2.0, 0.0);
      d2.draw_cube_wires(pos, fdim, fdim, fdim, Color::MAROON);
    }

    d2.draw_plane(
      Vector3::new(0.0, -1.0, 0.0),
      Vector2::new(2.0*dimension as f32, 2.0*dimension as f32),
      Color::LIGHTGRAY,
    );
  }
}

fn init_world(rl: &mut RaylibHandle, dimension: u16) -> (World, Camera3D) {
  let camera = Camera3D::perspective(
    Vector3::new(1.7 * dimension as f32, dimension as f32, 1.7 * dimension as f32),
    Vector3::new(0.0, dimension as f32 / 4.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    70.0,
  );

  rl.set_camera_mode(camera, CameraMode::CAMERA_THIRD_PERSON);
  rl.set_target_fps(60);

  (World::new(sm::World::new(dimension)), camera)
}

fn input_world(world: &mut World, rl: &mut RaylibHandle, _camera: &Camera3D) {
  let dimension = world.sm_world.rules.dims;
  // reset
  if rl.is_key_pressed(KeyboardKey::KEY_R) {
    let prev_update_ticks = world.update_ticks;
    *world = World::new(sm::World::new(dimension));
    world.update_ticks = prev_update_ticks;
  }

  // wireframe modifier
  if rl.is_key_pressed(KeyboardKey::KEY_D) {
    world.draw_wireframe = !world.draw_wireframe;
  }

  // update ticks modifiers
  if rl.is_key_pressed(KeyboardKey::KEY_UP) && world.update_ticks <= 55 {
    world.update_ticks += 5;
  }
  if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
    if world.update_ticks.checked_sub(5).is_some() {
      world.update_ticks -= 5;
    } else {
      world.update_ticks = 1;
    }
    assert!(world.update_ticks >= 1); // use % update_ticks; cannot div by 0
  }
}

fn render_world(world: &mut World, rl: &mut RaylibHandle, thread: &RaylibThread, camera: &mut Camera3D) {
  rl.update_camera(camera);
  world.ticks += 1;

  // draw scope
  {
    let mut d = rl.begin_drawing(thread);
    // setup
    {
      d.clear_background(Color::WHITE);
    }
    // cells
    {
      if world.ticks % world.update_ticks == 0 {
        let mut updated_world = world.clone();
        let handle = thread::spawn(move || {
          updated_world.update();
          updated_world
        });
        world.draw(&mut d, camera);
        // thread shouldnt panic
        *world = handle.join().unwrap();
      } else {
        world.draw(&mut d, camera);
      }
    }
    // ui
    {
      d.draw_rectangle(10, 10, 120, 100, Color::SKYBLUE);
      d.draw_rectangle_lines(10, 10, 120, 100, Color::BLUE);
      d.draw_text("3D Game of Life", 20, 20, 12, Color::BLACK);
      d.draw_text(&format!("Ticks: {}", world.ticks), 20, 35, 12, Color::BLACK);
      d.draw_text(&format!("Update ticks: {}", world.update_ticks), 20, 50, 12, Color::BLACK);
    }
  }
}

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(WIDTH, HEIGHT)
    .title("3D Life")
    .build();

  let dimension = DIMENSIONS;
  let (mut world, mut camera) = init_world(&mut rl, dimension);
  
  while !rl.window_should_close() {
    input_world(&mut world, &mut rl, &camera);
    render_world(&mut world, &mut rl, &thread, &mut camera);    
  }
}
