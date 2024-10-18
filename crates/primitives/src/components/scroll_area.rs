use std::time::Duration;

use leptos::{
  either::{Either, EitherOf4},
  ev::{pointerdown, pointerenter, pointerleave, pointermove, pointerup, scroll, wheel},
  html::{self, Div},
  leptos_dom::helpers::TimeoutHandle,
  prelude::*,
};
use leptos_use::{
  use_debounce_fn, use_document, use_event_listener, use_event_listener_with_options, use_raf_fn,
  use_resize_observer, utils::Pausable, UseEventListenerOptions,
};
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, DomRect, HtmlDivElement, PointerEvent, WheelEvent};

use crate::{
  presence::create_presence,
  primitive::Primitive,
  util::{
    create_state_machine::{create_state_machine, InvalidState, MachineState},
    linear_scale,
  },
  Direction, Orientation,
};

#[derive(Clone, Default)]
struct Scrollbar {
  size: f64,
  padding_start: f64,
  padding_end: f64,
}

#[derive(Clone, Default)]
struct Sizes {
  content: f64,
  viewport: f64,
  scrollbar: Scrollbar,
}

#[derive(Default, Clone, PartialEq)]
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
  scroll_hide_delay: Signal<u64>,
  scroll_area: NodeRef<Div>,
  viewport: NodeRef<Div>,
  // on_viewport_change: Callback<NodeRef<Div>>,
  content: NodeRef<Div>,
  // on_content_change: Callback<NodeRef<Div>>,
  scrollbar_x: NodeRef<Div>,
  // on_scrollbar_x_change: Callback<Option<NodeRef<Div>>>,
  scrollbar_x_enabled: Signal<bool>,
  on_scrollbar_x_enabled_change: Callback<bool>,
  scrollbar_y: NodeRef<Div>,
  // on_scrollbar_y_change: Callback<Option<NodeRef<Div>>>,
  scrollbar_y_enabled: Signal<bool>,
  on_scrollbar_y_enabled_change: Callback<bool>,
  on_corner_width_change: Callback<u32>,
  on_corner_height_change: Callback<u32>,
}

