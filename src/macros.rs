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

/// Macro for defining a getter method for each given field, returning a slice
/// of all values of the corresponding entity attributes.
#[macro_export]
macro_rules! entity_set_getter_methods {
    ($id:ident, $($name:ident : $name_mut:ident : $type:ty,)*) => {
        #[inline(always)]
        pub fn size(&self) -> usize { self.base.size() }

        #[inline(always)]
        pub fn ids(&self) -> &[$id] { self.base.ids() }

        $(
            #[inline(always)]
            pub fn $name(&self) -> &[$type] { &self.$name }

            #[inline(always)]
            pub fn $name_mut(&mut self) -> &mut [$type] { &mut self.$name }
        )*
    };
}

/// Macro for defining a type with a vector for each given field, as well as
/// `new`, `despawn`, and getter methods.
#[macro_export]
macro_rules! entity_set {
    ($name:ident, $id:ident, $($field_name:ident : $field_name_mut:ident : $field_type:ty,)*) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $id(usize);

        impl From<usize> for $id {
            #[inline(always)]
            fn from(other: usize) -> Self { $id(other) }
        }

        #[derive(Debug)]
        pub struct $name {
            base: $crate::world::entity::collections::EntitySetBase<$id>,
            $($field_name: Vec<$field_type>,)*
        }

        impl $name {
            pub fn new() -> Self {
                entity_set_new_method_body!($name, $($field_name,)*)
            }

            pub fn despawn(&mut self, id: $id) {
                entity_set_despawn_method_body!(self, id, $($field_name,)*);
            }

            entity_set_getter_methods!($id, $($field_name: $field_name_mut: $field_type,)*);
        }
    };
}
