use leptos::{
  html::{AnyElement, Div},
  *,
};
use leptos_use::{use_debounce_fn, use_resize_observer};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CssStyleDeclaration, WheelEvent};

use crate::{
  components::{presence::create_presence, primitive::Primitive},
  util::{
    create_state_machine::{create_state_machine, InvalidState, MachineState},
    linear_scale, Direction, Orientation,
  },
  Attributes,
};

#[derive(Clone, Default)]
struct Scrollbar {
  size: i32,
  padding_start: i32,
  padding_end: i32,
}

#[derive(Clone, Default)]
struct Sizes {
  content: i32,
  viewport: i32,
  scrollbar: Scrollbar,
}

#[derive(Default, Clone)]
pub enum ScrollAreaKind {
  Auto,
  Always,
  Scroll,
  #[default]
  Hover,
}

#[derive(Clone)]
pub struct ScrollAreaContextValue {
  kind: MaybeSignal<ScrollAreaKind>,
  direction: Signal<Direction>,
  scroll_hide_delay: Signal<u32>,
  scroll_area: NodeRef<AnyElement>,
  viewport: NodeRef<AnyElement>,
  // on_viewport_change: Callback<Option<NodeRef<AnyElement>>>,
  content: NodeRef<Div>,
  // on_content_change: Callback<Option<NodeRef<AnyElement>>>,
  scrollbar_x: NodeRef<AnyElement>,
  // on_scrollbar_x_change: Callback<Option<NodeRef<AnyElement>>>,
  scrollbar_x_enabled: Signal<bool>,
  on_scrollbar_x_enabled_change: Callback<bool>,
  scrollbar_y: NodeRef<AnyElement>,
  // on_scrollbar_y_change: Callback<Option<NodeRef<AnyElement>>>,
  scrollbar_y_enabled: Signal<bool>,
  on_scrollbar_y_enabled_change: Callback<bool>,
  on_corner_width_change: Callback<u32>,
  on_corner_height_change: Callback<u32>,
}