#[component]
pub fn ScrollAreaRoot(
  #[prop(optional, into)] kind: MaybeSignal<ScrollAreaKind>,
  #[prop(optional, into)] direction: MaybeSignal<Direction>,
  #[prop(default=600.into(), into)] scroll_hide_delay: MaybeSignal<u64>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let viewport = NodeRef::<Div>::new();
  let content = NodeRef::<Div>::new();
  let scrollbar_x = NodeRef::<Div>::new();
  let scrollbar_y = NodeRef::<Div>::new();

  let (corner_width, set_corner_width) = signal(0);
  let (corner_height, set_corner_height) = signal(0);

  let (scrollbar_x_enabled, set_scrollbar_x_enabled) = signal(false);
  let (scrollbar_y_enabled, set_scrollbar_y_enabled) = signal(false);

  let direction = Signal::derive(move || direction.get());

  provide_context(ScrollAreaContextValue {
    kind,
    direction,
    scroll_hide_delay: Signal::derive(move || scroll_hide_delay.get()),
    scroll_area: node_ref,
    viewport,
    content,
    scrollbar_x,
    scrollbar_x_enabled: Signal::derive(move || scrollbar_x_enabled.get()),
    scrollbar_y,
    scrollbar_y_enabled: Signal::derive(move || scrollbar_y_enabled.get()),
    on_corner_width_change: Callback::new(move |value| {
      set_corner_width.set(value);
    }),
    on_corner_height_change: Callback::new(move |value| {
      set_corner_height.set(value);
    }),
    on_scrollbar_x_enabled_change: Callback::new(move |value| {
      set_scrollbar_x_enabled.set(value);
    }),
    on_scrollbar_y_enabled_change: Callback::new(move |value| {
      set_scrollbar_y_enabled.set(value);
    }),
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    node.style(("position", "relative"));
    node.style((
      "--primitive-scroll-area-corner-width",
      format!("{}px", corner_width.get()),
    ));
    node.style((
      "--primitive-scroll-area-corner-height",
      format!("{}px", corner_height.get()),
    ));
  });

  view! {
    <Primitive
      element={html::div}
      node_ref={node_ref}
      as_child={as_child}
      {..}
      dir=move || direction.get().to_string()
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn ScrollAreaViewport(
  #[prop(optional, into)] nonce: MaybeProp<String>,

  //#[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaViewport must be used in a ScrollAreaRoot component");

  let content_ref = context.content;

  // Effect::new(move |_| {
  //   let Some(node) = node_ref.get() else {
  //     return;
  //   };

  //   (context.on_viewport_change)(node);
  // });

  // Effect::new(move |_| {
  //   let Some(content) = context.content.get() else {
  //     return;
  //   };

  //   (context.on_content_change)(content);
  // });

  Effect::new(move |_| {
    let Some(viewport) = context.viewport.get() else {
      return;
    };

    viewport.style((
      "overflow-x",
      if context.scrollbar_x_enabled.get() {
        "scroll"
      } else {
        "hidden"
      },
    ));
    viewport.style((
      "overflow-y",
      if context.scrollbar_y_enabled.get() {
        "scroll"
      } else {
        "hidden"
      },
    ));
  });

  view! {
    <>
      <style nonce=Signal::derive(move || nonce.get())>
        r"[data-leptix-scroll-area-viewport] {
            scrollbar-width:none;
            -ms-overflow-style:none;
            -webkit-overflow-scrolling:touch;
        }

        [data-leptix-scroll-area-viewport]::-webkit-scrollbar{
            display:none
        }"
      </style>

      <Primitive
        element={html::div}
        node_ref={context.viewport}
        as_child={as_child}
        attr:data-leptix-scroll-area-viewport=""
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

#[component]
pub fn ScrollAreaScrollbar(
  #[prop(optional, into)] force_mount: MaybeSignal<bool>,
  #[prop(optional, into)] orientation: MaybeSignal<Orientation>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbar must be used in a ScrollAreaRoot component");

  let ScrollAreaContextValue {
    on_scrollbar_x_enabled_change,
    on_scrollbar_y_enabled_change,
    ..
  } = context;

  Effect::new(move |_| {
    if orientation.get() == Orientation::Horizontal {
      on_scrollbar_x_enabled_change.run(true);
    } else {
      on_scrollbar_y_enabled_change.run(true);
    }
  });

  on_cleanup(move || {
    if orientation.get() == Orientation::Horizontal {
      on_scrollbar_x_enabled_change.run(false);
    } else {
      on_scrollbar_y_enabled_change.run(false);
    }
  });

  match context.kind.get() {
    ScrollAreaKind::Hover => EitherOf4::A(view! {
      <ScrollAreaScrollbarHover
        force_mount=force_mount
        orientation=orientation
        node_ref=node_ref
        as_child=as_child
      >
        {children()}
      </ScrollAreaScrollbarHover>
    }),
    ScrollAreaKind::Scroll => EitherOf4::B(view! {
      <ScrollAreaScrollbarScroll
        force_mount=force_mount
        orientation=orientation
        node_ref=node_ref
        as_child=as_child
      >
        {children()}
      </ScrollAreaScrollbarScroll>
    }),
    ScrollAreaKind::Auto => EitherOf4::C(view! {
      <ScrollAreaScrollbarAuto
        force_mount=force_mount
        orientation=orientation
        node_ref=node_ref
        as_child=as_child
      >
        {children()}
      </ScrollAreaScrollbarAuto>
    }),
    ScrollAreaKind::Always => EitherOf4::D(view! {
      <ScrollAreaScrollbarVisible
        orientation=orientation
        node_ref=node_ref
        as_child=as_child
      >
        {children()}
      </ScrollAreaScrollbarVisible>
    }),
  }
}

#[component]
fn ScrollAreaScrollbarHover(
  force_mount: MaybeSignal<bool>,
  orientation: MaybeSignal<Orientation>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarHover must be used in a ScrollAreaRoot component");

  let (visible, set_visible) = signal(false);

  Effect::new(move |_| {
    let timer_handle_value = StoredValue::<Option<TimeoutHandle>>::new(None);

    let remove_pointer_enter = use_event_listener(context.scroll_area, pointerenter, move |_| {
      if let Some(timeout_handle) = timer_handle_value.get_value() {
        timeout_handle.clear();
        timer_handle_value.set_value(None);
      }

      set_visible.set(true);
    });

    let remove_pointer_leave = use_event_listener(context.scroll_area, pointerleave, move |_| {
      let Ok(timer_handle) = set_timeout_with_handle(
        move || {
          set_visible.set(false);
        },
        Duration::from_millis(context.scroll_hide_delay.get().into()),
      ) else {
        return;
      };

      timer_handle_value.set_value(Some(timer_handle));
    });

    Owner::current().map(|owner| {
      owner.with_cleanup(move || {
        if let Some(timeout_handle) = timer_handle_value.get_value() {
          timeout_handle.clear();
          timer_handle_value.set_value(None);
        }

        remove_pointer_enter();
        remove_pointer_leave();
      })
    });
  });

  let is_present = Signal::derive(move || force_mount.get() || visible.get());

  let presence = create_presence(is_present, node_ref);

  let children = StoredValue::new(children);

  view! {
    <Show when=move || presence.get()>
      <ScrollAreaScrollbarAuto
        force_mount={force_mount}
        orientation={orientation}
        node_ref={node_ref}
        as_child={as_child}
        attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
      >
        {children.with_value(|children| children())}
      </ScrollAreaScrollbarAuto>
    </Show>
  }
}

#[component]
fn ScrollAreaScrollbarScroll(
  force_mount: MaybeSignal<bool>,
  orientation: MaybeSignal<Orientation>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarAuto must be used in a ScrollAreaRoot component");

  let is_horizontal = Signal::derive(move || orientation.get() == Orientation::Horizontal);

  let (state, send) = create_state_machine::<
    ScrollAreaScrollbarScrollState,
    ScrollAreaScrollbarScrollEvent,
  >(ScrollAreaScrollbarScrollState::Hidden.into());

  let scroll_end = use_debounce_fn(
    move || send.run(ScrollAreaScrollbarScrollEvent::ScrollEnd),
    100.,
  );

  Effect::new(move |_| {
    if state.get() != ScrollAreaScrollbarScrollState::Idle {
      return;
    }

    let Ok(handle) = set_timeout_with_handle(
      move || {
        send.run(ScrollAreaScrollbarScrollEvent::Hide);
      },
      Duration::from_millis(context.scroll_hide_delay.get().into()),
    ) else {
      return;
    };

    on_cleanup(move || {
      handle.clear();
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

    let viewport_scroll_end = scroll_end.clone();
    _ = use_event_listener(context.viewport, scroll, move |_| {
      let scroll_position = if is_horizontal.get() {
        viewport.scroll_left()
      } else {
        viewport.scroll_top()
      };

      let has_scroll_in_direction_changed = prev_scroll_position.get_value() != scroll_position;

      if has_scroll_in_direction_changed {
        send.run(ScrollAreaScrollbarScrollEvent::Scroll);
        viewport_scroll_end();
      }
    });
  });

  let is_present = Signal::derive(move || {
    force_mount.get() || state.get() == ScrollAreaScrollbarScrollState::Hidden
  });

  let presence = create_presence(is_present, node_ref);

  let children = StoredValue::new(children);

  view! {
    <Show when=move || presence.get()>
        <ScrollAreaScrollbarVisible
          orientation={orientation}
          on_pointer_enter=Callback::new(move |_| send.run(ScrollAreaScrollbarScrollEvent::PointerEnter))
          on_pointer_leave=Callback::new(move |_| send.run(ScrollAreaScrollbarScrollEvent::PointerLeave))
          node_ref={node_ref}
          as_child={as_child}
          attr:data-state=move || {
            if state.get() == ScrollAreaScrollbarScrollState::Hidden {
              "hidden"
            } else {
              "visible"
            }
          }
        >
            {children.with_value(|children| children())}
        </ScrollAreaScrollbarVisible>
    </Show>
  }
}

#[component]
fn ScrollAreaScrollbarAuto(
  force_mount: MaybeSignal<bool>,
  orientation: MaybeSignal<Orientation>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarAuto must be used in a ScrollAreaRoot component");

  let (visible, set_visible) = signal(false);

  let is_horizontal = move || orientation.get() == Orientation::Horizontal;

  let handle_resize = use_debounce_fn(
    move || {
      let Some(viewport) = context.viewport.get() else {
        return;
      };

      set_visible.set(if is_horizontal() {
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

  let is_present = Signal::derive(move || force_mount.get() || visible.get());

  let presence = create_presence(is_present, node_ref);

  let children = StoredValue::new(children);

  view! {
    <Show when=move || presence.get()>
        <ScrollAreaScrollbarVisible
          orientation={orientation}
          node_ref={node_ref}
          as_child={as_child}
          attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
        >
            {children.with_value(|children| children())}
        </ScrollAreaScrollbarVisible>
    </Show>
  }
}

#[component]
fn ScrollAreaScrollbarVisible(
  orientation: MaybeSignal<Orientation>,

  #[prop(default=Callback::new(|_|{}), into)] on_pointer_enter: Callback<()>,
  #[prop(default=Callback::new(|_|{}), into)] on_pointer_leave: Callback<()>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarVisible must be used in a ScrollAreaRoot component");

  let thumb_ref = RwSignal::new_local(None);

  let pointer_offset = StoredValue::new(0.0f64);
  let (sizes, set_sizes) = signal(Sizes::default());
  let thumb_ratio = Signal::derive(move || sizes.get().viewport / sizes.get().content);

  let get_scroll_position = move |pointer_position: f64, direction: Direction| {
    get_scroll_position_from_pointer(
      pointer_position,
      pointer_offset.get_value(),
      &sizes.get(),
      direction,
    )
  };

  view! {
    {move || {
      let children = children.clone();

      match orientation.get() {
        Orientation::Horizontal => Either::Left(view! {
          <ScrollAreaScrollbarX
            on_sizes_change=Callback::new(move |sizes| {
              set_sizes.set(sizes);
            })
            on_pointer_enter=on_pointer_enter
            on_pointer_leave=on_pointer_leave
            sizes=Signal::derive(move || sizes.get()).into()
            has_thumb=Signal::derive(move || thumb_ratio.get() > 0. && thumb_ratio.get() < 1.).into()
            on_thumb_change=Callback::new(move |thumb| {
              thumb_ref.set(Some(thumb));
            })
            on_thumb_pointer_up=Callback::new(move |_| {
              pointer_offset.set_value(0.)
            })
            on_thumb_pointer_down=Callback::new(move |pointer_position: f64| {
              pointer_offset.set_value(pointer_position);
            })
            on_thumb_position_change=Callback::new(move |_| {
              let (Some(viewport), Some(thumb_el)) = (context.viewport.get(), thumb_ref.get()) else {
                return;
              };

              let scroll_position = viewport.scroll_left() as f64;
              let offset = get_thumb_offset_from_scroll(scroll_position, &sizes.get(), context.direction.get());

              _ = thumb_el.style(("transform", format!("translate3d({offset}px, 0, 0)")));
            })
            on_wheel_scroll=Callback::new(move |scroll_position: f64| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(scroll_position as i32);
              }
            })
            on_drag_scroll=Callback::new(move |pointer_position: f64| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(get_scroll_position(pointer_position, context.direction.get()) as i32);
              }
            })
            node_ref=node_ref
            as_child=as_child
          >
            {children()}
          </ScrollAreaScrollbarX>
        }),
        Orientation::Vertical => Either::Right(view! {
          <ScrollAreaScrollbarY
            on_sizes_change=Callback::new(move |sizes| {
              set_sizes.set(sizes);
            })
            on_pointer_enter=on_pointer_enter
            on_pointer_leave=on_pointer_leave
            sizes=Signal::derive(move || sizes.get()).into()
            has_thumb=Signal::derive(move || thumb_ratio.get() > 0. && thumb_ratio.get() < 1.).into()
            on_thumb_change=Callback::new(move |thumb| {
              thumb_ref.set(Some(thumb));
            })
            on_thumb_pointer_up=Callback::new(move |_| {
              pointer_offset.set_value(0.)
            })
            on_thumb_pointer_down=Callback::new(move |pointer_position| {
              pointer_offset.set_value(pointer_position);
            })
            on_thumb_position_change=Callback::new(move |_| {
              let (Some(viewport), Some(thumb_el)) = (context.viewport.get(), thumb_ref.get()) else {
                return;
              };

              let scroll_position = viewport.scroll_top() as f64;
              let offset = get_thumb_offset_from_scroll(scroll_position, &sizes.get(), context.direction.get());

              _ = thumb_el.style(("transform", format!("translate3d(0, {offset}px, 0)")));
            })
            on_wheel_scroll=Callback::new(move |scroll_position: f64| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(scroll_position as i32);
              }
            })
            on_drag_scroll=Callback::new(move |pointer_position| {
              if let Some(viewport) = context.viewport.get() {
                viewport.set_scroll_top(get_scroll_position(pointer_position, context.direction.get()) as i32);
              }
            })
            node_ref=node_ref
            as_child=as_child
          >
            {children()}
          </ScrollAreaScrollbarY>
        })
      }
    }}
  }
}

#[component]
fn ScrollAreaScrollbarX(
  sizes: MaybeSignal<Sizes>,
  has_thumb: MaybeSignal<bool>,

  on_sizes_change: Callback<Sizes>,
  on_thumb_change: Callback<HtmlDivElement>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<f64>,
  on_thumb_position_change: Callback<()>,
  on_wheel_scroll: Callback<f64>,
  on_drag_scroll: Callback<f64>,
  #[prop(default=Callback::new(|_:()|{}))] on_pointer_enter: Callback<()>,
  #[prop(default=Callback::new(|_:()|{}))] on_pointer_leave: Callback<()>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarX must be used in a ScrollAreaRoot component");

  let (computed_style, set_computed_style) = signal_local::<Option<CssStyleDeclaration>>(None);

  let effect_sizes = sizes.clone();
  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    if let Ok(computed_style) = window().get_computed_style(&node) {
      set_computed_style.set(computed_style);
    }

    _ = node.style((
      "right",
      if context.direction.get() == Direction::LeftToRight {
        "var(--primitive-scroll-area-corner-width)"
      } else {
        "0"
      },
    ));
    _ = node.style((
      "left",
      if context.direction.get() == Direction::RightToLeft {
        "var(--primitive-scroll-area-corner-width)"
      } else {
        "0"
      },
    ));
    _ = node.style(("bottom", "0"));
    _ = node.style((
      "--primitive-scroll-area-thumb-width",
      format!("{}px", get_thumb_size(&effect_sizes.get()).trunc()),
    ));
  });

  view! {
    <ScrollAreaScrollbarImpl
      sizes=Signal::derive(move || sizes.get())
      has_thumb=Signal::derive(move || has_thumb.get())
      on_pointer_enter={on_pointer_enter}
      on_pointer_leave={on_pointer_leave}
      on_thumb_pointer_up={on_thumb_pointer_up}
      on_thumb_change={on_thumb_change}
      on_thumb_pointer_down=Callback::new(move |Pointer{x, ..}| {
        on_thumb_pointer_down.run(x);
      })
      on_thumb_position_change={on_thumb_position_change}
      on_drag_scroll=Callback::new(move |Pointer { y, .. }| {
        on_drag_scroll.run(y);
      })
      on_wheel_scroll=Callback::new(move |(event, max_scroll_pos): (WheelEvent, f64)| {
        let Some(viewport) = context.viewport.get() else {
          return;
        };

        let scroll_pos = viewport.scroll_top() as f64 + event.delta_y();
        on_wheel_scroll.run(scroll_pos);

        if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
          event.prevent_default();
        }
      })
      on_resize=Callback::new(move |_| {
        let (Some(node_el), Some(viewport), Some(computed_style)) = (node_ref.get(), context.viewport.get(), computed_style.get()) else {
          return;
        };

        on_sizes_change.run(Sizes {
          content: viewport.scroll_height() as f64,
          viewport: viewport.offset_height() as f64,
          scrollbar: Scrollbar {
            size: node_el.client_height() as f64,
            padding_start: computed_style
              .get_property_value("padding-top")
              .expect("no padding top")
              .parse::<f64>()
              .unwrap(),
            padding_end: computed_style
              .get_property_value("padding-bottom")
              .expect("no padding bottom")
              .parse::<f64>()
              .unwrap(),
          }
        });
      })
      node_ref={node_ref}
      as_child={as_child}
      attr:data-orientation="horizontal"
    >
      {children()}
    </ScrollAreaScrollbarImpl>
  }
}

