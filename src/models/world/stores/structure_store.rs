use std::collections::HashMap;
use crate::models::world::dimension::Dimension;
use crate::models::world_structures::generic_structure::GenericParentStructure;

#[derive(Debug)]
pub struct StructureStoreReference {
    pub chunk_ref: i64,
    pub structure_id: String
}

pub struct StructureStore {
    structures: HashMap<i64, Vec<GenericParentStructure>>,
}

impl StructureStore {
    pub fn new() -> StructureStore {
        StructureStore {
            structures: HashMap::new(),
        }
    }

    pub fn structures(&self) -> Vec<&GenericParentStructure> {
        self.structures.values().flatten().collect()
    }

    pub fn add_structure(&mut self, structure: GenericParentStructure) {
        let chunk_ref = structure.chunk_position().to_chunk_ref();
        self.structures.entry(chunk_ref).or_insert_with(Vec::new).push(structure);
    }

    pub fn add_structures(&mut self, structures: HashMap<i64, Vec<GenericParentStructure>>) {
        self.structures.extend(structures.into_iter());
    }

    pub fn get_structures_by_chunk_reference(&self, chunk_reference: i64) -> &[GenericParentStructure] {
        self.structures.get(&chunk_reference).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn get_structure_by_store_reference(&self, reference: &StructureStoreReference) -> Option<&GenericParentStructure> {
        if let Some(vec) = self.structures.get(&reference.chunk_ref) {
            for st in vec {
                if st.id() == &reference.structure_id {
                    return Some(st);
                }
            }
        }
        None
    }

    pub fn get_structures_by_id(&self, id: &str) -> Vec<&GenericParentStructure> {
        let mut structures = Vec::new();
        for refed_strcut in self.structures.values() {
            for structure in refed_strcut.iter() {
                if structure.id() == id {
                    structures.push(structure);
                }
            }
        }
        structures
    }
}

impl StructureStoreReference {
    pub fn new(chunk_ref: i64, structure_id: String) -> StructureStoreReference {
        StructureStoreReference {
            chunk_ref,
            structure_id
        }
    }

    pub fn get<'a>(&self, dimension: &'a Dimension) -> Option<&'a GenericParentStructure> {
        dimension.structure_store().get_structure_by_store_reference(self)
    }
}