use std::{cell::RefCell, rc::Rc};

use super::super::object_system::{Component, CoreSystem, Layer, ObjectSystem};

use super::object::{Object2D, Object2DCore};

pub struct Layer2DCore;
impl CoreSystem for Layer2DCore {
    fn update(&mut self) {}
}

pub struct Layer2D {
    core: Rc<RefCell<Layer2DCore>>,
    components: Vec<Rc<RefCell<Component<Layer2DCore>>>>,
    objects: Vec<Object2D>,
}

impl<TComp> ObjectSystem<Layer2DCore, TComp> for Layer2D
where
    TComp: Component<Layer2DCore> + 'static,
{
    fn core(&self) -> Rc<RefCell<Layer2DCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<Layer2DCore>>>> {
        &mut self.components
    }
}

impl<TComp, TObjComp> Layer<Layer2DCore, TComp, Object2DCore, TObjComp, Object2D> for Layer2D
where
    TComp: Component<Layer2DCore> + 'static,
    TObjComp: Component<Object2DCore> + 'static,
{
    fn objects(&mut self) -> &mut Vec<Object2D> {
        &mut self.objects
    }
}