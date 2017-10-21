use std::collections::HashMap;
use std::hash::Hash;

/// Keeps track of entity identifiers and the indices of the corresponding
/// entities in the vectors in the entity set.
///
/// Used by the `entity_set!` macro.
#[derive(Debug)]
pub struct EntitySetBase<Id> where Id: Eq + Hash {
    next_id: usize,
    indices_by_id: HashMap<Id, usize>,
    ids_by_index: Vec<Id>,
}

impl<Id> EntitySetBase<Id> where Id: Eq + Hash {
    /// After calling this method, the vectors containing entity attributes
    /// should be empty.
    pub fn new() -> Self {
        EntitySetBase{
            next_id: 0,
            indices_by_id: HashMap::new(),
            ids_by_index: Vec::new(),
        }
    }
}

impl<Id> EntitySetBase<Id> where Id: Copy + Eq + From<usize> + Hash {
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.ids_by_index.len()
    }

    #[inline(always)]
    pub fn ids(&self) -> &[Id] {
        &self.ids_by_index
    }

    /// After calling this method, `Vec::push` should be called on every vector
    /// containing entity attributes.
    pub fn prepare_spawn(&mut self) -> Id {
        let id = Id::from(self.next_id);
        self.next_id += 1;

        let index = self.ids_by_index.len();
        self.indices_by_id.insert(id, index);
        self.ids_by_index.push(id);

        id
    }

    /// After calling this method, `Vec::swap_remove` should be called on every
    /// vector containing entity attributes, using the index returned by this
    /// method.
    pub fn prepare_despawn(&mut self, id: Id) -> usize {
        let &index = self.indices_by_id.get(&id).unwrap();
        let &last_id = self.ids_by_index.last().unwrap();
        self.ids_by_index.swap_remove(index);
        self.indices_by_id.insert(last_id, index);
        index
    }
}
