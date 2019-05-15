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
    core : Object2DCore,
    components : Vec<Box<Component<Object2DCore>>>,
}

impl Object2D {
    pub fn add_component<C>(&mut self, component : C)
        where C : Component<Object2DCore> + 'static
    {
        self.components.push(Box::new(component));
    }
}

impl<TComp> ObjectSystem<Object2DCore, TComp> for Object2D
    where TComp : Component<Object2DCore> + 'static
{
    fn core(&mut self) -> &mut Object2DCore {
        &mut self.core
    }

    fn components(&mut self) -> &mut Vec<Box<Component<Object2DCore>>> {
        &mut self.components
    }

    fn update_components(&mut self) {
        for c in &mut self.components {
            c.on_update(&mut self.core);
        }
    }
}

impl<TComp> Object<Object2DCore, TComp> for Object2D
    where TComp : Component<Object2DCore> + 'static
{

}