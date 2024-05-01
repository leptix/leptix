use std::collections::HashMap;

use leptos::{html::ElementDescriptor, *};
use web_sys::js_sys::Array;

#[derive(Clone)]
pub struct CollectionContextValue<
  ItemData: Clone + Ord + 'static,
  ItemElement: ElementDescriptor + Clone + 'static,
> {
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

pub fn create_collection_item_ref<
  ItemElement: ElementDescriptor + Clone + 'static,
  ItemData: Clone + Ord + 'static,
>(
  data: ItemData,
) -> NodeRef<ItemElement> {
  let CollectionContextValue { item_map, .. } =
    use_context::<CollectionContextValue<ItemData, ItemElement>>().expect(
      "create_collection_item_ref must be used in a component that provides a collection context",
    );

  let (id, set_id) = create_signal::<Option<CollectionItemId>>(None);
  let item_ref = create_node_ref::<ItemElement>();

  Effect::new(move |_| {
    if let Some(node) = item_ref.get() {
      let id = CollectionItemId::new();

      _ = node.attr(
        "data-primitive-collection-item",
        id.0.clone().into_attribute(),
      );

      set_id(Some(id));
    }
  });

  Effect::new(move |_| {
    let Some(id) = id.get() else {
      return;
    };

    item_map.update(|item_map| {
      item_map.insert(id.clone(), (item_ref, data.clone()));
    });

    on_cleanup(move || {
      item_map.update(|item_map| {
        item_map.remove(&id.clone());
      });
    });
  });

  item_ref
}

pub fn use_collection_context<
  ItemData: Clone + Ord + std::fmt::Debug + 'static,
  ItemElement: ElementDescriptor + Clone + 'static,
>() -> Signal<Vec<(NodeRef<ItemElement>, ItemData)>> {
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

    let el = collection_node.into_any();

    if el.is_null() {
      return vec![];
    }

    let Ok(ordered_nodes) = el.query_selector_all("[data-primitive-collection-item]") else {
      return vec![];
    };

    let ordered_nodes = Array::from(&ordered_nodes);

    let items = item_map.get();
    let mut foo = items.into_values().collect::<Vec<_>>();

    foo.sort_by(|curr, next| {
      ordered_nodes
        .index_of(&curr.0.get().unwrap().into_any(), 0)
        .cmp(&ordered_nodes.index_of(&next.0.get().unwrap().into_any(), 0))
    });

    foo
  })
}
