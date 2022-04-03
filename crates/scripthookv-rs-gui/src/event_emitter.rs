use std::{
  any::{Any, TypeId},
  borrow::Borrow,
  collections::HashMap,
  marker::PhantomData
};

#[derive(Default)]
pub struct EventEmitter<Sender: 'static> {
  listeners: HashMap<TypeId, Vec<Box<dyn Any>>>,
  phantom:   PhantomData<Sender>
}

impl<Sender: 'static> EventEmitter<Sender> {
  pub fn on<Event: 'static>(&mut self, callback: impl Fn(&Sender, &Event) + 'static) {
    let entry = self
      .listeners
      .entry(TypeId::of::<Event>())
      .or_insert(Default::default());
    let boxed_cb: Box<dyn Fn(&Sender, &Event)> = Box::new(callback);
    entry.push(Box::new(boxed_cb));
  }

  pub fn emit<Event: 'static>(&self, sender: &Sender, data: &Event) {
    if let Some(listeners) = self.listeners.get(&TypeId::of::<Event>()) {
      for listener in listeners {
        let any: &dyn Any = listener.borrow();
        let callback = any.downcast_ref::<Box<dyn Fn(&Sender, &Event)>>().unwrap();
        callback(sender, data);
      }
    }
  }
}
