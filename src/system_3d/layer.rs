use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{Component, CoreSystem, HasComponent, Layer};

use super::object::Object3D;

// 擬似的なstruct
pub struct Layer3DCore;

impl CoreSystem for Layer3DCore {
    fn update(&mut self) {}
}

impl Drop for Layer3DCore {
    fn drop(&mut self) {}
}

pub struct Layer3D {
    core: Rc<RefCell<Layer3DCore>>,
    components: Vec<Rc<RefCell<Component<Layer3DCore>>>>,
    objects: Vec<Box<Object3D>>,
}

impl Layer3D {
    pub fn new() -> Layer3D {
        Layer3D {
            core: Rc::new(RefCell::new(Layer3DCore)),
            components: Vec::new(),
            objects: Vec::new(),
        }
    }
}

impl HasComponent<Layer3DCore> for Layer3D {
    fn core(&self) -> Rc<RefCell<Layer3DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Layer3DCore>>>> {
        &mut self.components
    }
}

impl Layer<Object3D> for Layer3D {
    fn objects(&mut self) -> &mut Vec<Box<Object3D>> {
        &mut self.objects
    }
}