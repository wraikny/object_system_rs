use std::{cell::RefCell, rc::Rc};

use super::{
    object_system::{self, Component, CoreSystem, HasComponent, HasComponentInner, SceneInner},
    system_2d::{layer::Layer2D, object::Object2D},
    system_3d::{layer::Layer3D, object::Object3D},
};

// 擬似的なstruct
pub struct SceneCore;

impl CoreSystem for SceneCore {
    fn update(&mut self) {}
}

impl Drop for SceneCore {
    fn drop(&mut self) {}
}

pub struct Scene {
    core: Rc<RefCell<SceneCore>>,
    components: Vec<Rc<RefCell<Component<SceneCore>>>>,
    layers2d: Vec<Box<Layer2D>>,
    layers3d: Vec<Box<Layer3D>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            core: Rc::new(RefCell::new(SceneCore)),
            components: Vec::new(),
            layers2d: Vec::new(),
            layers3d: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.update_core();
        self.update_components();
        (self as &mut SceneInner<_, _, Layer2D, _, Object2D>).update_layers();
        (self as &mut SceneInner<_, _, Layer3D, _, Object3D>).update_layers();
    }
}


impl HasComponent<SceneCore> for Scene {
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

impl object_system::Scene<Layer3D, Object3D> for Scene {
    fn layers(&mut self) -> &mut Vec<Box<Layer3D>> {
        &mut self.layers3d
    }
}