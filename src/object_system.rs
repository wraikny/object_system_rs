use std::{cell::RefCell, rc::Rc};

pub trait CoreSystem {
    fn update(&mut self);
}

pub trait Component<T> {
    fn on_update(&mut self, _core: Rc<RefCell<T>>) {}
}

pub trait HasComponent<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{
    fn core(&self) -> Rc<RefCell<TCore>>;
    fn components(&mut self) -> &mut Vec<Rc<RefCell<Component<TCore>>>>;

    fn add_component(&mut self, component: TComp) {
        self.components().push(Rc::new(RefCell::new(component)));
    }
}

crate trait HasComponentInner<TCore, TComp>: HasComponent<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
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

impl<T, TCore, TComp> HasComponentInner<TCore, TComp> for T
where
    T: HasComponent<TCore, TComp>,
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{
}

pub trait Object {}

crate trait ObjectInner<TCore, TComp>:
    Object + HasComponentInner<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{
    fn update(&mut self) {
        self.update_core();
        self.update_components();
    }
}

impl<T, TCore, TComp> ObjectInner<TCore, TComp> for T
where
    T: Object + HasComponentInner<TCore, TComp>,
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
{}

pub trait Layer<TObj>
where
    TObj: Object + 'static,
{
    fn objects(&mut self) -> &mut Vec<Box<TObj>>;

    fn add_object(&mut self, object: TObj) {
        self.objects().push(Box::new(object));
    }
}

crate trait LayerInner<TCore, TComp, TObjCore, TObjComp, TObj>:
    Layer<TObj> + HasComponentInner<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: ObjectInner<TObjCore, TObjComp> + 'static,
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

impl<T, TCore, TComp, TObjCore, TObjComp, TObj>
    LayerInner<TCore, TComp, TObjCore, TObjComp, TObj>
    for T
where
    T: Layer<TObj> + HasComponentInner<TCore, TComp>,
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: ObjectInner<TObjCore, TObjComp> + 'static,
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

crate trait SceneInner<TCore, TComp, TLyCore, TLyComp, TLayer, TObjCore, TObjComp, TObj>:
    Scene<TLayer, TObj> + HasComponentInner<TCore, TComp>
where
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TLyCore: CoreSystem,
    TLyComp: Component<TLyCore> + 'static,
    TLayer: LayerInner<TLyCore, TLyComp, TObjCore, TObjComp, TObj>,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: ObjectInner<TObjCore, TObjComp> + 'static,
{
    fn update_layers(&mut self) {
        let layers = self.layers();
        for layer in layers {
            layer.update();
        }
    }
}

impl<TCore, TComp, TLyCore, TLyComp, TLayer, TObjCore, TObjComp, TObj, T>
    SceneInner<TCore, TComp, TLyCore, TLyComp, TLayer, TObjCore, TObjComp, TObj>
    for T
where
    T: Scene<TLayer, TObj> + HasComponentInner<TCore, TComp>,
    TCore: CoreSystem,
    TComp: Component<TCore> + 'static,
    TLyCore: CoreSystem,
    TLyComp: Component<TLyCore> + 'static,
    TLayer: LayerInner<TLyCore, TLyComp, TObjCore, TObjComp, TObj>,
    TObjCore: CoreSystem,
    TObjComp: Component<TObjCore> + 'static,
    TObj: ObjectInner<TObjCore, TObjComp> + 'static,
{}