#[component]
pub fn ScrollAreaRoot(
  #[prop(optional)] kind: MaybeSignal<ScrollAreaKind>,
  #[prop(optional)] direction: Option<MaybeSignal<Direction>>,
  #[prop(optional)] scroll_hide_delay: Option<MaybeSignal<u32>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let scroll_area = NodeRef::<AnyElement>::new();
  let viewport = NodeRef::<AnyElement>::new();
  let content = NodeRef::<Div>::new();
  let scrollbar_x = NodeRef::<AnyElement>::new();
  let scrollbar_y = NodeRef::<AnyElement>::new();

  let (corner_width, set_corner_width) = create_signal(0);
  let (corner_height, set_corner_height) = create_signal(0);

  let (scrollbar_x_enabled, set_scrollbar_x_enabled) = create_signal(false);
  let (scrollbar_y_enabled, set_scrollbar_y_enabled) = create_signal(false);

  let direction = Signal::derive(move || {
    direction
      .map(|direction| direction.get())
      .unwrap_or_default()
  });

  provide_context(ScrollAreaContextValue {
    kind,
    direction,
    scroll_hide_delay: Signal::derive(move || {
      scroll_hide_delay
        .map(|scroll_hide_delay| scroll_hide_delay.get())
        .unwrap_or(600)
    }),
    scroll_area,
    viewport,
    content,
    scrollbar_x,
    scrollbar_x_enabled: Signal::derive(move || scrollbar_x_enabled.get()),
    scrollbar_y,
    scrollbar_y_enabled: Signal::derive(move || scrollbar_y_enabled.get()),
    on_corner_width_change: Callback::new(move |value| {
      set_corner_width(value);
    }),
    on_corner_height_change: Callback::new(move |value| {
      set_corner_height(value);
    }),
    on_scrollbar_x_enabled_change: Callback::new(move |value| {
      set_scrollbar_x_enabled(value);
    }),
    on_scrollbar_y_enabled_change: Callback::new(move |value| {
      set_scrollbar_y_enabled(value);
    }),
  });

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [(
      "dir",
      Signal::derive(move || match direction.get() {
        Direction::LeftToRight => "ltr",
        Direction::RightToLeft => "rtl",
      })
      .into_attribute(),
    )]
    .into_iter(),
  );

  view! {
    <Primitive
      element=html::div
      attrs=merged_attrs
      node_ref=node_ref
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn ScrollAreaViewport(
  #[prop(optional)] nonce: Option<MaybeSignal<String>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaViewport must be used in a ScrollAreaRoot component");

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend(
    [
      ("data-primitive-scroll-area-viewport", "".into_attribute()),
      (
        "style",
        Signal::derive(move || {
          format!(
            "overflow-x: {}; overflow-y: {}; ",
            if context.scrollbar_x_enabled.get() {
              "scroll"
            } else {
              "hidden"
            },
            if context.scrollbar_y_enabled.get() {
              "scroll"
            } else {
              "hidden"
            }
          )
        })
        .into_attribute(),
      ),
    ]
    .into_iter(),
  );

  let content_ref = context.content;

  view! {
    <>
      <style
        inner_html="[data-primitive-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-primitive-scroll-area-viewport]::-webkit-scrollbar{display:none}"
        nonce=nonce.into_attribute()
      />
      <Primitive
        element=html::div
        attrs=attrs
      >
        <div
          node_ref=content_ref
          style="min-width: 100%; display: table"
        >
          {children()}
        </div>
      </Primitive>
    </>
  }
}

pub struct ForceMount;

#[component]
pub fn ScrollAreaScrollbar(
  #[prop(optional)] force_mount: Option<ForceMount>,
  #[prop(optional)] orientation: MaybeSignal<Orientation>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: Children,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbar must be used in a ScrollAreaRoot component");

  let ScrollAreaContextValue {
    on_scrollbar_x_enabled_change,
    on_scrollbar_y_enabled_change,
    ..
  } = context;

  match context.kind.get() {
    ScrollAreaKind::Always => view! {},
    ScrollAreaKind::Scroll => view! {},
    ScrollAreaKind::Auto => view! {},
    ScrollAreaKind::Hover => view! {},
  };

  view! {}
}

#[component]
fn ScrollAreaScrollbarHover(
  #[prop(optional)] force_mount: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarHover must be used in a ScrollAreaRoot component");

  let (visible, set_visible) = create_signal(false);

  Effect::new(move |_| {
    let Some(scroll_area) = context.scroll_area.get() else {
      return;
    };

    let timer_handle_value = StoredValue::new(0);

    let handle_pointer_enter = Closure::<dyn FnMut()>::new(move || {
      window().clear_timeout_with_handle(timer_handle_value.get_value());
      set_visible(true);
    });

    let handle_pointer_leave = Closure::<dyn FnMut()>::new(move || {
      let hide = Closure::<dyn Fn()>::new(move || {
        set_visible(false);
      });

      let Ok(timer_handle) = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        hide.as_ref().unchecked_ref(),
        context.scroll_hide_delay.get() as i32,
      ) else {
        return;
      };

      timer_handle_value.set_value(timer_handle);
    });

    _ = scroll_area.add_event_listener_with_callback(
      "pointerenter",
      handle_pointer_enter.as_ref().unchecked_ref(),
    );
    _ = scroll_area.add_event_listener_with_callback(
      "pointerleave",
      handle_pointer_leave.as_ref().unchecked_ref(),
    );

    on_cleanup(move || {
      window().clear_timeout_with_handle(timer_handle_value.get_value());

      _ = scroll_area.add_event_listener_with_callback(
        "pointerenter",
        handle_pointer_enter.as_ref().unchecked_ref(),
      );
      _ = scroll_area.add_event_listener_with_callback(
        "pointerleave",
        handle_pointer_leave.as_ref().unchecked_ref(),
      );

      handle_pointer_enter.forget();
      handle_pointer_leave.forget();
    });
  });

  let is_present = Signal::derive(move || {
    force_mount
      .map(|force_mount| force_mount.get())
      .unwrap_or(visible.get())
  });

  // let presence = create_presence(is_present);

  view! {
    {move || is_present.get().then(|| {
      let mut merged_attrs = attrs.clone();
      merged_attrs.extend(
        [(
          "data-state",
          Signal::derive(move || if visible.get() { "visible" } else { "hidden" }).into_attribute(),
        )]
        .into_iter(),
      );

      let children = children.clone();

      view!{
        <ScrollAreaScrollbarAuto
          force_mount=Signal::derive(move || force_mount.map(|force_mount| force_mount.get()).unwrap_or(false)).into()
          orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or_default()).into()
          attrs=merged_attrs
          node_ref=node_ref
        >
          {children()}
        </ScrollAreaScrollbarAuto>
        }
    })}
  }
}

