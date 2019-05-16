use std::{cell::RefCell, rc::Rc};

use super::super::{
    object_system::{Component, HasComponent, Layer, Scene},
    scene::{self, SceneCore},
    system_2d::{
        layer::{Layer2D, Layer2DCore},
        object::{Object2D, Object2DCore},
    },
    system_3d::{
        layer::{Layer3D, Layer3DCore},
        object::{Object3D, Object3DCore},
    },

};

struct Object2DComponent {
    count: i32,
}

impl Object2DComponent {
    fn new() -> Self {
        Object2DComponent { count: 0 }
    }
}
impl Component<Object2DCore> for Object2DComponent {
    fn on_update(&mut self, core: Rc<RefCell<Object2DCore>>) {
        self.count += 1;
        core.borrow_mut().count += 1;
    }
}

struct Layer2DComponent;
impl Component<Layer2DCore> for Layer2DComponent {}

struct SceneComponent;
impl Component<SceneCore> for SceneComponent {}

#[test]
fn updating() {
    let mut scene = scene::Scene::new();
    {
        let comp1 = SceneComponent;
        scene.add_component(Rc::new(RefCell::new(comp1)));
    }
    {
        let mut layer = Layer2D::new();
        {
            let comp1 = Layer2DComponent;
            let comp2 = Layer2DComponent;
            layer.add_component(Rc::new(RefCell::new(comp1)));
            layer.add_component(Rc::new(RefCell::new(comp2)));
        }
        {
            let mut object1 = Object2D::new();
            {
                let comp1 = Object2DComponent::new();
                let comp2 = Object2DComponent::new();
                object1.add_component(Rc::new(RefCell::new(comp1)));
                object1.add_component(Rc::new(RefCell::new(comp2)));
            }
            let object2 = Object2D::new();
            layer.add_object(object1);
            layer.add_object(object2);
        }
        scene.add_layer(layer);
    }
    {

        let mut layer = Layer2D::new();
        {
            let object1 = Object2D::new();
            let object2 = Object2D::new();
            layer.add_object(object1);
            layer.add_object(object2);
        }

        scene.add_layer(layer);
    }
    {

        let mut layer = Layer3D::new();
        {
            let object1 = Object3D::new();
            let object2 = Object3D::new();
            layer.add_object(object1);
            layer.add_object(object2);
        }

        scene.add_layer(layer);
    }

    for _ in 0..100 {
        scene.update();
    }
}