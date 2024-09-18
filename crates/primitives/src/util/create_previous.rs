use leptos::prelude::*;

#[derive(Clone)]
pub(crate) struct Previous<T> {
  value: T,
  previous: T,
}

pub(crate) fn create_previous<T: Clone + PartialEq + Send + Sync + 'static>(
  initial: Signal<T>,
) -> Signal<T> {
  let prev = StoredValue::new(Previous {
    value: initial.get_untracked(),
    previous: initial.get_untracked(),
  });

  Memo::new(move |_| {
    if prev.get_value().value != initial.get() {
      prev.set_value(Previous {
        previous: prev.get_value().value,
        value: initial.get(),
      });
    }

    prev.get_value().value
  })
  .into()
}