#[component]
fn ScrollAreaScrollbarScroll(
  #[prop(optional)] force_mount: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarAuto must be used in a ScrollAreaRoot component");

  let is_horizontal = Signal::derive(move || {
    orientation
      .map(|orientation| orientation.get())
      .unwrap_or_default()
      == Orientation::Horizontal
  });

  let (state, send) = create_state_machine::<
    ScrollAreaScrollbarScrollState,
    ScrollAreaScrollbarScrollEvent,
  >(ScrollAreaScrollbarScrollState::Hidden.into());

  let scroll_end = use_debounce_fn(
    move || send(ScrollAreaScrollbarScrollEvent::ScrollEnd),
    100.,
  );

  Effect::new(move |_| {
    if state.get() != ScrollAreaScrollbarScrollState::Idle {
      return;
    }

    let hide = Closure::<dyn Fn()>::new(move || {
      send(ScrollAreaScrollbarScrollEvent::Hide);
    });

    let Ok(handle) = window().set_timeout_with_callback_and_timeout_and_arguments_0(
      hide.as_ref().unchecked_ref(),
      context.scroll_hide_delay.get() as i32,
    ) else {
      return;
    };

    on_cleanup(move || {
      window().clear_timeout_with_handle(handle);
      hide.forget();
    });
  });

  Effect::new(move |_| {
    let Some(viewport) = context.viewport.get() else {
      return;
    };

    let prev_scroll_position = StoredValue::new(if is_horizontal.get() {
      viewport.scroll_left()
    } else {
      viewport.scroll_top()
    });

    let handle_scroll = Closure::<dyn FnMut()>::new(move || {
      let scroll_position = if is_horizontal.get() {
        viewport.scroll_left()
      } else {
        viewport.scroll_top()
      };

      let has_scroll_in_direction_changed = prev_scroll_position.get_value() != scroll_position;

      if has_scroll_in_direction_changed {
        send(ScrollAreaScrollbarScrollEvent::Scroll);
      }
    });
  });

  let is_present = Signal::derive(move || {
    force_mount
      .map(|force_mount| force_mount.get())
      .unwrap_or(false)
      || state.get() == ScrollAreaScrollbarScrollState::Hidden
  });

  // let presence = create_presence(is_present);

  view! {
    {move || is_present.get().then(|| {
      let mut merged_attrs = attrs.clone();

      merged_attrs.extend([
        ("data-state", Signal::derive(move || if state.get() == ScrollAreaScrollbarScrollState::Hidden { "hidden" } else { "visible" }).into_attribute())
      ].into_iter());

      let children = children.clone();

      view! {
        <ScrollAreaScrollbarVisible
          attrs=merged_attrs
          on_pointer_enter=Callback::new(move |_| send(ScrollAreaScrollbarScrollEvent::PointerEnter))
          on_pointer_leave=Callback::new(move |_| send(ScrollAreaScrollbarScrollEvent::PointerLeave))
        >
          {children()}
        </ScrollAreaScrollbarVisible>
      }
    })}
  }
}