#[component]
fn ScrollAreaScrollbarY(
  #[prop(default=Callback::new(|_|{}), into)] on_pointer_enter: Callback<()>,
  #[prop(default=Callback::new(|_|{}), into)] on_pointer_leave: Callback<()>,

  sizes: MaybeSignal<Sizes>,
  has_thumb: MaybeSignal<bool>,
  on_sizes_change: Callback<Sizes>,
  on_thumb_change: Callback<HtmlDivElement>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<f64>,
  on_thumb_position_change: Callback<()>,
  on_wheel_scroll: Callback<f64>,
  on_drag_scroll: Callback<f64>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarY must be used in a ScrollAreaRoot component");

  let (computed_style, set_computed_style) = signal_local::<Option<CssStyleDeclaration>>(None);

  let effect_sizes = sizes.clone();
  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    if let Ok(computed_style) = window().get_computed_style(&node) {
      set_computed_style.set(computed_style);
    }

    _ = node.style(("top", "0"));

    if let Some(direction) =
      (context.direction.get() == Direction::LeftToRight).then_some(0.to_string())
    {
      node.style(("right", direction));
    } else {
      node.style("right");
    }

    if let Some(direction) =
      (context.direction.get() == Direction::RightToLeft).then_some(0.to_string())
    {
      node.style(("left", direction));
    } else {
      node.style("left");
    }

    node.style(("bottom", "var(--primitive-scroll-area-corner-height)"));
    node.style((
      "--primitive-scroll-area-thumb-height",
      format!("{}px", get_thumb_size(&effect_sizes.get()).trunc()),
    ));
  });

  view! {
    <ScrollAreaScrollbarImpl
      sizes=Signal::derive(move || sizes.get())
      has_thumb=Signal::derive(move || has_thumb.get())
      on_pointer_enter={on_pointer_enter}
      on_pointer_leave={on_pointer_leave}
      on_thumb_pointer_up={on_thumb_pointer_up}
      on_thumb_change={on_thumb_change}
      on_thumb_pointer_down=Callback::new(move |Pointer{x, ..}| {
        on_thumb_pointer_down.run(x);
      })
      on_thumb_position_change={on_thumb_position_change}
      on_drag_scroll=Callback::new(move |Pointer { y, .. }| {
        on_drag_scroll.run(y);
      })
      on_wheel_scroll=Callback::new(move |(event, max_scroll_pos): (WheelEvent, f64)| {
        let Some(viewport) = context.viewport.get() else {
          return;
        };

        let scroll_pos = viewport.scroll_top() as f64 + event.delta_y();
        on_wheel_scroll.run(scroll_pos);

        if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
          event.prevent_default();
        }
      })
      on_resize=Callback::new(move |_| {
        let (Some(node_el), Some(viewport), Some(computed_style)) = (node_ref.get(), context.viewport.get(), computed_style.get()) else {
          return;
        };

        on_sizes_change.run(Sizes {
          content: viewport.scroll_height() as f64,
          viewport: viewport.offset_height() as f64,
          scrollbar: Scrollbar {
            size: node_el.client_height() as f64,
            padding_start: computed_style
              .get_property_value("padding-top")
              .expect("no padding top")
              .parse::<f64>()
              .unwrap_or_default(),
            padding_end: computed_style
              .get_property_value("padding-bottom")
              .expect("no padding bottom")
              .parse::<f64>()
              .unwrap_or_default(),
          }
        });
      })
      node_ref={node_ref}
      as_child={as_child}
      attr:data-orientation="vertical"
    >
      {children()}
    </ScrollAreaScrollbarImpl>
  }
}

