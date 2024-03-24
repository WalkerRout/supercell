
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lib_simulation::{World, CubeCell}; // external crate; pub fns only

mod bench_world {
  use super::*;

  pub fn update(c: &mut Criterion) {
    let mut group = c.benchmark_group("World::update");
    for size in [10u16, 20, 40, 50, 60, 70, 80, 100].iter() {
      group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
        let (mut prev, mut world) = World::<CubeCell>::new(size);
        b.iter(|| world.update(&mut prev))
      });
    }
    group.finish();
  }
}

criterion_group!(benches_world, 
  bench_world::update,
);
criterion_main!(benches_world);