#[component]
fn ScrollAreaScrollbarAuto(
  #[prop(optional)] force_mount: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarAuto must be used in a ScrollAreaRoot component");

  let (visible, set_visible) = create_signal(false);

  let is_horizontal = Signal::derive(move || {
    orientation
      .map(|orientation| orientation.get())
      .unwrap_or_default()
      == Orientation::Horizontal
  });

  let handle_resize = use_debounce_fn(
    move || {
      let Some(viewport) = context.viewport.get() else {
        return;
      };

      set_visible(if is_horizontal.get() {
        viewport.offset_width() < viewport.scroll_width()
      } else {
        viewport.offset_height() < viewport.scroll_height()
      });
    },
    10.,
  );

  let viewport_resize = handle_resize.clone();
  use_resize_observer(context.viewport, move |_, _| {
    viewport_resize();
  });

  let content_resize = handle_resize.clone();
  use_resize_observer(context.content, move |_, _| {
    content_resize();
  });

  let is_present = Signal::derive(move || {
    force_mount
      .map(|force_mount| force_mount.get())
      .unwrap_or(false)
      || visible.get()
  });

  // let presence = create_presence(is_present);

  view! {
    {move || is_present.get().then(|| {
      let mut merged_attrs = attrs.clone();
      merged_attrs.extend([
        ("data-state", Signal::derive(move || if visible.get() { "visible" } else { "hidden" }).into_attribute())
      ].into_iter());

      let children = children.clone();

      view! {
        <ScrollAreaScrollbarVisible
          orientation=Signal::derive(move || orientation.map(|orientation| orientation.get()).unwrap_or(Orientation::Vertical)).into()
          attrs=merged_attrs
          node_ref=node_ref
        >
          {children()}
        </ScrollAreaScrollbarVisible>
      }
    })}
  }
}

#[component]
fn ScrollAreaScrollbarVisible(
  #[prop(optional)] force_mount: Option<MaybeSignal<bool>>,
  #[prop(optional)] orientation: Option<MaybeSignal<Orientation>>,
  #[prop(optional)] on_pointer_enter: Option<Callback<()>>,
  #[prop(optional)] on_pointer_leave: Option<Callback<()>>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarVisible must be used in a ScrollAreaRoot component");

  let thumb_ref = RwSignal::<Option<HtmlElement<AnyElement>>>::new(None);

  let pointer_offset = StoredValue::new(0);
  let (sizes, set_sizes) = create_signal(Sizes::default());
  let thumb_ratio = Signal::derive(move || sizes.get().viewport / sizes.get().content);

  let get_scroll_position = move |pointer_position: i32, direction: Direction| {
    get_scroll_position_from_pointer(
      pointer_position,
      pointer_offset.get_value(),
      &sizes.get(),
      direction,
    )
  };

  view! {
    {move || {
      let merged_attrs = attrs.clone();
      let children = children.clone();

      match orientation.map(|orientation| orientation.get()).unwrap_or_default() {
        Orientation::Horizontal => view! {
          <ScrollAreaScrollbarX
            attrs=merged_attrs
            node_ref=node_ref
            on_sizes_change=Callback::new(move |sizes| {
              set_sizes(sizes);
            })
            sizes=Signal::derive(move || sizes.get()).into()
            has_thumb=Signal::derive(move || thumb_ratio.get() > 0 && thumb_ratio.get() < 1).into()
            on_thumb_change=Callback::new(move |thumb| {
              thumb_ref.set(Some(thumb));
            })
            on_thumb_pointer_up=Callback::new(move |_| {
              pointer_offset.set_value(0)
            })
            on_thumb_pointer_down=Callback::new(move |pointer_position| {
              pointer_offset.set_value(pointer_position);
            })
            on_thumb_position_change=Callback::new(move |_| {
              let (Some(viewport), Some(thumb_el)) = (context.viewport.get(), thumb_ref.get()) else {
                return;
              };

              let scroll_position = viewport.scroll_left();
              let offset = get_thumb_offset_from_scroll(scroll_position, &sizes.get(), context.direction.get());

              _ = thumb_el.style("transform", format!("translate3d(0, {offset}px, 0)"));
            })
            on_wheel_scroll=Callback::new(move |scroll_position| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(scroll_position);
              }
            })
            on_drag_scroll=Callback::new(move |pointer_position| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(get_scroll_position(pointer_position, context.direction.get()) as i32);
              }
            })
          >
            {children()}
          </ScrollAreaScrollbarX>
        },
        Orientation::Vertical => view! {
          <ScrollAreaScrollbarY
            attrs=merged_attrs
            node_ref=node_ref
            on_sizes_change=Callback::new(move |sizes| {
              set_sizes(sizes);
            })
            sizes=Signal::derive(move || sizes.get()).into()
            has_thumb=Signal::derive(move || thumb_ratio.get() > 0 && thumb_ratio.get() < 1).into()
            on_thumb_change=Callback::new(move |thumb| {
              thumb_ref.set(Some(thumb));
            })
            on_thumb_pointer_up=Callback::new(move |_| {
              pointer_offset.set_value(0)
            })
            on_thumb_pointer_down=Callback::new(move |pointer_position| {
              pointer_offset.set_value(pointer_position);
            })
            on_thumb_position_change=Callback::new(move |_| {
              let (Some(viewport), Some(thumb_el)) = (context.viewport.get(), thumb_ref.get()) else {
                return;
              };

              let scroll_position = viewport.scroll_left();
              let offset = get_thumb_offset_from_scroll(scroll_position, &sizes.get(), context.direction.get());

              _ = thumb_el.style("transform", format!("translate3d(0, {offset}px, 0)"));
            })
            on_wheel_scroll=Callback::new(move |scroll_position| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(scroll_position);
              }
            })
            on_drag_scroll=Callback::new(move |pointer_position| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(get_scroll_position(pointer_position, context.direction.get()) as i32);
              }
            })
          >
            {children()}
          </ScrollAreaScrollbarY>
        }
      }
    }}
  }
}

