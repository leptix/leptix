use std::collections::HashMap;

use leptos::{
  html::{CreateElement, ElementType},
  prelude::*,
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Array;

use leptos_use::core::{IntoElementMaybeSignalType, OptionSignalMarker};

#[derive(Clone)]
pub struct CollectionContextValue<ItemData: Send + Sync + Clone + Ord + 'static, ItemElement>
where
  ItemElement: ElementType + Clone + 'static,
  <ItemElement as ElementType>::Output: JsCast + Clone + 'static,
  NodeRef<ItemElement>: IntoElementMaybeSignalType<web_sys::EventTarget, OptionSignalMarker>
    + WithUntracked<Value = Option<ItemElement::Output>>,
  <NodeRef<ItemElement> as WithUntracked>::Value: Clone + 'static,
{
  pub(crate) collection_ref: NodeRef<ItemElement>,
  pub(crate) item_map: RwSignal<HashMap<CollectionItemId, (NodeRef<ItemElement>, ItemData)>>,
}

use derive_more::Deref;

#[derive(Deref, Clone, PartialEq, Eq, Hash)]
pub struct CollectionItemId(String);

impl CollectionItemId {
  fn new() -> Self {
    Self(nanoid::nanoid!())
  }
}

pub fn use_collection_item_ref<ItemElement, ItemData: Clone + Ord + Send + Sync + 'static>(
  item_ref: NodeRef<ItemElement>,
  data: ItemData,
) -> NodeRef<ItemElement>
where
  ItemElement: ElementType + CreateElement<Dom> + Clone + 'static,
  <ItemElement as ElementType>::Output: JsCast + leptos::html::ElementExt + Clone + 'static,
  NodeRef<ItemElement>: IntoElementMaybeSignalType<web_sys::EventTarget, OptionSignalMarker>
    + WithUntracked<Value = Option<ItemElement::Output>>,
  <NodeRef<ItemElement> as WithUntracked>::Value: Clone + 'static,
{
  let CollectionContextValue { item_map, .. } =
    use_context::<CollectionContextValue<ItemData, ItemElement>>().expect(
      "create_collection_item_ref must be used in a component that provides a collection context",
    );

  let (id, set_id) = signal::<Option<CollectionItemId>>(None);
  //let item_ref = NodeRef::<ItemElement>::new();

  Effect::new(move |_| {
    if let Some(node) = item_ref.get() {
      let id = CollectionItemId::new();

      // node.attr(("data-primitive-collection-item", id.0.as_str()));

      set_id.set(Some(id));
    }
  });

  Effect::new(move |_| {
    let Some(id) = id.get() else {
      return;
    };

    item_map.update(|item_map| {
      item_map.insert(id.clone(), (item_ref, data.clone()));
    });
  });

  on_cleanup(move || {
    let Some(id) = id.get() else {
      return;
    };

    _ = item_map.try_update(|item_map| {
      item_map.remove(&id.clone());
    });
  });

  item_ref
}

pub fn use_collection_context<
  ItemData: Clone + Ord + std::fmt::Debug + Sync + Send + 'static,
  ItemElement,
>() -> Signal<Vec<(NodeRef<ItemElement>, ItemData)>>
where
  NodeRef<ItemElement>: IntoElementMaybeSignalType<web_sys::EventTarget, OptionSignalMarker>
    + WithUntracked<Value = Option<ItemElement::Output>>,
  <NodeRef<ItemElement> as WithUntracked>::Value: Clone,
  ItemElement: ElementType + Clone + Send + Sync + 'static,
  <ItemElement as ElementType>::Output: JsCast + Clone + 'static,
{
  let CollectionContextValue {
    collection_ref,
    item_map,
  } = use_context::<CollectionContextValue<ItemData, ItemElement>>().expect(
    "use_collection_context must be used in a component that provides a collection context",
  );

  Signal::derive(move || {
    let Some(collection_node) = collection_ref.get() else {
      return vec![];
    };

    let Some(el) = collection_node.dyn_ref::<web_sys::HtmlElement>() else {
      return vec![];
    };

    let Ok(ordered_nodes) = el.query_selector_all("[data-primitive-collection-item]") else {
      return vec![];
    };

    let ordered_nodes = Array::from(&ordered_nodes);

    let items = item_map.get();
    let mut foo = items.into_values().collect::<Vec<_>>();

    foo.sort_by(|curr, next| {
      ordered_nodes
        .index_of(
          &curr
            .0
            .get()
            .and_then(|curr| curr.dyn_into::<JsValue>().ok())
            .unwrap(),
          0,
        )
        .cmp(
          &ordered_nodes.index_of(
            &next
              .0
              .get()
              .and_then(|curr| curr.dyn_into::<JsValue>().ok())
              .unwrap(),
            0,
          ),
        )
    });

    foo
  })
}
