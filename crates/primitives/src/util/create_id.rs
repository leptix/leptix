use leptos::*;

pub(crate) fn create_id() -> Signal<String> {
  let (id, _) = create_signal(nanoid::nanoid!());

  Signal::derive(id)
}

// pub(crate) fn create_deterministic_id(id: String) -> Signal<String> {
//   let (id, set_id) = create_signal(nanoid::naonid!());

//   Effect::new(move |_| {

//   });

// }
