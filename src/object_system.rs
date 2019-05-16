use std::{cell::RefCell, rc::Rc};

pub trait CoreSystem {
    fn update(&mut self);
}

pub trait Component<T> {
    fn on_update(&mut self, _core: Rc<RefCell<T>>) {}
}

pub trait HasComponent<TCore>
where
    TCore: CoreSystem,
{
    fn core(&self) -> Rc<RefCell<TCore>>;
    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<TCore>>>>;

    fn add_component(&mut self, component: Rc<RefCell<Component<TCore>>>) {
        self.components().push(component);
    }
}

crate trait HasComponentInner<TCore>: HasComponent<TCore>
where
    TCore: CoreSystem,
{
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

impl<T, TCore> HasComponentInner<TCore> for T
where
    T: HasComponent<TCore>,
    TCore: CoreSystem,
{
}

pub trait Object {}

crate trait ObjectInner<TCore>: Object + HasComponentInner<TCore>
where
    TCore: CoreSystem,
{
    fn update(&mut self) {
        self.update_core();
        self.update_components();
    }
}

impl<T, TCore> ObjectInner<TCore> for T
where
    T: Object + HasComponentInner<TCore>,
    TCore: CoreSystem,
{
}

pub trait Layer<TObj>
where
    TObj: Object + 'static,
{
    fn objects(&mut self) -> &mut Vec<Box<TObj>>;

    fn add_object(&mut self, object: TObj) {
        self.objects().push(Box::new(object));
    }
}

crate trait LayerInner<TCore, TObjCore, TObj>:
    Layer<TObj> + HasComponentInner<TCore>
where
    TCore: CoreSystem,
    TObjCore: CoreSystem,
    TObj: ObjectInner<TObjCore> + 'static,
{
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

impl<T, TCore, TObjCore, TObj> LayerInner<TCore, TObjCore, TObj>
    for T
where
    T: Layer<TObj> + HasComponentInner<TCore>,
    TCore: CoreSystem,
    TObjCore: CoreSystem,
    TObj: ObjectInner<TObjCore> + 'static,
{
}

pub trait Scene<TLayer, TObj>
where
    TLayer: Layer<TObj>,
    TObj: Object + 'static,
{
    fn layers(&mut self) -> &mut Vec<Box<TLayer>>;

    fn add_layer(&mut self, layer: TLayer) {
        self.layers().push(Box::new(layer));
    }
}

crate trait SceneInner<TCore, TLyCore, TLayer, TObjCore, TObj>:
    Scene<TLayer, TObj> + HasComponentInner<TCore>
where
    TCore: CoreSystem,
    TLyCore: CoreSystem,
    TLayer: LayerInner<TLyCore, TObjCore, TObj>,
    TObjCore: CoreSystem,
    TObj: ObjectInner<TObjCore> + 'static,
{
    fn update_layers(&mut self) {
        let layers = self.layers();
        for layer in layers {
            layer.update();
        }
    }
}

impl<TCore, TLyCore, TLayer, TObjCore, TObj, T>
    SceneInner<TCore, TLyCore, TLayer, TObjCore, TObj> for T
where
    T: Scene<TLayer, TObj> + HasComponentInner<TCore>,
    TCore: CoreSystem,
    TLyCore: CoreSystem,
    TLayer: LayerInner<TLyCore, TObjCore, TObj>,
    TObjCore: CoreSystem,
    TObj: ObjectInner<TObjCore> + 'static,
{
}
