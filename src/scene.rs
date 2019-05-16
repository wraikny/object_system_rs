use std::{cell::RefCell, rc::Rc};

use super::{
    object_system::{self, Component, CoreSystem, HasComponent},
    system_2d::{
        layer::{Layer2D},
        object::{Object2D},
    },
};

pub struct SceneCore;
impl CoreSystem for SceneCore {
    fn update(&mut self) {}
}

pub struct Scene {
    core: Rc<RefCell<SceneCore>>,
    components: Vec<Rc<RefCell<Component<SceneCore>>>>,
    layers2d: Vec<Box<Layer2D>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            core: Rc::new(RefCell::new(SceneCore)),
            components: Vec::new(),
            layers2d: Vec::new(),
        }
    }
}


impl<TComp> HasComponent<SceneCore, TComp> for Scene
where
    TComp: Component<SceneCore> + 'static,
{
    fn core(&self) -> Rc<RefCell<SceneCore>> {
        self.core.clone()
    }

    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<SceneCore>>>> {
        &mut self.components
    }
}

impl object_system::Scene<Layer2D, Object2D> for Scene {
    fn layers(&mut self) -> &mut Vec<Box<Layer2D>> {
        &mut self.layers2d
    }
}