fn is_scrolling_within_scrollbar_bounds(scroll_pos: f64, max_scroll_pos: f64) -> bool {
  scroll_pos > 0. && scroll_pos < max_scroll_pos
}

#[derive(Clone)]
struct ScrollbarContextValue {
  has_thumb: Signal<bool>,
  scrollbar: NodeRef<Div>,
  on_thumb_change: Callback<HtmlDivElement>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<Pointer>,
  on_thumb_position_change: Callback<()>,
}

struct Pointer {
  x: f64,
  y: f64,
}

#[component]
fn ScrollAreaScrollbarImpl(
  sizes: Signal<Sizes>,
  has_thumb: Signal<bool>,

  #[prop(default=Callback::new(|_|{}), into)] on_pointer_enter: Callback<()>,
  #[prop(default=Callback::new(|_|{}), into)] on_pointer_leave: Callback<()>,
  on_thumb_change: Callback<HtmlDivElement>,
  on_thumb_pointer_up: Callback<()>,
  on_thumb_pointer_down: Callback<Pointer>,
  on_thumb_position_change: Callback<()>,
  on_drag_scroll: Callback<Pointer>,
  on_wheel_scroll: Callback<(WheelEvent, f64)>,
  on_resize: Callback<()>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  children: ChildrenFn,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaScrollbarImpl must be used in a ScrollArea component");

  let rect_ref = StoredValue::<Option<DomRect>, LocalStorage>::new_local(None);
  let previous_webkit_user_select_ref = StoredValue::new(String::new());
  let max_scroll_position = move || sizes.get().content - sizes.get().viewport;

  Effect::new(move |_| {
    let document = use_document();
    let Some(document) = document.as_ref() else {
      return;
    };

    _ = use_event_listener_with_options(
      document.clone(),
      wheel,
      move |ev: WheelEvent| {
        let Some(target) = ev.target() else {
          return;
        };

        let Some(target_el) = target.dyn_ref::<web_sys::Element>() else {
          return;
        };

        let is_scroll_wheel = node_ref
          .get()
          .map(|scrollbar| scrollbar.contains(Some(target_el)))
          .unwrap_or(false);

        if is_scroll_wheel {
          on_wheel_scroll.run((ev, max_scroll_position()));
        }
      },
      UseEventListenerOptions::default().passive(false),
    );
  });

  _ = Effect::watch(
    move || {
      _ = sizes.get();
    },
    move |_, _, _| {
      on_thumb_position_change.run(());
    },
    true,
  );

  use_resize_observer(node_ref, move |_, _| {
    on_resize.run(());
  });

  use_resize_observer(context.content, move |_, _| {
    on_resize.run(());
  });

  let handle_drag_scroll = move |ev: PointerEvent| {
    let Some(rect) = rect_ref.get_value() else {
      return;
    };

    on_drag_scroll.run(Pointer {
      x: ev.client_x() as f64 - rect.left(),
      y: ev.client_y() as f64 - rect.top(),
    })
  };

  Effect::new(move |_| {
    let Some(el) = node_ref.get() else {
      return;
    };

    el.style(("position", "absolute"));
    el.on(pointerdown, move |ev: PointerEvent| {
      let main_pointer = 0;

      if ev.button() != main_pointer {
        return;
      }
      let Some(target) = ev.target() else {
        return;
      };

      let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() else {
        return;
      };

      rect_ref.set_value(Some(el.get_bounding_client_rect()));

      let Some(body) = document().body() else {
        return;
      };

      // body.style.webkitUserSelect = "none";

      let Ok(webkit_user_select) = body.style().get_property_value("webkitUserSelect") else {
        return;
      };

      previous_webkit_user_select_ref.set_value(webkit_user_select);

      if let Some(viewport) = context.viewport.get() {
        _ = viewport.style(("scroll-behavior", "auto"));
      }

      handle_drag_scroll(ev);
    });
    el.on(pointermove, move |ev: PointerEvent| {
      handle_drag_scroll(ev);
    });
    el.on(pointerup, move |ev: PointerEvent| {
      let Some(target) = ev.target() else {
        return;
      };

      let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() else {
        return;
      };

      if el.has_pointer_capture(ev.pointer_id()) {
        _ = el.release_pointer_capture(ev.pointer_id());
      }

      let Some(body) = document().body() else {
        return;
      };

      // body.style.webkitUserSelect = previous_webkit_user_select_ref.get_value();

      if let Some(viewport) = context.viewport.get() {
        _ = viewport.style(("scroll-behavior", ""));
      }

      rect_ref.set_value(None);
    });
    el.on(pointerenter, move |_| {
      on_pointer_enter.run(());
    });
    el.on(pointerleave, move |_| {
      on_pointer_leave.run(());
    });
  });

  provide_context(ScrollbarContextValue {
    scrollbar: node_ref,
    on_thumb_change,
    has_thumb,
    on_thumb_pointer_up,
    on_thumb_pointer_down,
    on_thumb_position_change,
  });

  view! {
    <Primitive
      element={html::div}
      node_ref={node_ref}
      as_child={as_child}
    >
      {children()}
    </Primitive>
  }
}

