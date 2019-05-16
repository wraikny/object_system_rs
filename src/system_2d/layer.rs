use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{Component, CoreSystem, HasComponent, Layer};

use super::object::Object2D;

// 擬似的なstruct
pub struct Layer2DCore;

impl CoreSystem for Layer2DCore {
    fn update(&mut self) {}
}

impl Drop for Layer2DCore {
    fn drop(&mut self) {}
}

pub struct Layer2D {
    core: Rc<RefCell<Layer2DCore>>,
    components: Vec<Rc<RefCell<Component<Layer2DCore>>>>,
    objects: Vec<Box<Object2D>>,
}

impl Layer2D {
    pub fn new() -> Layer2D {
        Layer2D {
            core: Rc::new(RefCell::new(Layer2DCore)),
            components: Vec::new(),
            objects: Vec::new(),
        }
    }
}

impl HasComponent<Layer2DCore> for Layer2D {
    fn core(&self) -> Rc<RefCell<Layer2DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Layer2DCore>>>> {
        &mut self.components
    }
}

impl Layer<Object2D> for Layer2D {
    fn objects(&mut self) -> &mut Vec<Box<Object2D>> {
        &mut self.objects
    }
}