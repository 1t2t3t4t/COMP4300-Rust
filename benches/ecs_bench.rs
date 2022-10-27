use criterion::{criterion_group, criterion_main, Criterion};
use ecs::entity::Entity;
use ecs::manager::EntityManager;
use rand::Rng;

struct A;
struct B;
struct C;

#[derive(Default, Debug)]
struct ComponentA(i32);

#[derive(Default, Debug)]
struct ComponentB(f64);

#[derive(Default, Debug)]
struct ComponentC(u32);

#[derive(Debug)]
struct MyComponent(String);

fn random_assign_component(entity: &mut Entity) {
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

fn setup_manager<const N: usize>() -> EntityManager {
    let mut manager = EntityManager::new();
    for _ in 0..N {
        let mut entity = manager.add_tag(A);
        random_assign_component(&mut entity);
    }
    manager
        .add_tag(C)
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

    c.bench_function("query from 10,000 entities", |b| {
        let mut manager = setup_manager::<10_000>();
        manager.update();

        b.iter(|| {
            let my_comp = manager.query_entities_component::<MyComponent>();
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
            let my_comp = manager.query_entities_components::<(MyComponent, ComponentC, C)>();
            assert_eq!(my_comp.len(), 1);
        });
    });

    c.bench_function(
        "query multiple comps mut with tag from 10,000 entities",
        |b| {
            let mut manager = setup_manager::<10_000>();
            manager.update();

            b.iter(|| {
                let my_comp = manager.query_entities_component_tag_mut::<MyComponent, C>();
                assert_eq!(my_comp.len(), 1);
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
