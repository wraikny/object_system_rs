use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{Component, CoreSystem, HasComponent, Object};

pub struct Object2DCore;
impl CoreSystem for Object2DCore {
    fn update(&mut self) {}
}

pub struct Object2D {
    core: Rc<RefCell<Object2DCore>>,
    components: Vec<Rc<RefCell<Component<Object2DCore>>>>,
}

impl Object2D {
    pub fn new() -> Object2D {
        Object2D {
            core: Rc::new(RefCell::new(Object2DCore)),
            components: Vec::new(),
        }
    }
}

impl HasComponent<Object2DCore> for Object2D {
    fn core(&self) -> Rc<RefCell<Object2DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Object2DCore>>>> {
        &mut self.components
    }
}

impl Object for Object2D {}
