use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{
    CoreSystem,
    ObjectSystem,
    Component,
    Object,
};

pub struct Object2DCore;
impl CoreSystem for Object2DCore {
    fn update(&mut self) {}
}

pub struct Object2D {
    core : Rc<RefCell<Object2DCore>>,
    components : Vec<Rc<RefCell<Component<Object2DCore>>>>,
}

impl<TComp> ObjectSystem<Object2DCore, TComp> for Object2D
    where TComp : Component<Object2DCore> + 'static
{
    fn core(&self) -> Rc<RefCell<Object2DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Object2DCore>>>> {
        &mut self.components
    }
}

impl<TComp> Object<Object2DCore, TComp> for Object2D
    where TComp : Component<Object2DCore> + 'static
{}