type NodeSignal = RwSignal<Option<HtmlElement<AnyElement>>>;

#[component]
fn ScrollAreaScrollbarX(
  sizes: MaybeSignal<Sizes>,
  has_thumb: MaybeSignal<bool>,
  on_sizes_change: Callback<Sizes>,
  on_thumb_change: Callback<HtmlElement<AnyElement>>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<i32>,
  on_thumb_position_change: Callback<()>,
  on_wheel_scroll: Callback<i32>,
  on_drag_scroll: Callback<i32>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarX must be used in a ScrollAreaRoot component");

  let (computed_style, set_computed_style) = create_signal::<Option<CssStyleDeclaration>>(None);

  let node_ref = NodeSignal::new(None);

  Effect::new(move |_| {
    if let Some(node_el) = node_ref.get() {
      if let Ok(computed_style) = window().get_computed_style(&node_el) {
        set_computed_style(computed_style);
      }
    }
  });

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend([("data-state", "horizontal".into_attribute())].into_iter());

  view! {
    <ScrollAreaScrollbarImpl
      sizes=Signal::derive(move || sizes.get())

      on_thumb_pointer_down=Callback::new(move |Pointer{x, ..}| {
        on_thumb_pointer_down(x);
      })
      on_drag_scroll=Callback::new(move |Pointer { y, .. }| {
        on_drag_scroll(y);
      })
      on_wheel_scroll=Callback::new(move |(event, max_scroll_pos): (WheelEvent, i32)| {
        let Some(viewport) = context.viewport.get() else {
          return;
        };

        let scroll_pos = viewport.scroll_top() + event.delta_y() as i32;
        on_wheel_scroll(scroll_pos);

        if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
          event.prevent_default();
        }
      })
      on_resize=Callback::new(move |_| {
        let (Some(node_el), Some(viewport), Some(computed_style)) = (node_ref.get(), context.viewport.get(), computed_style.get()) else {
          return;
        };

        on_sizes_change(Sizes {
          content: viewport.scroll_height(),
          viewport: viewport.offset_height(),
          scrollbar: Scrollbar {
            size: node_el.client_height(),
            padding_start: computed_style
              .get_property_value("padding-top")
              .expect("no padding top")
              .parse::<i32>()
              .unwrap(),
            padding_end: computed_style
              .get_property_value("padding-bottom")
              .expect("no padding bottom")
              .parse::<i32>()
              .unwrap(),
          }
        });
      })

      attrs=merged_attrs
      node_ref=node_ref
    >
      {children()}
    </ScrollAreaScrollbarImpl>
  }
}

