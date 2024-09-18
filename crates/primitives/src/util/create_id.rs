use leptos::prelude::*;

pub fn create_id() -> Signal<String> {
  let (id, _) = signal(nanoid::nanoid!());

  id.into()
}
