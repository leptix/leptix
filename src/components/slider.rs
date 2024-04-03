use std::collections::HashMap;

use leptos::{
  html::{AnyElement, Input, Span},
  *,
};

use leptos_use::use_element_size;
use strum::EnumString;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
  js_sys::{Array, Function, JsString, Object, Reflect},
  DomRect, Event, EventInit, KeyboardEvent, PointerEvent,
};

use crate::{
  components::{
    collection::{use_collection_context, CollectionContextValue},
    primitive::Primitive,
  },
  util::{
    create_controllable_signal::{create_controllable_signal, CreateControllableSignalProps},
    create_previous::create_previous,
    linear_scale, Direction, Orientation,
  },
  Attributes,
};

#[derive(Clone)]
struct SliderContextValue {
  disabled: Signal<bool>,
  min: Signal<f64>,
  max: Signal<f64>,
  values: Signal<Vec<f64>>,
  value_index_to_change: StoredValue<usize>,
  thumbs: StoredValue<Vec<HtmlElement<AnyElement>>>,
  orientation: Signal<Orientation>,
}

#[component]
pub fn SliderRoot(
  #[prop(optional)] name: Option<MaybeSignal<String>>,
  #[prop(optional)] min: Option<MaybeSignal<f64>>,
  #[prop(optional)] max: Option<MaybeSignal<f64>>,
  #[prop(optional)] step: Option<MaybeSignal<f64>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] disabled: Option<MaybeSignal<bool>>,
  #[prop(optional)] min_steps_between_thumbs: Option<MaybeSignal<f64>>,
  #[prop(optional)] value: Option<MaybeSignal<Vec<f64>>>,
  #[prop(optional)] default_value: Option<MaybeSignal<Vec<f64>>>,
  #[prop(optional)] inverted: Option<MaybeSignal<bool>>,
  #[prop(optional)] on_value_change: Option<Callback<Vec<f64>>>,
  #[prop(optional)] on_value_commit: Option<Callback<Vec<f64>>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let node_ref = NodeRef::<AnyElement>::new();
  let thumbs = StoredValue::new(Vec::<HtmlElement<AnyElement>>::new());
  let value_index_to_change = StoredValue::new(0usize);

  let is_form_control = Signal::derive(move || {
    if let Some(node) = node_ref.get() {
      node.closest("form").ok().flatten().is_some()
    } else {
      true
    }
  });

  let (values, set_values) = create_controllable_signal(CreateControllableSignalProps {
    value: Signal::derive(move || value.as_ref().map(|value| value.get())),
    default_value: Signal::derive(move || {
      Some(
        default_value
          .as_ref()
          .map(|default_value| default_value.get())
          .unwrap_or(vec![min.map(|min| min.get()).unwrap_or(0.)]),
      )
    }),
    on_change: Callback::new(move |value| {
      let thumbs = thumbs.get_value();
      let thumbs = Vec::from_iter(thumbs.iter());

      thumbs
        .get(value_index_to_change.get_value())
        .map(|thumb| _ = thumb.focus());

      if let Some(on_value_change) = on_value_change {
        on_value_change(value);
      }
    }),
  });

  let values_before_slide_start = StoredValue::new(values.get());

  let update_values = move |value: f64, at_index: usize, commit: bool| {
    let decimal_count = get_decimal_count(step.map(|step| step.get()).unwrap_or(1.));
    let snap_to_step = round_value(
      (value - min.map(|min| min.get()).unwrap_or(0.)) / step.map(|step| step.get()).unwrap_or(1.),
      decimal_count as u32,
    );
    let next_value = snap_to_step.clamp(
      min.map(|min| min.get()).unwrap_or(0.),
      max.map(|max| max.get()).unwrap_or(100.),
    );

    set_values.update(move |values| {
      let next_values =
        get_next_sorted_values(values.as_ref().unwrap_or(&vec![]), next_value, at_index);

      if has_min_steps_between_values(
        values.as_ref().unwrap_or(&vec![]),
        min_steps_between_thumbs
          .map(|min_steps| min_steps.get())
          .unwrap_or(0.)
          * step.map(|step| step.get()).unwrap_or(0.),
      ) {
        if let Some(position) = next_values.iter().position(|value| value == &next_value) {
          value_index_to_change.set_value(position);
        }

        let has_changed = next_values
          .iter()
          .zip(values.as_ref().unwrap_or(&vec![]).iter())
          .all(|(a, b)| a == b);

        if has_changed {
          if commit {
            if let Some(on_value_commit) = on_value_commit {
              on_value_commit(next_values.clone());
            }
          }

          *values = Some(next_values);
        }
      }
    });
  };

  let start_update = update_values.clone();
  let handle_slide_start = Callback::new(move |value: f64| {
    let Some(closest_index) = find_closest_index(&values.get().unwrap_or_default(), value) else {
      return;
    };

    let req_update = start_update.clone();
    request_animation_frame(move || {
      // start_update(value, closest_index, false);
      req_update(value, closest_index, false);
    });
  });

  let move_update = update_values.clone();
  let handle_slide_move = Callback::new(move |value: f64| {
    let req_update = move_update.clone();

    request_animation_frame(move || {
      req_update(value, value_index_to_change.get_value(), false);
      // move_update(value, value_index_to_change.get_value(), false);
    });
  });

  let handle_slide_end = Callback::new(move |_: ()| {
    let prev_value = values_before_slide_start
      .get_value()
      .map(|values| values.get(value_index_to_change.get_value()).cloned())
      .flatten();
    let next_value = values
      .get()
      .map(|values| values.get(value_index_to_change.get_value()).cloned())
      .flatten();
    let has_changed = next_value != prev_value;

    if has_changed {
      if let Some(on_value_commit) = on_value_commit {
        on_value_commit(values.get().unwrap_or_default());
      }
    }
  });

  provide_context(SliderContextValue {
    disabled: Signal::derive(move || disabled.map(|disabled| disabled.get()).unwrap_or(false)),
    min: Signal::derive(move || min.map(|min| min.get()).unwrap_or(0.)),
    max: Signal::derive(move || max.map(|max| max.get()).unwrap_or(100.)),
    value_index_to_change,
    thumbs,
    values: Signal::derive(move || values.get().unwrap_or_default()),
    orientation: Signal::derive(move || {
      orientation
        .map(|orientation| orientation.get())
        .unwrap_or_default()
    }),
  });

  provide_context(CollectionContextValue::<SliderCollectionItem, AnyElement> {
    collection_ref: NodeRef::new(),
    item_map: RwSignal::new(HashMap::new()),
  });

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      (
        "aria-disabled",
        Signal::derive(move || disabled.map(|disabled| disabled.get())).into_attribute(),
      ),
      (
        "data-disabled",
        Signal::derive(move || disabled.map(|disabled| disabled.get().then_some("")))
          .into_attribute(),
      ),
    ]
    .into_iter(),
  );

  let home_key_down_update = update_values.clone();
  let end_key_down_update = update_values.clone();

  view! {
    <Slider
      node_ref=node_ref
      attrs=merged_attrs
      min=Signal::derive(move || min.map(|min| min.get()).unwrap_or(0.))
      max=Signal::derive(move || max.map(|max| max.get()).unwrap_or(100.))
      inverted=Signal::derive(move || inverted.map(|inverted| inverted.get()).unwrap_or(false))
      orientation=Signal::derive(move || {
        orientation
          .map(|orientation| orientation.get())
          .unwrap_or_default()
      })
      on_slide_start=handle_slide_start
      on_slide_move=handle_slide_move
      on_slide_end=handle_slide_end
      on_home_key_down=Callback::new(move |_| {
        if disabled.map(|disabled| disabled.get()).unwrap_or(false) == false {
          home_key_down_update(min.map(|min| min.get()).unwrap_or(0.), 0, true);
        }
      })
      on_end_key_down=Callback::new(move |_| {
        if disabled.map(|disabled| disabled.get()).unwrap_or(false) == false {
          end_key_down_update(min.map(|min| min.get()).unwrap_or(0.), values.get().unwrap_or_default().len() - 1, true);
        }
      })
      on_step_key_down=Callback::new(move |Step{ event, direction }| {
        if disabled.map(|disabled| disabled.get()).unwrap_or(false) {
          return;
        }

        let is_page_key = ["PageUp", "PageDown"].contains(&event.key().as_str());
        let is_skip_key = is_page_key || (event.shift_key() && ["ArrowUp", "ArrowLeft", "ArrowRight", "ArrowDown"].contains(&event.key().as_str()));
        let multiplier = if is_skip_key { 10.0f64 } else { 1.0f64 };
        let at_index = value_index_to_change.get_value();
        let value = values.get().unwrap_or_default().get(at_index).cloned().unwrap_or(0.);
        let step_in_direction = step.map(|step| step.get()).unwrap_or(1.) * multiplier * match direction { OrientationDirection::Forward => 1.0f64, OrientationDirection::Backward => -1.0f64 };

        update_values(value + step_in_direction, at_index, true);
      })
    >
      {children()}
      // {move || is_form_control.get().then(|| {
      //   let values = values.clone();

      //   view! {
      //     <For
      //       each=move || {
      //         let values = values
      //           .get()
      //           .unwrap_or_default();

      //         values
      //           .into_iter()
      //           .enumerate()
      //           .collect::<Vec<_>>()
      //       }
      //       key=|(index, _)| *index
      //       children=move |(_, value)| {
      //         view! {
      //           <BubbleInput
      //             name=Signal::derive(move || name.map(|name| format!("{}{}", name.get(), if values.get().unwrap_or_default().len() > 1 { "[]" } else { "" })))
      //             value=Signal::derive(move || value)
      //           />
      //         }
      //       }
      //     />
      //   }
      // })}
    </Slider>
  }
}

