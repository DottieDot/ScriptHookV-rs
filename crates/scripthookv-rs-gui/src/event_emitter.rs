use std::{
  any::{Any, TypeId},
  borrow::Borrow,
  collections::HashMap,
  marker::PhantomData
};

pub struct EventEmitter<Sender: 'static> {
  listeners: HashMap<TypeId, Vec<Box<dyn Any>>>,
  phantom:   PhantomData<Sender>
}

impl<Sender: 'static> EventEmitter<Sender> {
  pub fn on<Event: 'static>(&mut self, callback: impl Fn(&mut Sender, &Event) + 'static) {
    let entry = self
      .listeners
      .entry(TypeId::of::<Event>())
      .or_insert_with(Default::default);
    let boxed_cb: Box<dyn Fn(&mut Sender, &Event)> = Box::new(callback);
    entry.push(Box::new(boxed_cb));
  }

  pub fn emit<Event: 'static>(&self, sender: &mut Sender, data: &Event) {
    if let Some(listeners) = self.listeners.get(&TypeId::of::<Event>()) {
      for listener in listeners {
        let any: &dyn Any = listener.borrow();
        let callback = any
          .downcast_ref::<Box<dyn Fn(&mut Sender, &Event)>>()
          .unwrap();
        callback(sender, data);
      }
    }
  }
}

impl<Sender: 'static> std::default::Default for EventEmitter<Sender> {
  fn default() -> Self {
    Self {
      listeners: Default::default(),
      phantom:   Default::default()
    }
  }
}
