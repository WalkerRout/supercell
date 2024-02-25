
use raylib::prelude::*;

use lib_simulation as sm;
use sm::Cell;

use std::thread;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;
const Y_OFFSET: f32 = 3.0;
const UPDATE_TICKS: usize = 5;
const DIMENSIONS: u16 = 42;
const ALIVE_COLOR: Color = Color::DARKGRAY;
const DECAYING_COLOR: Color = Color::new(0x8c, 0x40, 0x40, 0xFF);

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

    let mut d2 = d.begin_mode3D(camera);
    for cell in &self.sm_world.cells {
      let pos = index_to_vector3(cell.index);
      let size = 1.0;
      match cell.status() {
        sm::CellStatus::Alive => d2.draw_cube(pos, size, size, size, ALIVE_COLOR),
        sm::CellStatus::Decaying => d2.draw_cube(pos, size, size, size, DECAYING_COLOR),
        _ => (),
      }
      // kind of unstable
      if self.draw_wireframe {
        d2.draw_cube_wires(pos, size, size, size, Color::MAROON);
      }
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
  if rl.is_key_pressed(KeyboardKey::KEY_DOWN) && world.update_ticks - 5 >= 1 {
    world.update_ticks -= 5;
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
        *world = handle.join().expect("updated_world.update() should not panic");
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