#[derive(Clone, PartialEq, EnumString, strum::Display, strum::IntoStaticStr)]
enum Side {
  Top,
  Right,
  Bottom,
  Left,
}

#[derive(Clone, PartialEq)]
enum OrientationDirection {
  Forward,
  Backward,
}

#[derive(Clone)]
enum Size {
  Width,
  Height,
}

#[derive(Clone)]
struct OrientationContextValue {
  start_edge: Signal<Side>,
  end_edge: Signal<Side>,
  size: Signal<Size>,
  direction: Signal<OrientationDirection>,
}

struct Step {
  event: KeyboardEvent,
  direction: OrientationDirection,
}
#[derive(Clone)]
enum SlideDirection {
  FromLeft,
  FromRight,
  FromBottom,
  FromTop,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct SliderCollectionItem;

#[component]
fn Slider(
  max: Signal<f64>,
  min: Signal<f64>,
  inverted: Signal<bool>,
  orientation: Signal<Orientation>,
  #[prop(optional)] direction: Option<MaybeSignal<Direction>>,
  #[prop(optional)] on_slide_start: Option<Callback<f64>>,
  #[prop(optional)] on_slide_move: Option<Callback<f64>>,
  #[prop(optional)] on_slide_end: Option<Callback<()>>,
  #[prop(optional)] on_home_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(optional)] on_end_key_down: Option<Callback<KeyboardEvent>>,
  #[prop(optional)] on_step_key_down: Option<Callback<Step>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let dom_rect = StoredValue::<Option<DomRect>>::new(None);

  let (orientation_context, pointer_value, slide_direction) = match orientation.get() {
    Orientation::Horizontal => {
      let is_left_to_right = Signal::derive(move || {
        direction
          .map(|direction| direction.get())
          .unwrap_or_default()
          == Direction::LeftToRight
      });

      let is_sliding_from_left = Signal::derive(move || {
        (is_left_to_right.get() && !inverted.get()) || (!is_left_to_right.get() && inverted.get())
      });

      let pointer_value = Callback::new(move |pointer: i32| {
        let rect = dom_rect
          .get_value()
          .unwrap_or(node_ref.get().unwrap().get_bounding_client_rect());

        let input = (0., rect.width());
        let output = if is_sliding_from_left.get() {
          (min.get(), max.get())
        } else {
          (max.get(), min.get())
        };
        let value = linear_scale(input, output);

        dom_rect.set_value(Some(rect.clone()));

        value(pointer as f64 - rect.left())
      });

      (
        OrientationContextValue {
          start_edge: Signal::derive(move || {
            if is_sliding_from_left.get() {
              Side::Left
            } else {
              Side::Right
            }
          }),
          end_edge: Signal::derive(move || {
            if is_sliding_from_left.get() {
              Side::Right
            } else {
              Side::Left
            }
          }),
          direction: Signal::derive(move || {
            if is_sliding_from_left.get() {
              OrientationDirection::Forward
            } else {
              OrientationDirection::Backward
            }
          }),
          size: Signal::derive(|| Size::Width),
        },
        pointer_value,
        Signal::derive(move || {
          if is_sliding_from_left.get() {
            SlideDirection::FromLeft
          } else {
            SlideDirection::FromRight
          }
        }),
      )
    }
    Orientation::Vertical => {
      let is_sliding_from_bottom = Signal::derive(move || !inverted.get());

      let pointer_value = Callback::new(move |pointer: i32| {
        let rect = dom_rect
          .get_value()
          .unwrap_or(node_ref.get().unwrap().get_bounding_client_rect());

        let input = (0., rect.width());
        let output = if is_sliding_from_bottom.get() {
          (min.get(), max.get())
        } else {
          (max.get(), min.get())
        };
        let value = linear_scale(input, output);

        dom_rect.set_value(Some(rect.clone()));

        value(pointer as f64 - rect.left())
      });

      (
        OrientationContextValue {
          start_edge: Signal::derive(move || {
            if is_sliding_from_bottom.get() {
              Side::Bottom
            } else {
              Side::Top
            }
          }),
          end_edge: Signal::derive(move || {
            if is_sliding_from_bottom.get() {
              Side::Top
            } else {
              Side::Bottom
            }
          }),
          direction: Signal::derive(move || {
            if is_sliding_from_bottom.get() {
              OrientationDirection::Forward
            } else {
              OrientationDirection::Backward
            }
          }),
          size: Signal::derive(|| Size::Height),
        },
        pointer_value,
        Signal::derive(move || {
          if is_sliding_from_bottom.get() {
            SlideDirection::FromBottom
          } else {
            SlideDirection::FromTop
          }
        }),
      )
    }
  };

  let mut merged_attrs = attrs.clone();
  merged_attrs.push((
    "data-orientation",
    Signal::derive(move || match orientation.get() {
      Orientation::Horizontal => "horizontal",
      Orientation::Vertical => "vertical",
    })
    .into_attribute(),
  ));

  if orientation.get() == Orientation::Horizontal {
    if let Some(direction) = direction {
      merged_attrs.push((
        "dir",
        Signal::derive(move || match direction.get() {
          Direction::LeftToRight => "ltr",
          Direction::RightToLeft => "rtl",
        })
        .into_attribute(),
      ));
    }
  }

  let context =
    use_context::<SliderContextValue>().expect("Slider must be used in a SliderRoot component");

  provide_context(orientation_context);

  // node_ref.on_load(|node| {
  //   _ = node.style("--primitive-slider-thumb-transform", "translateX(50%)");
  // });

  view! {
    <Primitive
      element=html::span
      node_ref=Some(node_ref)
      attrs=merged_attrs
      on:keydown=move |ev: KeyboardEvent| {
        if ev.key() == "Home" {
          if let Some(on_home_key_down) = on_home_key_down {
            on_home_key_down(ev.clone());
          }

          ev.prevent_default();
        } else if ev.key() == "End" {
          if let Some(on_end_key_down) = on_end_key_down {
            on_end_key_down(ev.clone());
          }

          ev.prevent_default();
        } else if ["PageUp", "PageDown", "ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"].contains(&ev.key().as_ref()) {
          if let Some(on_step_key_down) = on_step_key_down {
            let is_back_key = match slide_direction.get() {
              SlideDirection::FromLeft => ["Home", "PageDown", "ArrowDown", "ArrowLeft"].contains(&ev.key().as_ref()),
              SlideDirection::FromRight => ["Home", "PageDown", "ArrowDown", "ArrowRight"].contains(&ev.key().as_ref()),
              SlideDirection::FromTop => ["Home", "PageDown", "ArrowDown", "ArrowLeft"].contains(&ev.key().as_ref()),
              SlideDirection::FromBottom => ["Home", "PageDown", "ArrowUp", "ArrowLeft"].contains(&ev.key().as_ref()),
            };

            on_step_key_down(Step {
              event: ev.clone(),
              direction: if is_back_key {
                OrientationDirection::Backward
              } else {
                OrientationDirection::Forward
              }
            });
          }

          ev.prevent_default();
        }
      }
      on:pointerdown=move |ev: PointerEvent| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target_el) = target.dyn_ref::<web_sys::HtmlElement>() else {
          return;
        };

        _ = target_el.set_pointer_capture(ev.pointer_id());
        ev.prevent_default();

        if context.thumbs.get_value().iter().any(|el| {
          let el: &web_sys::HtmlElement = &el;

          el == target_el
        }) {
          _ = target_el.focus();
        } else {
          if let Some(on_slide_start) = on_slide_start {
            on_slide_start(pointer_value(ev.client_x()));
          }
        }
      }
      on:pointermove=move |ev: PointerEvent| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target_el) = target.dyn_ref::<web_sys::HtmlElement>() else {
          return;
        };

        if target_el.has_pointer_capture(ev.pointer_id()) {
          if let Some(on_slide_move) = on_slide_move {
            on_slide_move(pointer_value(ev.client_x()));
          }
        }
      }
      on:pointerup=move |ev: PointerEvent| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target_el) = target.dyn_ref::<web_sys::HtmlElement>() else {
          return;
        };

        if target_el.has_pointer_capture(ev.pointer_id()) {
          _ = target_el.release_pointer_capture(ev.pointer_id());

          dom_rect.set_value(None);

          if let Some(on_slide_end) = on_slide_end {
            on_slide_end(());
          }
        }
      }
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn SliderTrack(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context = use_context::<SliderContextValue>()
    .expect("SliderTrack must be used in a SliderRoot component");

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      (
        "data-disabled",
        Signal::derive(move || context.disabled.get().then_some("")).into_attribute(),
      ),
      (
        "data-orientation",
        Signal::derive(move || match context.orientation.get() {
          Orientation::Horizontal => "horizontal",
          Orientation::Vertical => "vertical",
        })
        .into_attribute(),
      ),
    ]
    .into_iter(),
  );

  view! {
    <Primitive
      element=html::span
      attrs=merged_attrs
      node_ref=Some(node_ref)
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn SliderRange(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context = use_context::<SliderContextValue>()
    .expect("SliderRange must be used in a SliderRoot component");
  let orientation = use_context::<OrientationContextValue>()
    .expect("SliderRange must be used in a SliderRoot component");

  let value_count = Signal::derive(move || context.values.get().len());
  let percentages = Signal::derive(move || {
    context
      .values
      .get()
      .iter()
      .map(|value| convert_value_to_percentage(*value, context.min.get(), context.max.get()))
      .collect::<Vec<_>>()
  });

  let offset_start = Signal::derive(move || {
    if value_count.get() > 1 {
      percentages
        .get()
        .iter()
        .fold(f64::INFINITY, |min, &x| min.min(x))
    } else {
      0.0f64
    }
  });
  let offset_end =
    Signal::derive(move || 100.0f64 - percentages.get().iter().fold(0.0f64, |max, &x| max.max(x)));

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend([
    (
      "data-disabled",
      Signal::derive(move || context.disabled.get().then_some("")).into_attribute(),
    ),
    (
      "data-orientation",
      Signal::derive(move || match context.orientation.get() {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
      })
      .into_attribute(),
    ),
  ]);

  // node_ref.on_load(move |node| {
  //   _ = node
  //     .style(
  //       orientation.start_edge.get().to_string().to_lowercase(),
  //       format!("{}%", offset_start.get()),
  //     )
  //     .style(
  //       orientation.end_edge.get().to_string().to_lowercase(),
  //       format!("{}%", offset_end.get()),
  //     );
  // });

  view! {
    <Primitive
      element=html::span
      attrs=merged_attrs
      node_ref=Some(node_ref)
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn SliderThumb(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let get_items = use_collection_context::<SliderCollectionItem, AnyElement>();
  let (index, set_index) = create_signal::<Option<usize>>(None);

  let context = use_context::<SliderContextValue>()
    .expect("SliderThumb must be used in a SliderRoot component");
  let orientation = use_context::<OrientationContextValue>()
    .expect("SliderThumb must be used in a SliderRoot component");

  let size = use_element_size(node_ref);

  let value = Signal::derive(move || Some(*context.values.get().get(index.get()?)?));

  let percent = Signal::derive(move || {
    value
      .get()
      .map(|value| convert_value_to_percentage(value, context.min.get(), context.max.get()))
      .unwrap_or(0.)
  });

  let label = Signal::derive(move || {
    index
      .get()
      .map(|index| get_label(index, context.values.get().len()))
      .flatten()
  });

  let orientation_size = Signal::derive(move || match orientation.size.get() {
    Size::Width => size.width.get(),
    Size::Height => size.height.get(),
  });

  let thumbs_in_bound_offset = Signal::derive(move || {
    get_thumb_in_bounds_offset(
      orientation_size.get(),
      percent.get(),
      match orientation.direction.get() {
        OrientationDirection::Backward => -1.0f64,
        OrientationDirection::Forward => 1.0f64,
      },
    )
  });

  // node_ref.on_load(move |node| {
  //   let Some(index) = get_items().iter().position(|item| {
  //     let Some(item) = item.0.get() else {
  //       return false;
  //     };

  //     let item_el: &web_sys::Element = &item;
  //     let node_el: &web_sys::Element = &node;

  //     item_el == node_el
  //   }) else {
  //     return;
  //   };

  //   set_index(Some(index));

  //   // if value.get().is_none() {
  //   //   _ = node.style("display", "none");
  //   // }
  // });

  create_effect(move |_| {
    let Some(node_ref) = node_ref.get() else {
      return;
    };

    context.thumbs.update_value(|thumbs| {
      thumbs.push(node_ref.clone());
    });

    on_cleanup(move || {
      context.thumbs.update_value(|thumbs| {
        if let Some(position) = thumbs.iter().position(|thumb| {
          let thumb_el: &web_sys::Element = thumb;
          let node_el: &web_sys::Element = &node_ref.clone();

          thumb_el == node_el
        }) {
          _ = thumbs.remove(position);
        }
      });
    });
  });

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      ("role", "slider".into_attribute()),
      (
        "aria-label",
        attrs
          .iter()
          .find(|(name, _)| name.eq(&"aria-label"))
          .map_or(label.get(), |(_, attr)| {
            attr.as_nameless_value_string().map(|attr| attr.to_string())
          })
          .into_attribute(),
      ),
      (
        "aria-valuemin",
        Signal::derive(move || context.min.get()).into_attribute(),
      ),
      (
        "aria-valuenow",
        Signal::derive(move || value.get()).into_attribute(),
      ),
      (
        "aria-valuemax",
        Signal::derive(move || context.max.get()).into_attribute(),
      ),
      (
        "aria-orientation",
        Signal::derive(move || match context.orientation.get() {
          Orientation::Horizontal => "horizontal",
          Orientation::Vertical => "vertical",
        })
        .into_attribute(),
      ),
      (
        "data-orientation",
        Signal::derive(move || match context.orientation.get() {
          Orientation::Horizontal => "horizontal",
          Orientation::Vertical => "vertical",
        })
        .into_attribute(),
      ),
      (
        "data-disabled",
        Signal::derive(move || context.disabled.get().then_some("")).into_attribute(),
      ),
    ]
    .into_iter(),
  );

  let span_ref = NodeRef::<Span>::new();
  span_ref.on_load(move |node| {
    _ = node.style(
      orientation.start_edge.get().to_string().to_lowercase(),
      format!(
        "calc({}% + {}px)",
        percent.get(),
        thumbs_in_bound_offset.get()
      ),
    );
  });

  view! {
    <span style="position: absolute" node_ref=span_ref>
      <Primitive
        element=html::span
        attrs=merged_attrs
        // node_ref=Some(node_ref)
      >
        {children()}
      </Primitive>
    </span>
  }
}

#[component]
fn BubbleInput(name: Signal<Option<String>>, value: Signal<f64>) -> impl IntoView {
  let node_ref = NodeRef::<Input>::new();
  let prev_value = create_previous(Signal::derive(move || value.get()));

  Effect::new(move |_| {
    (|| {
      let input = node_ref.get()?;
      let input_el = window().get("HTMLInputElement")?;
      let input_proto = Reflect::get(&input_el, &JsString::from("prototype"))
        .ok()?
        .dyn_into::<Object>()
        .ok()?;

      let input_descriptor_set = Reflect::get(
        &Object::get_own_property_descriptor(&input_proto, &JsString::from("value")),
        &JsString::from("set"),
      )
      .ok()?
      .dyn_into::<Function>()
      .ok()?;

      if prev_value.get() != value.get() {
        let mut ev_options = EventInit::new();
        ev_options.bubbles(true);

        let ev = Event::new_with_event_init_dict("input", &ev_options).ok()?;

        _ = Reflect::apply(
          &input_descriptor_set,
          &input,
          &Array::from_iter([JsValue::from_f64(value.get())].into_iter()),
        );

        _ = input.dispatch_event(&ev);
      }

      Some(())
    })();
  });

  // node_ref.on_load(move |node| {
  //   node.set_default_value(&value.get().to_string());
  // });

  view! {
    <input
      aria-hidden
      name=Signal::derive(move || name.get()).into_attribute()
      value=Signal::derive(move || value.get()).into_attribute()
      node_ref=node_ref
      style:display="absolute"
    />
  }
}

fn get_label(index: usize, total_values: usize) -> Option<String> {
  if total_values > 2 {
    Some(format!("Value {} of {total_values}", index + 1))
  } else if total_values == 2 {
    ["Minimum", "Maximum"]
      .get(index)
      .map(|label| label.to_string())
  } else {
    None
  }
}

fn convert_value_to_percentage(value: f64, min: f64, max: f64) -> f64 {
  let max_steps = max - min;
  let percent_per_step = 100. / max_steps;
  let percentage = percent_per_step * (value - min);

  percentage.clamp(0., 100.)
}

fn get_next_sorted_values(prev_values: &Vec<f64>, next_value: f64, at_index: usize) -> Vec<f64> {
  let mut next_values = prev_values.clone();

  next_values.insert(at_index, next_value);
  next_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
  next_values
}

fn find_closest_index(values: &[f64], next_value: f64) -> Option<usize> {
  values
    .iter()
    .enumerate()
    .min_by_key(|(_, &value)| (next_value - value).abs() as i32)
    .map(|(index, _)| index)
}

fn has_min_steps_between_values(values: &[f64], min_steps_between_values: f64) -> bool {
  if min_steps_between_values <= 0. {
    return true;
  }

  values
    .windows(2)
    .map(|pair| pair[1] - pair[0])
    .fold(None, |min: Option<f64>, current| match min {
      Some(min_val) => Some(min_val.min(current)),
      None => Some(current),
    })
    .map(|steps_between_values| steps_between_values >= min_steps_between_values)
    .unwrap_or(false)
}

fn get_decimal_count(value: f64) -> usize {
  value.to_string().split('.').nth(1).unwrap_or("").len()
}

fn round_value(value: f64, decimal_count: u32) -> f64 {
  let rounder = 10_f64.powi(decimal_count as i32);
  (value * rounder).round() / rounder
}

fn get_thumb_in_bounds_offset(width: f64, left: f64, direction: f64) -> f64 {
  let half_width = width / 2.0;
  let half_percent = 50.0;
  let offset = linear_scale((0.0, half_percent), (0.0, half_width));

  (half_width - offset(left) * direction) * direction
}
