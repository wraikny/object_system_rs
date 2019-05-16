use std::{cell::RefCell, rc::Rc};

crate trait CoreSystem {
    fn update(&mut self);
}

pub trait Component<T> {
    fn on_update(&mut self, _core: Rc<RefCell<T>>) {}
}

crate trait ObjectSystem<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{
    fn core(&self) -> Rc<RefCell<TCore>>;
    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<TCore>>>>;

    fn add_component(&mut self, component: TComp) {
        self.components().push(Rc::new(RefCell::new(component)));
    }

    fn update_core(&self) {
        self.core().borrow_mut().update();
    }

    //fn update_components(&mut self);
    fn update_components(&mut self) {
        let core = self.core();
        for c in self.components() {
            c.borrow_mut().on_update(core.clone());
        }
    }
}

crate trait Object<TCore, TComp>: ObjectSystem<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{
    fn update(&mut self) {
        self.update_core();
        self.update_components();
    }
}

crate trait Layer<TCore, TComp, TObjCore, TObjComp, TObj>: ObjectSystem<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: Object<TObjCore, TObjComp>,
{
    fn objects(&mut self) -> &mut Vec<TObj>;

    fn add_object(&mut self, object: TObj) {
        self.objects().push(object);
    }

    fn update_objects(&mut self) {
        let objects = self.objects();
        for object in objects {
            object.update();
        }
    }

    fn update(&mut self) {
        self.update_core();
        self.update_components();
        self.update_objects();
    }
}

crate trait Scene<TCore, TComp, TLayerCore, TLayerComp, TLayer, TObjCore, TObjComp, TObj>:
    ObjectSystem<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TLayerCore: CoreSystem,
    TLayerComp: Component<TLayerCore> + 'static,
    TLayer: Layer<TLayerCore, TLayerComp, TObjCore, TObjComp, TObj>,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: Object<TObjCore, TObjComp>,
{
    fn layers(&mut self) -> &mut Vec<TLayer>;

    fn add_layer(&mut self, layer: TLayer) {
        self.layers().push(layer);
    }

    fn update_layers(&mut self) {
        let layers = self.layers();
        for layer in layers {
            layer.update();
        }
    }
}