#[component]
fn ScrollAreaScrollbarY(
  sizes: MaybeSignal<Sizes>,
  has_thumb: MaybeSignal<bool>,
  on_sizes_change: Callback<Sizes>,
  on_thumb_change: Callback<HtmlElement<AnyElement>>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<i32>,
  on_thumb_position_change: Callback<()>,
  on_wheel_scroll: Callback<i32>,
  on_drag_scroll: Callback<i32>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
  children: ChildrenFn,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarY must be used in a ScrollAreaRoot component");

  let (computed_style, set_computed_style) = create_signal::<Option<CssStyleDeclaration>>(None);

  let node_ref = NodeSignal::new(None);

  Effect::new(move |_| {
    if let Some(node_el) = node_ref.get() {
      if let Ok(computed_style) = window().get_computed_style(&node_el) {
        set_computed_style(computed_style);
      }
    }
  });

  let mut merged_attrs = attrs.clone();
  merged_attrs.extend([("data-state", "vertical".into_attribute())].into_iter());

  view! {
    <ScrollAreaScrollbarImpl
      sizes=Signal::derive(move || sizes.get())

      on_thumb_pointer_down=Callback::new(move |Pointer{x, ..}| {
        on_thumb_pointer_down(x);
      })
      on_drag_scroll=Callback::new(move |Pointer { y, .. }| {
        on_drag_scroll(y);
      })
      on_wheel_scroll=Callback::new(move |(event, max_scroll_pos): (WheelEvent, i32)| {
        let Some(viewport) = context.viewport.get() else {
          return;
        };

        let scroll_pos = viewport.scroll_top() + event.delta_y() as i32;
        on_wheel_scroll(scroll_pos);

        if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
          event.prevent_default();
        }
      })
      on_resize=Callback::new(move |_| {
        let (Some(node_el), Some(viewport), Some(computed_style)) = (node_ref.get(), context.viewport.get(), computed_style.get()) else {
          return;
        };

        on_sizes_change(Sizes {
          content: viewport.scroll_height(),
          viewport: viewport.offset_height(),
          scrollbar: Scrollbar {
            size: node_el.client_height(),
            padding_start: computed_style
              .get_property_value("padding-top")
              .expect("no padding top")
              .parse::<i32>()
              .unwrap(),
            padding_end: computed_style
              .get_property_value("padding-bottom")
              .expect("no padding bottom")
              .parse::<i32>()
              .unwrap(),
          }
        });
      })

      attrs=merged_attrs
      node_ref=node_ref
    >
      {children()}
    </ScrollAreaScrollbarImpl>
  }
}

fn is_scrolling_within_scrollbar_bounds(scroll_pos: i32, max_scroll_pos: i32) -> bool {
  scroll_pos > 0 && scroll_pos < max_scroll_pos
}

#[derive(Clone)]
struct ScrollbarContextValue {
  has_thumb: Signal<bool>,
  scrollbar: RwSignal<Option<HtmlElement<AnyElement>>>,
  on_thumb_change: Callback<RwSignal<Option<HtmlElement<AnyElement>>>>,
}

struct Pointer {
  x: i32,
  y: i32,
}

#[component]
pub fn ScrollAreaScrollbarImpl(
  sizes: Signal<Sizes>,

  on_thumb_pointer_down: Callback<Pointer>,
  on_drag_scroll: Callback<Pointer>,
  on_wheel_scroll: Callback<(WheelEvent, i32)>,
  on_resize: Callback<()>,

  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeSignal,
  children: ChildrenFn,
) -> impl IntoView {
  view! {}
}

#[component]
pub fn ScrollAreaThumb(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  view! {}
}

