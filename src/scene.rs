use super::super::object_system::{
    Component,
    CoreSystem,
    ObjectSystem,
    Layer,
    Scene,
};

pub struct SceneCore;
impl CoreSystem for SceneCore {
    fn update(&mut self) {}
}

pub struct Scene {
    core : SceneCore,
    components : Vec<Box<Component<SceneCore>>>,
    layers : Vec<Box<Layer>>
}


impl<TComp> ObjectSystem<SceneCore, TComp> for Scene
    where TComp : Component<SceneCore> + 'static
{
    fn core(&mut self) -> &mut SceneCore {
        &mut self.core
    }

    fn components(&mut self) -> Vec<Box<TComp>> {
        self.components
    }

    fn update_components(&mut self) {
        for c in &mut self.components {
            c.on_update(&mut self.core);
        }
    }
}

impl<TComp, TLayerCore, TLayerComp, TLayer, TObjCore, TObjComp, TObj>
Scene<SceneCore, TComp, TLayerCore, TLayerComp, TLayer, TObjCore, TObjComp, TObj>
for Scene
    where
        TComp : Component<TCore> + 'static,
        TLayerCore : CoreSystem,
        TLayerComp : Component<TLayerCore> + 'static,
        TLayer : Layer<TLayerCore, TLayerComp, TObjCore, TObjComp, TObj>,
        TObjCore : CoreSystem,
        TObjComp : Component<TObjCore> + 'static,
        TObj : Object<TObjCore, TObjComp>
{
    fn layers(&mut self) -> &mut Vec<impl Layer> {
        &mut self.layers
    }
}