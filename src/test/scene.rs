use super::super::{
    object_system::{Component, HasComponent, Layer, Scene, SceneInner},
    scene::{self, SceneCore},
    system_2d::{
        layer::{Layer2D, Layer2DCore},
        object::{Object2D, Object2DCore},
    },

};

struct ObjectComponent;
impl Component<Object2DCore> for ObjectComponent {}

struct LayerComponent;
impl Component<Layer2DCore> for LayerComponent {}

struct SceneComponent;
impl Component<SceneCore> for SceneComponent {}

#[test]
fn updating() {
    updating_impl::<SceneComponent, LayerComponent, ObjectComponent>();
}

fn updating_impl<TComp, TLyComp, TObjComp>()
where
    TComp: Component<SceneCore> + 'static,
    TLyComp: Component<Layer2DCore> + 'static,
    TObjComp: Component<Object2DCore> + 'static,
{
    let mut sample_scene = scene::Scene::new();
    {
        let mut layer1 = Layer2D::new();
        {
            let comp1 = LayerComponent;
            let comp2 = LayerComponent;
            layer1.add_component(comp1);
            layer1.add_component(comp2);
        }
        {
            let mut object1 = Object2D::new();
            {
                let comp1 = ObjectComponent;
                let comp2 = ObjectComponent;
                object1.add_component(comp1);
                object1.add_component(comp2);
            }
            let object2 = Object2D::new();
            layer1.add_object(object1);
            layer1.add_object(object2);
        }

        let mut layer2 = Layer2D::new();
        {
            let object1 = Object2D::new();
            let object2 = Object2D::new();
            layer2.add_object(object1);
            layer2.add_object(object2);
        }

        sample_scene.add_layer(layer1);
        sample_scene.add_layer(layer2);
    }
    let scene = (&mut sample_scene) as (&mut SceneInner<_, TComp, _, TLyComp, _, _, TObjComp, _>);
    for _ in 0..100 {
        scene.update_layers();
    }
}