#[component]
pub fn ScrollAreaCorner(
  #[prop(attrs)] attrs: Attributes,
  #[prop(optional)] node_ref: NodeRef<AnyElement>,
) -> impl IntoView {
  view! {}
}

fn get_thumb_size(sizes: &Sizes) -> i32 {
  let ratio = sizes.viewport / sizes.content;
  let scrollbar_padding = sizes.scrollbar.padding_start - sizes.scrollbar.padding_end;
  let thumb_size = (sizes.scrollbar.size - scrollbar_padding) * ratio;

  thumb_size.max(18)
}

fn get_scroll_position_from_pointer(
  pointer_position: i32,
  pointer_offset: i32,
  sizes: &Sizes,
  direction: Direction,
) -> f64 {
  let thumb_size_px = get_thumb_size(sizes);
  let offset = if pointer_offset == 0 {
    thumb_size_px / 2
  } else {
    pointer_offset
  };
  let thumb_offset_from_end = thumb_size_px - offset;
  let min_pointer_pos = sizes.scrollbar.padding_start + offset;
  let max_pointer_pos = sizes.scrollbar.size - sizes.scrollbar.padding_end - thumb_offset_from_end;
  let max_scroll_pos = sizes.content - sizes.viewport;
  let scroll_range = if direction == Direction::LeftToRight {
    (0., max_scroll_pos as f64)
  } else {
    ((max_scroll_pos * -1) as f64, 0.)
  };
  let interpolate = linear_scale(
    (min_pointer_pos as f64, max_pointer_pos as f64),
    scroll_range,
  );

  interpolate(pointer_position as f64)
}

fn get_thumb_offset_from_scroll(scroll_position: i32, sizes: &Sizes, direction: Direction) -> f64 {
  let thumb_size_px = get_thumb_size(sizes);
  let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
  let scrollbar = sizes.scrollbar.size - scrollbar_padding;
  let max_scroll_pos = sizes.content - sizes.viewport;
  let max_thumb_pos = scrollbar - thumb_size_px;
  let scroll_clamp_range = if direction == Direction::LeftToRight {
    (0., max_scroll_pos as f64)
  } else {
    ((max_scroll_pos * -1) as f64, 0.)
  };

  let interpolate = linear_scale((0., max_scroll_pos as f64), (0., max_thumb_pos as f64));

  interpolate((scroll_position as f64).clamp(scroll_clamp_range.0, scroll_clamp_range.1))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScrollAreaScrollbarScrollState {
  Hidden,
  Scrolling,
  Interacting,
  Idle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScrollAreaScrollbarScrollEvent {
  Scroll,
  ScrollEnd,
  PointerEnter,
  PointerLeave,
  Hide,
}

impl MachineState<ScrollAreaScrollbarScrollState, ScrollAreaScrollbarScrollEvent>
  for ScrollAreaScrollbarScrollState
{
  fn send(&self, event: ScrollAreaScrollbarScrollEvent) -> Result<Self, InvalidState> {
    match (self, event) {
      (Self::Hidden, ScrollAreaScrollbarScrollEvent::Scroll) => Ok(Self::Scrolling),
      (Self::Scrolling, ScrollAreaScrollbarScrollEvent::ScrollEnd) => Ok(Self::Idle),
      (Self::Scrolling, ScrollAreaScrollbarScrollEvent::PointerEnter) => Ok(Self::Interacting),
      (Self::Interacting, ScrollAreaScrollbarScrollEvent::Scroll) => Ok(Self::Interacting),
      (Self::Interacting, ScrollAreaScrollbarScrollEvent::PointerLeave) => Ok(Self::Idle),
      (Self::Idle, ScrollAreaScrollbarScrollEvent::Hide) => Ok(Self::Hidden),
      (Self::Idle, ScrollAreaScrollbarScrollEvent::Scroll) => Ok(Self::Scrolling),
      (Self::Idle, ScrollAreaScrollbarScrollEvent::PointerEnter) => Ok(Self::Interacting),
      _ => Err(InvalidState),
    }
  }
}
