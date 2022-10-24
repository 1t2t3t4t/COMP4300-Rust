use crate::entity::Entity;
use std::any::TypeId;
use std::collections::HashSet;

pub trait TypesQueryable<'e> {
    type QueryResult;

    fn get_types() -> HashSet<TypeId>;
    fn query<T>(entity: &'e Entity<T>) -> Option<Self::QueryResult>;
}

macro_rules! types_queryable_tuple {
    ($a:tt) => {};
    ($a:tt, $($b:tt),+) => {
        impl<'e, $a, $($b), +> TypesQueryable<'e> for ($a, $($b), +) where $a: std::any::Any, $($b : std::any::Any),+ {
            type QueryResult = (&'e $a, $(&'e $b),+);

            fn get_types() -> HashSet<std::any::TypeId> {
                HashSet::from([
                    std::any::TypeId::of::<$a>(),
                    $(std::any::TypeId::of::<$b>()),+
                ])
            }

            fn query<T>(entity: &'e Entity<T>) -> Option<Self::QueryResult> {
                if !entity.has_components::<Self>() {
                    None
                } else {
                    Some((
                        entity.get_component::<$a>().unwrap(),
                        $(entity.get_component::<$b>().unwrap()),+
                    ))
                }
            }
        }
        types_queryable_tuple!($($b),+);
    }
}

// Auto implement tuple query typeid getter
types_queryable_tuple!(A, B, C, D, E, F, G, H, I, J, K);
