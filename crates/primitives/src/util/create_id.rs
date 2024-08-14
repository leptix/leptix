use leptos::prelude::*;

pub fn create_id() -> Signal<String> {
  let (id, _) = create_signal(nanoid::nanoid!());

  id.into()
}