#[component]
pub fn ScrollAreaThumb(
  #[prop(optional)] force_mount: MaybeSignal<bool>,

  #[prop(optional)] node_ref: NodeRef<Div>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let ScrollbarContextValue { has_thumb, .. } = use_context::<ScrollbarContextValue>()
    .expect("ScrollAreaThumb must be used in a ScrollAreaScrollbarImpl component");

  let is_present = Signal::derive(move || has_thumb.get() || force_mount.get());

  let presence = create_presence(is_present, node_ref);

  let children = StoredValue::new(children);

  view! {
    <Show when=move || presence.get()>
      <ScrollAreaThumbImpl
        node_ref={node_ref}
        as_child={as_child}
      >
        {children.with_value(|children| children.as_ref().map(|children| children()))}
      </ScrollAreaThumbImpl>
    </Show>
  }
}

#[component]
fn ScrollAreaThumbImpl(
  #[prop(optional)] node_ref: NodeRef<Div>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaThumb must be used in a ScrollArea component");
  let scrollbar_context = use_context::<ScrollbarContextValue>()
    .expect("ScrollAreaThumb must be used in a ScrollAreaScrollbarImpl component");

  let remove_unlinked_scroll_listener_ref = StoredValue::<Option<Callback<()>>>::new(None);

  let debounce_scroll_end = use_debounce_fn(
    move || {
      let Some(remove_unlinked_scroll_listener) = remove_unlinked_scroll_listener_ref.get_value()
      else {
        return;
      };

      remove_unlinked_scroll_listener.run(());
      remove_unlinked_scroll_listener_ref.set_value(None);
    },
    100.0,
  );

  Effect::new(move |_| {
    let scroll_listener_debounce_end = debounce_scroll_end.clone();
    _ = use_event_listener(context.viewport, scroll, move |_| {
      scroll_listener_debounce_end();

      if remove_unlinked_scroll_listener_ref.get_value().is_some() {
        return;
      }

      let Some(viewport) = context.viewport.get() else {
        return;
      };

      let listener =
        add_unlinked_scroll_listener(viewport, scrollbar_context.on_thumb_position_change);
      remove_unlinked_scroll_listener_ref.set_value(Some(listener));

      scrollbar_context.on_thumb_position_change.run(());
    });

    scrollbar_context.on_thumb_position_change.run(());
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    node.style(("width", "var(--primitive-scroll-area-thumb-width)"));
    node.style(("height", "var(--primitive-scroll-area-thumb-height)"));
    // onPointerDownCapture?
    node.on(pointerdown, move |ev: PointerEvent| {
      let Some(target) = ev.target() else {
        return;
      };

      let Some(node) = target.dyn_ref::<web_sys::HtmlElement>() else {
        return;
      };

      let rect = node.get_bounding_client_rect();
      let x = ev.client_x() as f64 - rect.left();
      let y = ev.client_y() as f64 - rect.top();

      scrollbar_context
        .on_thumb_pointer_down
        .run(Pointer { x, y });
    });
    node.on(pointerup, move |_| {
      scrollbar_context.on_thumb_pointer_up.run(());
    });

    scrollbar_context.on_thumb_change.run(node);
  });

  let children = StoredValue::new(children);

  view! {
    <Primitive
      element={html::div}
      as_child={as_child}
      node_ref={node_ref}
    >
      {children.with_value(|children| children.as_ref().map(|children| children()))}
    </Primitive>
  }
}

