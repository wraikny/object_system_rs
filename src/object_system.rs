use std::{cell::{RefCell}, rc::Rc};

pub trait Component<T> {
    fn on_added(&mut self, _ : &mut T) {}
    fn on_owner_added(&mut self, _ : &mut T) {}
    fn on_update(&mut self, _ : &mut T) {}
}


pub struct Events<T> {
    on_added_event : Option<Box<Fn(&mut T)>>,
    on_update_event : Option<Box<Fn(&mut T)>>,
}

impl<T> Events<T> {
    pub fn on_added_event(f : Box<Fn(&mut T)>) -> Events<T> {
        Events {
            on_added_event : Some(f),
            on_update_event : None,
        }
    }

    pub fn on_update_event(f : Box<Fn(&mut T)>) -> Events<T> {
        Events {
            on_update_event : Some(f),
            on_added_event : None,
        }
    }
}

impl<T> Component<T> for Events<T> {
    fn on_added(&mut self, owner : &mut T) {
        self.on_added_event.iter().for_each(|f| f(owner));
    }

    fn on_update(&mut self, owner : &mut T) {
        self.on_update_event.iter().for_each(|f| f(owner));
    }
}

pub trait ComponentSystem<T> {
    fn add_component(&mut self, component: Rc<RefCell<Component<T>>>);
}

impl<T> ComponentSystem<T> {
    pub fn add_component_static<U>(&mut self, component : U)
        where U : 'static + Component<T>
    {
        self.add_component(Rc::new(RefCell::new(component)));
    }
}

pub struct Components<T> {
    components : Vec<Rc<RefCell<Component<T>>>>,
}

impl<T> Default for Components<T> {
    fn default() -> Self {
        Components {
            components : Vec::new(),
        }
    }
}

impl<T> Components<T> {
    crate fn add_component(&mut self, component: Rc<RefCell<Component<T>>>) {
        self.components.push(component.clone());
    }
}

pub trait GameObject<T> {
    fn object_mut(&mut self) -> &mut T;
    fn update(&mut self);
}

mod sys {
    crate enum AltseedObject2DImpl{}
    // extern {
        // crate fn object2d_create() -> *mut AltseedObject2DImpl;
        // crate fn object2d_free(obj: *mut AltseedObject2DImpl);
    // }
}

pub struct Object2D {
    // crate raw : *mut sys::AltseedObject2DImpl,
}

impl Default for Object2D {
    fn default() -> Self {
        Object2D {
            // raw : sys::object2d_create(),
        }
    }
}

impl Drop for Object2D {
    fn drop(&mut self) {
        // sys::object2d_free(&mut self.raw);
    }
}

#[macro_export]
macro_rules! impl_object2d {
    ( @ $name:ident < $( $N:ident $(: $b0:ident $(+$b:ident)* )? ),* > { $($result:tt)* }) => (
        struct $name {
            $($result)*
            object2d : Object2D,
            components : Components<Self>,
        }

        impl < $( $N $(: $b0 $(+$b)* )? ),* > ComponentSystem<Self> for $name < $( $N ),* > {
            fn add_component(&mut self, component : Rc<RefCell<Component<Self>>>) {
                self.components.add_component(component.clone());
                component.borrow_mut().on_added(self);
            }
        }

        impl < $( $N $(: $b0 $(+$b)* )? ),* > GameObject<Object2D> for $name < $( $N ),* > {
            fn object_mut(&mut self) -> &mut Object2D {
                &mut self.object2d
            }

            fn update(&mut self) {
                for c in self.components.components.clone() {
                    c.borrow_mut().on_update(self);
                }
            }
        }
    );
    ( $name:ident { $( $param:ident : $type:ty ),* $(,)* } ) => (
        impl_object2d!(@ $name < > { $($param : $type,)* });
    );
    ( $name:ident ) => (
        impl_object2d!(@ $name < > { } );
    );
}

impl_object2d! (
    EmptyObject2D {
        name : String
    }
);

fn test() {
    // let a = EmptyObject2D;
}

pub struct Layer2D {
    objects : Vec<Rc<RefCell<GameObject<Object2D>>>>,
}

impl Layer2D {
    pub fn new() -> Self {
        Layer2D {
            objects : Vec::new(),
        }
    }

    // crate fn update(&mut self) {
    //     for c in self.components.components.clone() {
    //         c.borrow_mut().on_update(self);
    //     }

    //     for o in self.objects.clone() {
    //         o.borrow_mut().update();
    //     }
    // }
}

// impl ComponentSystem<Self> for Layer2D {
//     fn add_component(&mut self, component : Rc<RefCell<Component<Self>>>) {
//         self.components.add_component(component.clone());
//         component.borrow_mut().on_added(self);
//     }
// }
