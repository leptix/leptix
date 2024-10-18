use leptos::prelude::*;

pub struct CreateControllableSignalProps<T: Clone + PartialEq + Send + Sync + 'static> {
  pub value: Signal<Option<T>>,
  pub default_value: Signal<Option<T>>,
  pub on_change: Callback<T>,
}

#[derive(Clone, Copy)]
pub struct WriteControllableSignal<T: Clone + Send + Sync + 'static> {
  is_controlled: Signal<bool>,
  value: Signal<Option<T>>,
  pub(crate) set_uncontrolled_value: WriteSignal<Option<T>>,
  pub(crate) on_change: Callback<Option<T>>,
}

impl<T: Clone + Send + Sync + 'static> WriteControllableSignal<T> {
  pub fn set(&self, value: T) {
    if self.is_controlled.get() {
      self.on_change.run(Some(value));
    } else {
      let set_uncontrolled_value = self.set_uncontrolled_value;
      let cloned_value = value.clone();

      // request_animation_frame(move || {
      set_uncontrolled_value.set(Some(cloned_value));
      // self.set_uncontrolled_value.set(Some(cloned_value));
      // });
      self.on_change.run(Some(value));
    }
  }

  pub fn update(&self, callback: impl FnOnce(&mut Option<T>)) {
    if self.is_controlled.get() {
      let mut value = self.value.get();

      callback(&mut value);

      self.on_change.run(value);
    } else {
      self.set_uncontrolled_value.update(|value| {
        callback(value);
        self.on_change.run(value.clone());
      });
    }
  }
}

pub fn create_controllable_signal<T: Clone + PartialEq + Send + Sync + 'static>(
  CreateControllableSignalProps {
    value,
    default_value,
    on_change,
  }: CreateControllableSignalProps<T>,
) -> (Signal<Option<T>>, WriteControllableSignal<T>) {
  let (uncontrolled_value, set_uncontrolled_value) =
    create_uncontrolled_signal(CreateUncontrolledSignalProps {
      default_value,
      on_change,
    });

  let is_controlled = Signal::derive(move || value.get().is_some());
  let value = Signal::derive(move || {
    if is_controlled.get() {
      value.get()
    } else {
      uncontrolled_value.get()
    }
  });

  (
    value,
    WriteControllableSignal {
      is_controlled,
      value,
      set_uncontrolled_value,
      on_change: Callback::new(move |value| {
        if let Some(value) = value {
          on_change.run(value);
        }
      }),
    },
  )
}

pub(crate) struct CreateUncontrolledSignalProps<T: Clone + Send + Sync + 'static> {
  default_value: Signal<Option<T>>,
  on_change: Callback<T>,
}

fn create_uncontrolled_signal<T: Clone + PartialEq + Send + Sync + 'static>(
  CreateUncontrolledSignalProps {
    default_value,
    on_change,
  }: CreateUncontrolledSignalProps<T>,
) -> (ReadSignal<Option<T>>, WriteSignal<Option<T>>) {
  let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.get_untracked());

  let prev_value = StoredValue::new(uncontrolled_value.get_untracked());

  Effect::new(move |_| {
    if prev_value.get_value() != uncontrolled_value.get() {
      if let Some(value) = uncontrolled_value.get() {
        on_change.run(value);
      }

      prev_value.set_value(uncontrolled_value.get());
    }
  });

  (uncontrolled_value, set_uncontrolled_value)
}