#[component]
pub fn ScrollAreaCorner(
  #[prop(optional)] node_ref: NodeRef<Div>,
  #[prop(optional)] children: Option<ChildrenFn>,

  #[prop(optional, into)] as_child: MaybeProp<bool>,
) -> impl IntoView {
  let context = use_context::<ScrollAreaContextValue>()
    .expect("ScrollAreaCorner must be used in a ScrollArea component");

  let has_both_scrollbars_visible =
    move || context.scrollbar_x.get().is_some() && context.scrollbar_y.get().is_some();
  let has_corner =
    move || context.kind.get() != ScrollAreaKind::Scroll && has_both_scrollbars_visible();

  let (width, set_width) = signal(0);
  let (height, set_height) = signal(0);
  let has_size = move || width.get() != 0 && height.get() != 0;

  use_resize_observer(context.scrollbar_x, move |_, _| {
    let height = match context.scrollbar_x.get() {
      Some(scrollbar_x) => scrollbar_x.offset_height(),
      None => 0,
    };

    context.on_corner_height_change.run(height as u32);
    set_height.set(height);
  });

  use_resize_observer(context.scrollbar_y, move |_, _| {
    let width = match context.scrollbar_y.get() {
      Some(scrollbar_y) => scrollbar_y.offset_width(),
      None => 0,
    };

    context.on_corner_width_change.run(width as u32);
    set_width.set(width);
  });

  Effect::new(move |_| {
    let Some(node) = node_ref.get() else {
      return;
    };

    node.style(("width", width.get().to_string()));
    node.style(("height", height.get().to_string()));
    node.style(("position", "absolute"));

    if let Direction::LeftToRight = context.direction.get() {
      node.style(("right", "0"));
    } else {
      node.style("right");
    }

    if let Direction::RightToLeft = context.direction.get() {
      node.style(("left", "0"));
    } else {
      node.style("left");
    }

    node.style(("bottom", "0"));
  });

  let children = StoredValue::new(children);

  view! {
    <Show when=move || has_corner() || has_size()>
      <Primitive
      element={html::div}
        node_ref={node_ref}
        as_child={as_child}
      >
        {children.with_value(|children| children.as_ref().map(|children| children()))}
      </Primitive>
    </Show>
  }
}

