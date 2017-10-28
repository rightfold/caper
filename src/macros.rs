/// Macro for defining a despawn method that calls `EntitySetBase::new` and
/// `Vec::new` appropriately.
#[macro_export]
macro_rules! entity_set_new_method_body {
    ($Self:ident, $($name:ident,)*) => {
        $Self{
            base: $crate::world::entity::collections::EntitySetBase::new(),
            $($name: Vec::new(),)*
        }
    };
}

/// Macro for defining a spawn method that calls `EntitySetBase::spawn` and
/// `Vec::push` appropriately.
#[macro_export]
macro_rules! entity_set_spawn_method_body {
    ($self:expr, $($name:ident : $value:expr,)*) => {
        {
            let id = $self.base.prepare_spawn();
            $($self.$name.push($value);)*
            id
        }
    };
}

/// Macro for defining a despawn method that calls `EntitySetBase::despawn` and
/// `Vec::swap_remove` appropriately.
#[macro_export]
macro_rules! entity_set_despawn_method_body {
    ($self:expr, $id:expr, $($name:ident,)*) => {
        {
            let index = $self.base.prepare_despawn($id);
            $($self.$name.swap_remove(index);)*
        }
    };
}

/// Macro for defining a type with a vector for each given field, as well as
/// `new`, `despawn`, and getter methods.
#[macro_export]
macro_rules! entity_set {
    ($name:ident, $id:ident, $($field_name:ident : $field_type:ty,)*) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $id(usize);

        impl From<usize> for $id {
            #[inline(always)]
            fn from(other: usize) -> Self { $id(other) }
        }

        #[derive(Debug)]
        pub struct $name {
            base: $crate::world::entity::collections::EntitySetBase<$id>,
            $(#[doc(hidden)] pub $field_name: Vec<$field_type>,)*
        }

        impl $name {
            pub fn new() -> Self {
                entity_set_new_method_body!($name, $($field_name,)*)
            }

            pub fn despawn(&mut self, id: $id) {
                entity_set_despawn_method_body!(self, id, $($field_name,)*);
            }

            #[inline(always)]
            pub fn size(&self) -> usize {
                self.base.size()
            }

            #[inline(always)]
            pub fn ids(&self) -> &[$id] {
                self.base.ids()
            }
        }
    };
}

#[macro_export]
macro_rules! entity_field {
    ($self:expr, $field:ident) => {
        &$self.$field[..]
    };
    ($self:expr, mut $field:ident) => {
        &mut $self.$field[..]
    };
}

#[macro_export]
macro_rules! with_each_entity_set {
    ($macro:ident) => {
        $macro!(
            spider    : spiders    : SpiderSet,
            gas_spore : gas_spores : GasSporeSet,
        );
    };
}

#[macro_export]
macro_rules! simulate_deaths {
    ($set:expr) => {{
        let set = $set;
        let dead_ids = {
            let ids = set.ids().iter();
            let healths = entity_field!(set, healths).iter();
            ids.zip(healths)
                .filter(|&(_, &health)| health < 0.0)
                .map(|(&id, _)| id)
                .collect::<Vec<_>>()
        };
        for id in dead_ids {
            set.despawn(id);
        }
    }};
}
