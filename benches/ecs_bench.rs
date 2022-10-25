use criterion::{criterion_group, criterion_main, Criterion};
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum MyTag {
    A,
    B,
    C,
}

impl MyTag {
    fn rand() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => MyTag::A,
            1 => MyTag::B,
            2 => MyTag::C,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug)]
struct ComponentA(i32);

#[derive(Default, Debug)]
struct ComponentB(f64);

#[derive(Default, Debug)]
struct ComponentC(u32);

#[derive(Debug)]
struct MyComponent(String);

fn random_assign_component(entity: &mut Entity<MyTag>) {
    let mut thread_rand = rand::thread_rng();
    if thread_rand.gen_bool(0.3) {
        entity.add_component(ComponentA::default());
    }

    if thread_rand.gen_bool(0.4) {
        entity.add_component(ComponentB::default());
    }

    if thread_rand.gen_bool(0.5) {
        entity.add_component(ComponentC::default());
    }
}

fn setup_manager<const N: usize>() -> EntityManager<MyTag> {
    let mut manager = EntityManager::new();
    for _ in 0..N {
        let mut entity = manager.add_tag(MyTag::rand());
        random_assign_component(&mut entity);
    }
    manager
        .add_tag(MyTag::C)
        .add_component(MyComponent("Boss".to_string()))
        .add_component(ComponentC::default());

    manager
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("update 1,000 entities", |b| {
        b.iter(|| {
            let mut manager = setup_manager::<1_000>();
            manager.update();
        });
    });

    c.bench_function("query mut from 10,000 entities", |b| {
        let mut manager = setup_manager::<10_000>();
        manager.update();

        b.iter(|| {
            let my_comp = manager.query_entities_component_mut::<MyComponent>();
            assert_eq!(my_comp.len(), 1);
        });
    });

    c.bench_function("query multiple comps from 10,000 entities", |b| {
        let mut manager = setup_manager::<10_000>();
        manager.update();

        b.iter(|| {
            let my_comp = manager.query_entities_components::<(MyComponent, ComponentC)>();
            assert_eq!(my_comp.len(), 1);
        });
    });

    c.bench_function("query multiple comps with tag from 10,000 entities", |b| {
        let mut manager = setup_manager::<10_000>();
        manager.update();

        b.iter(|| {
            let my_comp =
                manager.query_entities_components_tag::<(MyComponent, ComponentC)>(MyTag::C);
            assert_eq!(my_comp.len(), 1);
        });
    });

    c.bench_function(
        "query multiple comps mut with tag from 10,000 entities",
        |b| {
            let mut manager = setup_manager::<10_000>();
            manager.update();

            b.iter(|| {
                let my_comp = manager.query_entities_component_tag_mut::<MyComponent>(MyTag::C);
                assert_eq!(my_comp.len(), 1);
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