fn get_thumb_size(sizes: &Sizes) -> f64 {
  let ratio = sizes.viewport / sizes.content;
  let scrollbar_padding = sizes.scrollbar.padding_start - sizes.scrollbar.padding_end;
  let thumb_size = (sizes.scrollbar.size - scrollbar_padding) * ratio;

  thumb_size.max(18.)
}

fn get_scroll_position_from_pointer(
  pointer_position: f64,
  pointer_offset: f64,
  sizes: &Sizes,
  direction: Direction,
) -> f64 {
  let thumb_size_px = get_thumb_size(sizes);
  let offset = if pointer_offset == 0. {
    thumb_size_px / 2.
  } else {
    pointer_offset
  };
  let thumb_offset_from_end = thumb_size_px - offset;
  let min_pointer_pos = sizes.scrollbar.padding_start + offset;
  let max_pointer_pos = sizes.scrollbar.size - sizes.scrollbar.padding_end - thumb_offset_from_end;
  let max_scroll_pos = sizes.content - sizes.viewport;
  let scroll_range = if direction == Direction::LeftToRight {
    (0., max_scroll_pos)
  } else {
    (max_scroll_pos * -1., 0.)
  };
  let interpolate = linear_scale((min_pointer_pos, max_pointer_pos), scroll_range);

  interpolate(pointer_position)
}

