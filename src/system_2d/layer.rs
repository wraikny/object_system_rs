use super::super::object_system::{
    Component,
    CoreSystem,
    ObjectSystem,
    Layer,
};

use super::object::{Object2DCore, Object2D};

pub struct Layer2DCore;
impl CoreSystem for Layer2DCore {
    fn update(&mut self) {}
}

pub struct Layer2D {
    core : Layer2DCore,
    components : Vec<Box<Component<Layer2DCore>>>,
    objects : Vec<Object2D>,
}

impl<TComp> ObjectSystem<Layer2DCore, TComp> for Layer2D
    where TComp : Component<Layer2DCore> + 'static
{
    fn core(&mut self) -> &mut Layer2DCore {
        &mut self.core
    }

    fn components(&mut self) -> &mut Vec<Box<Component<Layer2DCore>>> {
        &mut self.components
    }

    fn update_components(&mut self) {
        for c in &mut self.components {
            c.on_update(&mut self.core);
        }
    }
}

impl<TComp, TObjComp> Layer<Layer2DCore, TComp, Object2DCore, TObjComp, Object2D> for Layer2D
    where
        TComp : Component<Layer2DCore> + 'static,
        TObjComp : Component<Object2DCore> + 'static
{
    fn objects(&mut self) -> &mut Vec<Object2D> {
        &mut self.objects
    }
}