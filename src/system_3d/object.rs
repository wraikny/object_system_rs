use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{Component, CoreSystem, HasComponent, Object};

// 擬似的なstruct
pub struct Object3DCore;

impl Object3DCore {
    crate fn new() -> Self {
        Object3DCore
    }
}

impl Drop for Object3DCore {
    fn drop(&mut self) {}
}

impl CoreSystem for Object3DCore {
    fn update(&mut self) {}
}

pub struct Object3D {
    core: Rc<RefCell<Object3DCore>>,
    components: Vec<Rc<RefCell<Component<Object3DCore>>>>,
}

impl Object3D {
    pub fn new() -> Object3D {
        Object3D {
            core: Rc::new(RefCell::new(Object3DCore::new())),
            components: Vec::new(),
        }
    }
}

impl HasComponent<Object3DCore> for Object3D {
    fn core(&self) -> Rc<RefCell<Object3DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Object3DCore>>>> {
        &mut self.components
    }
}

impl Object for Object3D {}