fn get_thumb_offset_from_scroll(scroll_position: f64, sizes: &Sizes, direction: Direction) -> f64 {
  let thumb_size_px = get_thumb_size(sizes);
  let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
  let scrollbar = sizes.scrollbar.size - scrollbar_padding;
  let max_scroll_pos = sizes.content - sizes.viewport;
  let max_thumb_pos = scrollbar - thumb_size_px;
  let scroll_clamp_range = if direction == Direction::LeftToRight {
    (0., max_scroll_pos)
  } else {
    (max_scroll_pos * -1., 0.)
  };

  let interpolate = linear_scale((0., max_scroll_pos), (0., max_thumb_pos));

  interpolate((scroll_position).clamp(scroll_clamp_range.0, scroll_clamp_range.1))
}

fn add_unlinked_scroll_listener(node: HtmlDivElement, handler: Callback<()>) -> Callback<()> {
  let previous_position = StoredValue::new((node.scroll_left(), node.scroll_top()));

  let Pausable { pause, .. } = use_raf_fn(move |_| {
    let position = (node.scroll_left(), node.scroll_top());

    let is_horizontal_scroll = previous_position.get_value().0 != position.0;
    let is_vertical_scroll = previous_position.get_value().1 != position.1;

    if is_horizontal_scroll || is_vertical_scroll {
      handler.run(());
    }

    previous_position.set_value(position);
  });

  Callback::new(move |_| {
    // pause();
  })
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
