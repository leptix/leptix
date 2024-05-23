use leptos::html::Input;
use leptos::*;

use leptix_primitives::components::accordion::{
  AccordionContent, AccordionHeader, AccordionItem, AccordionKind, AccordionRoot, AccordionTrigger,
};
use leptix_primitives::components::aspect_ratio::AspectRatioRoot;
use leptix_primitives::components::avatar::{AvatarFallback, AvatarImage, AvatarRoot};
use leptix_primitives::components::checkbox::{CheckboxIndicator, CheckboxRoot, CheckedState};
use leptix_primitives::components::collapsible::{
  CollapsibleContent, CollapsibleRoot, CollapsibleTrigger,
};
use leptix_primitives::components::label::LabelRoot;
use leptix_primitives::components::progress::{ProgressIndicator, ProgressRoot};
use leptix_primitives::components::radio_group::{
  RadioGroupIndicator, RadioGroupItem, RadioGroupRoot,
};
use leptix_primitives::components::scroll_area::{
  ScrollAreaCorner, ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport,
};
use leptix_primitives::components::separator::Separator;
use leptix_primitives::components::slider::{SliderRange, SliderRoot, SliderThumb, SliderTrack};
use leptix_primitives::components::switch::{SwitchRoot, SwitchThumb};
use leptix_primitives::components::tabs::{TabsContent, TabsList, TabsRoot, TabsTrigger};
use leptix_primitives::components::toggle::ToggleRoot;
use leptix_primitives::components::toggle_group::{
  ToggleGroupItem, ToggleGroupKind, ToggleGroupRoot,
};
use leptix_primitives::components::toolbar::{
  ToolbarButton, ToolbarLink, ToolbarRoot, ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem,
};
use leptix_primitives::util::Orientation;
use leptix_primitives::Attributes;

use leptos_use::{use_interval_fn, utils::Pausable};

fn main() {
  console_error_panic_hook::set_once();

  mount_to_body(|| {
    view! {
      <main class="dark:bg-[#111113] p-4 flex flex-col gap-2 text-mauve11 dark:text-white">
        <PrimitivesDemo />
      </main>
    }
  });
}

#[component]
pub fn PrimitivesDemo() -> impl IntoView {
  view! {
    <>
      <ThemeToggle/>

      <WithTitle title="Accordion">
        <AccordionDemo/>
      </WithTitle>

      <WithTitle title="Aspect Ratio">
        <AspectRatioDemo/>
      </WithTitle>

      <WithTitle title="Avatar">
        <AvatarDemo/>
      </WithTitle>

      <WithTitle title="Checkbox">
        <CheckboxDemo/>
      </WithTitle>

      <WithTitle title="Collapsible">
        <CollapsibleDemo/>
      </WithTitle>

      <WithTitle title="Label">
        <LabelDemo/>
      </WithTitle>

      <WithTitle title="Progress">
        <ProgressDemo/>
      </WithTitle>

      <WithTitle title="Radio Group">
        <RadioGroupDemo/>
      </WithTitle>

      <WithTitle title="ScrollArea">
        <ScrollAreaDemo/>
      </WithTitle>

      <WithTitle title="Separator">
        <SeparatorDemo/>
      </WithTitle>

      <WithTitle title="Slider">
        <SliderDemo/>
      </WithTitle>

      <WithTitle title="Switch">
        <SwitchDemo/>
      </WithTitle>

      <WithTitle title="Tabs">
        <TabsDemo/>
      </WithTitle>

      <WithTitle title="Toggle">
        <ToggleDemo/>
      </WithTitle>

      <WithTitle title="Toggle Group">
        <ToggleGroupDemo/>
      </WithTitle>

      <WithTitle title="Toolbar">
        <ToolbarDemo/>
      </WithTitle>
    </>
  }
}

#[component]
fn ThemeToggle() -> impl IntoView {
  let (dark_theme, set_dark_theme) = create_signal(true);

  view! {
    <ToggleRoot
      attr:aria-label="Toggle italic"
      attr:class="dark:hover:bg-neutral-800 dark:bg-neutral-900 hover:bg-black/20 bg-black/10 color-mauve11 shadow-blackA4 flex h-[35px] w-[35px] items-center justify-center rounded leading-4 shadow-[0_2px_10px] focus:shadow-[0_0_0_2px] focus:shadow-black"
      pressed=true.into()
      on_click=Callback::new(move |_| {
          set_dark_theme
              .update(|dark_theme| {
                  *dark_theme = !*dark_theme;
              });
          let Some(el) = document().document_element() else {
              return;
          };
          if dark_theme.get() {
              _ = el.class_list().add_1("dark");
          } else {
              _ = el.class_list().remove_1("dark");
          }
      })
    >

      {move || {
          if dark_theme.get() {
              view! { <MoonIcon/> }
          } else {
              view! { <SunIcon/> }
          }
      }}

    </ToggleRoot>
  }
}

#[component]
fn SunIcon() -> impl IntoView {
  view! {
    <svg
      width="16"
      height="16"
      viewBox="0 0 15 15"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      style="display:var(--theme-toggle-sun-icon-display)"
    >
      <path
        d="M7.5 0C7.77614 0 8 0.223858 8 0.5V2.5C8 2.77614 7.77614 3 7.5 3C7.22386 3 7 2.77614 7 2.5V0.5C7 0.223858 7.22386 0 7.5 0ZM2.1967 2.1967C2.39196 2.00144 2.70854 2.00144 2.90381 2.1967L4.31802 3.61091C4.51328 3.80617 4.51328 4.12276 4.31802 4.31802C4.12276 4.51328 3.80617 4.51328 3.61091 4.31802L2.1967 2.90381C2.00144 2.70854 2.00144 2.39196 2.1967 2.1967ZM0.5 7C0.223858 7 0 7.22386 0 7.5C0 7.77614 0.223858 8 0.5 8H2.5C2.77614 8 3 7.77614 3 7.5C3 7.22386 2.77614 7 2.5 7H0.5ZM2.1967 12.8033C2.00144 12.608 2.00144 12.2915 2.1967 12.0962L3.61091 10.682C3.80617 10.4867 4.12276 10.4867 4.31802 10.682C4.51328 10.8772 4.51328 11.1938 4.31802 11.3891L2.90381 12.8033C2.70854 12.9986 2.39196 12.9986 2.1967 12.8033ZM12.5 7C12.2239 7 12 7.22386 12 7.5C12 7.77614 12.2239 8 12.5 8H14.5C14.7761 8 15 7.77614 15 7.5C15 7.22386 14.7761 7 14.5 7H12.5ZM10.682 4.31802C10.4867 4.12276 10.4867 3.80617 10.682 3.61091L12.0962 2.1967C12.2915 2.00144 12.608 2.00144 12.8033 2.1967C12.9986 2.39196 12.9986 2.70854 12.8033 2.90381L11.3891 4.31802C11.1938 4.51328 10.8772 4.51328 10.682 4.31802ZM8 12.5C8 12.2239 7.77614 12 7.5 12C7.22386 12 7 12.2239 7 12.5V14.5C7 14.7761 7.22386 15 7.5 15C7.77614 15 8 14.7761 8 14.5V12.5ZM10.682 10.682C10.8772 10.4867 11.1938 10.4867 11.3891 10.682L12.8033 12.0962C12.9986 12.2915 12.9986 12.608 12.8033 12.8033C12.608 12.9986 12.2915 12.9986 12.0962 12.8033L10.682 11.3891C10.4867 11.1938 10.4867 10.8772 10.682 10.682ZM5.5 7.5C5.5 6.39543 6.39543 5.5 7.5 5.5C8.60457 5.5 9.5 6.39543 9.5 7.5C9.5 8.60457 8.60457 9.5 7.5 9.5C6.39543 9.5 5.5 8.60457 5.5 7.5ZM7.5 4.5C5.84315 4.5 4.5 5.84315 4.5 7.5C4.5 9.15685 5.84315 10.5 7.5 10.5C9.15685 10.5 10.5 9.15685 10.5 7.5C10.5 5.84315 9.15685 4.5 7.5 4.5Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn MoonIcon() -> impl IntoView {
  view! {
    <svg
      width="16"
      height="16"
      viewBox="0 0 15 15"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      style="display:var(--theme-toggle-moon-icon-display)"
    >
      <path
        d="M2.89998 0.499976C2.89998 0.279062 2.72089 0.0999756 2.49998 0.0999756C2.27906 0.0999756 2.09998 0.279062 2.09998 0.499976V1.09998H1.49998C1.27906 1.09998 1.09998 1.27906 1.09998 1.49998C1.09998 1.72089 1.27906 1.89998 1.49998 1.89998H2.09998V2.49998C2.09998 2.72089 2.27906 2.89998 2.49998 2.89998C2.72089 2.89998 2.89998 2.72089 2.89998 2.49998V1.89998H3.49998C3.72089 1.89998 3.89998 1.72089 3.89998 1.49998C3.89998 1.27906 3.72089 1.09998 3.49998 1.09998H2.89998V0.499976ZM5.89998 3.49998C5.89998 3.27906 5.72089 3.09998 5.49998 3.09998C5.27906 3.09998 5.09998 3.27906 5.09998 3.49998V4.09998H4.49998C4.27906 4.09998 4.09998 4.27906 4.09998 4.49998C4.09998 4.72089 4.27906 4.89998 4.49998 4.89998H5.09998V5.49998C5.09998 5.72089 5.27906 5.89998 5.49998 5.89998C5.72089 5.89998 5.89998 5.72089 5.89998 5.49998V4.89998H6.49998C6.72089 4.89998 6.89998 4.72089 6.89998 4.49998C6.89998 4.27906 6.72089 4.09998 6.49998 4.09998H5.89998V3.49998ZM1.89998 6.49998C1.89998 6.27906 1.72089 6.09998 1.49998 6.09998C1.27906 6.09998 1.09998 6.27906 1.09998 6.49998V7.09998H0.499976C0.279062 7.09998 0.0999756 7.27906 0.0999756 7.49998C0.0999756 7.72089 0.279062 7.89998 0.499976 7.89998H1.09998V8.49998C1.09998 8.72089 1.27906 8.89997 1.49998 8.89997C1.72089 8.89997 1.89998 8.72089 1.89998 8.49998V7.89998H2.49998C2.72089 7.89998 2.89998 7.72089 2.89998 7.49998C2.89998 7.27906 2.72089 7.09998 2.49998 7.09998H1.89998V6.49998ZM8.54406 0.98184L8.24618 0.941586C8.03275 0.917676 7.90692 1.1655 8.02936 1.34194C8.17013 1.54479 8.29981 1.75592 8.41754 1.97445C8.91878 2.90485 9.20322 3.96932 9.20322 5.10022C9.20322 8.37201 6.82247 11.0878 3.69887 11.6097C3.45736 11.65 3.20988 11.6772 2.96008 11.6906C2.74563 11.702 2.62729 11.9535 2.77721 12.1072C2.84551 12.1773 2.91535 12.2458 2.98667 12.3128L3.05883 12.3795L3.31883 12.6045L3.50684 12.7532L3.62796 12.8433L3.81491 12.9742L3.99079 13.089C4.11175 13.1651 4.23536 13.2375 4.36157 13.3059L4.62496 13.4412L4.88553 13.5607L5.18837 13.6828L5.43169 13.7686C5.56564 13.8128 5.70149 13.8529 5.83857 13.8885C5.94262 13.9155 6.04767 13.9401 6.15405 13.9622C6.27993 13.9883 6.40713 14.0109 6.53544 14.0298L6.85241 14.0685L7.11934 14.0892C7.24637 14.0965 7.37436 14.1002 7.50322 14.1002C11.1483 14.1002 14.1032 11.1453 14.1032 7.50023C14.1032 7.25044 14.0893 7.00389 14.0623 6.76131L14.0255 6.48407C13.991 6.26083 13.9453 6.04129 13.8891 5.82642C13.8213 5.56709 13.7382 5.31398 13.6409 5.06881L13.5279 4.80132L13.4507 4.63542L13.3766 4.48666C13.2178 4.17773 13.0353 3.88295 12.8312 3.60423L12.6782 3.40352L12.4793 3.16432L12.3157 2.98361L12.1961 2.85951L12.0355 2.70246L11.8134 2.50184L11.4925 2.24191L11.2483 2.06498L10.9562 1.87446L10.6346 1.68894L10.3073 1.52378L10.1938 1.47176L9.95488 1.3706L9.67791 1.2669L9.42566 1.1846L9.10075 1.09489L8.83599 1.03486L8.54406 0.98184ZM10.4032 5.30023C10.4032 4.27588 10.2002 3.29829 9.83244 2.40604C11.7623 3.28995 13.1032 5.23862 13.1032 7.50023C13.1032 10.593 10.596 13.1002 7.50322 13.1002C6.63646 13.1002 5.81597 12.9036 5.08355 12.5522C6.5419 12.0941 7.81081 11.2082 8.74322 10.0416C8.87963 10.2284 9.10028 10.3497 9.34928 10.3497C9.76349 10.3497 10.0993 10.0139 10.0993 9.59971C10.0993 9.24256 9.84965 8.94373 9.51535 8.86816C9.57741 8.75165 9.63653 8.63334 9.6926 8.51332C9.88358 8.63163 10.1088 8.69993 10.35 8.69993C11.0403 8.69993 11.6 8.14028 11.6 7.44993C11.6 6.75976 11.0406 6.20024 10.3505 6.19993C10.3853 5.90487 10.4032 5.60464 10.4032 5.30023Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn ErrorIcon() -> impl IntoView {
  view! {
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="14"
      height="14"
      fill="currentColor"
      class="bi bi-exclamation-circle"
      viewBox="0 0 16 16"
    >
      <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"></path>
      <path d="M7.002 11a1 1 0 1 1 2 0 1 1 0 0 1-2 0M7.1 4.995a.905.905 0 1 1 1.8 0l-.35 3.507a.552.552 0 0 1-1.1 0z"></path>
    </svg>
  }
}

#[component]
fn WithTitle(
  title: &'static str,
  children: Children,
  #[prop(optional)] broken: bool,
) -> impl IntoView {
  view! {
    <div class="p-2 rounded-md dark:bg-neutral-900 bg-black/10 max-w-[500px]">
      <div class="flex gap-2 mb-2">
        <div class="px-2 pb-[0.075rem] font-semibold text-sm bg-violet9/20 border-violet9/20 border text-violet9 w-fit rounded-full">
          {title}
        </div>

        <Show when=move || broken>
            <div class="flex items-center gap-1 px-2 pb-[0.075rem] font-semibold text-sm bg-red9/20 border-red9/20 border text-red9 w-fit rounded-full">
                <div class="translate-y-[0.5px]">
                    <ErrorIcon/>
                </div>

                "Currently Broken"
            </div>
        </Show>
      </div>
      <div class="pl-2.5 mb-2.5 mr-2.5">{children()}</div>
    </div>
  }
}

#[component]
fn CheckboxDemo() -> impl IntoView {
  view! {
    <form>
      <div class="flex items-center">
        <CheckboxRoot
          default_checked=CheckedState::Checked(true).into()
          attr:class="shadow-blackA4 hover:bg-violet3 flex h-[25px] w-[25px] appearance-none items-center justify-center rounded-[4px] bg-white shadow-[0_2px_10px] outline-none focus:shadow-[0_0_0_2px_black]"
          attr:id="c1"
        >
          <CheckboxIndicator attr:class="text-violet11">
            <CheckIcon/>
          </CheckboxIndicator>
        </CheckboxRoot>

        <label class="pl-[15px] text-[15px] leading-none" for="c1">
          <span class="select-none">"Accept terms and conditions."</span>
        </label>
      </div>
    </form>
  }
}

#[component]
fn AspectRatioDemo() -> impl IntoView {
  view! {
    <div class="shadow-blackA4 w-[300px] overflow-hidden rounded-md shadow-[0_2px_10px]">
      <AspectRatioRoot ratio=(16.0 / 9.0).into()>
        <img
          class="h-full w-full object-cover"
          src="https://images.unsplash.com/photo-1535025183041-0991a977e25b?w=300&dpr=2&q=80"
          alt="Landscape photograph by Tobias Tullius"
        />
      </AspectRatioRoot>
    </div>
  }
}

#[component]
fn ProgressDemo() -> impl IntoView {
  let (progress, set_progress) = create_signal(25u32);
  let (indicator_style, set_indicator_style) = create_signal(format!(
    "transform: translateX(-{}%)",
    100 - progress.get_untracked()
  ));

  Effect::new(move |_| {
    let Pausable { pause, .. } = use_interval_fn(
      move || {
        set_progress.update(|progress| {
          if *progress < 100 {
            *progress = *progress + 25;
          } else {
            *progress = 0;
          }
        });

        set_indicator_style.set(format!(
          "transform: translateX(-{}%)",
          100 - (progress.get_untracked() % 101)
        ));
      },
      1000,
    );

    on_cleanup(move || {
      pause();
    });
  });

  view! {
    <ProgressRoot
      attr:class="relative overflow-hidden bg-black/25 rounded-full w-[300px] h-[25px] drop-shadow-md"
      attr:style="transform: translateZ(0)"
      value=progress.into()
    >
      <ProgressIndicator
        attr:class="bg-white w-full h-full transition-transform duration-[660ms] ease-[cubic-bezier(0.65, 0, 0.35, 1)]"
        attr:style=move || indicator_style.get()
      />
    </ProgressRoot>
  }
}

#[component]
fn CheckIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn AvatarDemo() -> impl IntoView {
  view! {
    <div class="flex gap-5">
      <AvatarRoot attr:class="bg-blackA1 shadow-[0_2px_10px] shadow-blackA4 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
        <AvatarImage
          attr:class="h-full w-full rounded-[inherit] object-cover"
          attr:src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
          attr:alt="Colm Tuite"
        />
        <AvatarFallback
          attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
          delay_ms=600
        >
          CT
        </AvatarFallback>
      </AvatarRoot>
      <AvatarRoot attr:class="bg-blackA1 shadow-[0_2px_10px] shadow-blackA4 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
        <AvatarImage
          attr:class="h-full w-full rounded-[inherit] object-cover"
          attr:src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
          attr:alt="Pedro Duarte"
        />
        <AvatarFallback
          attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
          delay_ms=600
        >
          JD
        </AvatarFallback>
      </AvatarRoot>
      <AvatarRoot attr:class="bg-blackA1 shadow-[0_2px_10px] shadow-blackA4 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
        <AvatarFallback attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium">
          PD
        </AvatarFallback>
      </AvatarRoot>
    </div>
  }
}

#[component]
fn CollapsibleDemo() -> impl IntoView {
  let (open, set_open) = create_signal(false);

  view! {
    <CollapsibleRoot
      attr:class="w-[300px]"
      open=open.into()
      on_open_change=Callback::new(move |open: bool| set_open.set(open))
    >
      <div class="flex items-center justify-between">
        <span class="dark:text-white text-[15px] leading-[25px] dark:text-white">
          "@peduarte starred 3 repositories"
        </span>
        <CollapsibleTrigger as_child=true>
          <button class="rounded-full h-[25px] w-[25px] inline-flex items-center justify-center text-violet11 shadow-[0_2px_10px] shadow-blackA4 outline-none data-[state=closed]:bg-white data-[state=open]:bg-violet3 hover:bg-violet3 focus:shadow-[0_0_0_2px] focus:shadow-black">
            {move || {
                if open.get() {
                    view! { <Cross2Icon/> }
                } else {
                    view! { <RowSpacingIcon/> }
                }
            }}

          </button>
        </CollapsibleTrigger>
      </div>

      <div class="bg-white rounded my-[10px] p-[10px] shadow-[0_2px_10px] shadow-blackA4">
        <span class="text-violet11 text-[15px] leading-[25px]">"leptix/primitives"</span>
      </div>

      <CollapsibleContent>
        <div class="bg-white rounded my-[10px] p-[10px] shadow-[0_2px_10px] shadow-blackA4">
          <span class="text-violet11 text-[15px] leading-[25px]">"@radix-ui/colors"</span>
        </div>
        <div class="bg-white rounded my-[10px] p-[10px] shadow-[0_2px_10px] shadow-blackA4">
          <span class="text-violet11 text-[15px] leading-[25px]">"@stitches/react"</span>
        </div>
      </CollapsibleContent>
    </CollapsibleRoot>
  }
}

#[component]
fn Cross2Icon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M11.7816 4.03157C12.0062 3.80702 12.0062 3.44295 11.7816 3.2184C11.5571 2.99385 11.193 2.99385 10.9685 3.2184L7.50005 6.68682L4.03164 3.2184C3.80708 2.99385 3.44301 2.99385 3.21846 3.2184C2.99391 3.44295 2.99391 3.80702 3.21846 4.03157L6.68688 7.49999L3.21846 10.9684C2.99391 11.193 2.99391 11.557 3.21846 11.7816C3.44301 12.0061 3.80708 12.0061 4.03164 11.7816L7.50005 8.31316L10.9685 11.7816C11.193 12.0061 11.5571 12.0061 11.7816 11.7816C12.0062 11.557 12.0062 11.193 11.7816 10.9684L8.31322 7.49999L11.7816 4.03157Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn RowSpacingIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M7.81832 0.68179C7.64258 0.506054 7.35766 0.506054 7.18192 0.68179L5.18192 2.68179C5.00619 2.85753 5.00619 3.14245 5.18192 3.31819C5.35766 3.49392 5.64258 3.49392 5.81832 3.31819L7.05012 2.08638L7.05012 5.50023C7.05012 5.74876 7.25159 5.95023 7.50012 5.95023C7.74865 5.95023 7.95012 5.74876 7.95012 5.50023L7.95012 2.08638L9.18192 3.31819C9.35766 3.49392 9.64258 3.49392 9.81832 3.31819C9.99406 3.14245 9.99406 2.85753 9.81832 2.68179L7.81832 0.68179ZM7.95012 12.9136V9.50023C7.95012 9.2517 7.74865 9.05023 7.50012 9.05023C7.25159 9.05023 7.05012 9.2517 7.05012 9.50023V12.9136L5.81832 11.6818C5.64258 11.5061 5.35766 11.5061 5.18192 11.6818C5.00619 11.8575 5.00619 12.1424 5.18192 12.3182L7.18192 14.3182C7.26632 14.4026 7.38077 14.45 7.50012 14.45C7.61947 14.45 7.73393 14.4026 7.81832 14.3182L9.81832 12.3182C9.99406 12.1424 9.99406 11.8575 9.81832 11.6818C9.64258 11.5061 9.35766 11.5061 9.18192 11.6818L7.95012 12.9136ZM1.49994 7.00017C1.2238 7.00017 0.999939 7.22403 0.999939 7.50017C0.999939 7.77631 1.2238 8.00017 1.49994 8.00017L13.4999 8.00017C13.7761 8.00017 13.9999 7.77631 13.9999 7.50017C13.9999 7.22403 13.7761 7.00017 13.4999 7.00017L1.49994 7.00017Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn AccordionDemo() -> impl IntoView {
  view! {
    <AccordionRoot
      attr:class="bg-mauve6 w-[300px] rounded-md shadow-[0_2px_10px] shadow-black/5"
      kind=AccordionKind::Single {
          value: None,
          default_value: Some("item-1".into()),
          collapsible: Some(true.into()),
          on_value_change: None,
      }
    >

      // kind=AccordionKind::Multiple {
      // value: None,
      // default_value: None,
      // on_value_change: None
      // }
      // <AccordionItemDemo value=Signal::derive(|| "item-1".into())>
      <AccordionItemDemo value="item-1".into()>
        <AccordionTriggerDemo>"Is it accessible?"</AccordionTriggerDemo>
        <AccordionContentDemo>
          "Yes. It adheres to the WAI-ARIA design pattern."
        </AccordionContentDemo>
      </AccordionItemDemo>

      <AccordionItemDemo value="item-2".into()>
        <AccordionTriggerDemo>"Is it unstyled?"</AccordionTriggerDemo>
        <AccordionContentDemo>
          "Yes. It's unstyled by default, giving you freedom over the look and feel."
        </AccordionContentDemo>
      </AccordionItemDemo>

      <AccordionItemDemo value="item-3".into()>
        <AccordionTriggerDemo>"Can it be animated?"</AccordionTriggerDemo>
        <AccordionContentDemo>
          "Yes! You can animate the Accordion with CSS or Rust."
        </AccordionContentDemo>
      </AccordionItemDemo>
    </AccordionRoot>
  }
}

#[component]
fn AccordionItemDemo(value: MaybeSignal<String>, children: Children) -> impl IntoView {
  view! {
    <AccordionItem
      value=value
      attr:class="focus-within:shadow-mauve12 mt-px overflow-hidden first:mt-0 first:rounded-t last:rounded-b focus-within:relative focus-within:z-10 focus-within:shadow-[0_0_0_2px]"
    >
      {children()}
    </AccordionItem>
  }
}

#[component]
fn AccordionTriggerDemo(children: Children) -> impl IntoView {
  view! {
    <AccordionHeader attr:class="flex">
      <AccordionTrigger attr:class="text-violet11 shadow-mauve6 hover:bg-mauve2 group flex h-[45px] flex-1 cursor-default items-center justify-between bg-white px-5 text-[15px] leading-none shadow-[0_1px_0] outline-none">
        {children()}
        <ChevronDownIcon
          attr:class="text-violet10 ease-[cubic-bezier(0.87,_0,_0.13,_1)] transition-transform duration-300 group-data-[state=open]:rotate-180"
          attr:aria-hidden=true.into_attribute()
        />
      </AccordionTrigger>
    </AccordionHeader>
  }
}

#[component]
fn AccordionContentDemo(children: ChildrenFn) -> impl IntoView {
  view! {
    <AccordionContent attr:class="AccordionContent text-mauve11 bg-mauve2 data-[state=open]:animate-slideDown data-[state=closed]:animate-slideUp overflow-hidden text-[15px]">
      <div class="py-[15px] px-5">{children()}</div>
    </AccordionContent>
  }
}

#[component]
fn ChevronDownIcon(#[prop(attrs)] attrs: Attributes) -> impl IntoView {
  view! {
    <svg
      width="15"
      height="15"
      viewBox="0 0 15 15"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      {..attrs}
    >
      <path
        d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn LabelDemo() -> impl IntoView {
  let node_ref = NodeRef::<Input>::new();
  node_ref.on_load(|node| {
    node.set_default_value("Pedro Duarte");
  });

  view! {
    <div class="flex gap-4">
      <LabelRoot
        attr:class="text-[15px] font-semibold leading-[35px] dark:text-white text-mauve11"
        for_html="firstName".into()
      >
        "First name"
      </LabelRoot>
      <input
        node_ref=node_ref
        class="bg-blackA2 shadow-blackA6 inline-flex h-[35px] flex-1 appearance-none items-center justify-center rounded-[4px] px-[10px] text-[15px] leading-none text-blackA7 dark:text-white shadow-[0_0_0_1px] outline-none focus:shadow-[0_0_0_2px_black] selection:text-white selection:bg-black/50"
        type="text"
        id="firstName"
      />
    </div>
  }
}

#[component]
fn SeparatorDemo() -> impl IntoView {
  view! {
    <div class="w-full max-w-[300px]">
      <div class="dark:text-white text-[15px] leading-5 font-medium">"Leptix Primitives"</div>
      <div class="dark:text-white text-[15px] leading-5">
        "Accessible and unstyled components for Leptos"
      </div>
      <Separator attr:class="bg-mauve11 dark:bg-white data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px my-[15px]"/>
      <div class="flex h-5 items-center">
        <div class="dark:text-white text-[15px] leading-5">"Blog"</div>
        <Separator
          attr:class="bg-mauve11 dark:bg-white data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
          decorative=true.into()
          orientation=Orientation::Vertical.into()
        />
        <div class="dark:text-white text-[15px] leading-5">"Docs"</div>
        <Separator
          attr:class="bg-mauve11 dark:bg-white data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px mx-[15px]"
          decorative=true.into()
          orientation=Orientation::Vertical.into()
        />
        <div class="dark:text-white text-[15px] leading-5">"Source"</div>
      </div>
    </div>
  }
}

#[component]
fn ToggleDemo() -> impl IntoView {
  view! {
    <ToggleRoot
      attr:aria-label="Toggle italic"
      attr:class="hover:bg-violet3 color-mauve11 data-[state=on]:bg-violet6 data-[state=on]:text-violet12 shadow-blackA4 flex h-[35px] w-[35px] items-center justify-center rounded bg-white text-violet9 leading-4 shadow-[0_2px_10px] focus:shadow-[0_0_0_2px] focus:shadow-black"
    >
      <i class="italic">"I"</i>
    </ToggleRoot>
  }
}

#[component]
fn ToggleGroupDemo() -> impl IntoView {
  let toggle_group_item_classes = "hover:bg-violet3 color-mauve11 data-[state=on]:bg-violet6 data-[state=on]:text-violet9 flex h-[35px] w-[35px] items-center justify-center bg-white text-[#65636d] leading-4 first:rounded-l last:rounded-r focus:z-10 focus:shadow-[0_0_0_2px] focus:shadow-black focus:outline-none";

  view! {
    <ToggleGroupRoot
      attr:class="inline-flex bg-mauve6 rounded shadow-[0_2px_10px] shadow-blackA4 space-x-px"
      kind=ToggleGroupKind::Single {
          value: None,
          on_value_change: None,
          default_value: Some("center".into()),
      }

      attr:aria-label="Text alignment"
    >
      <ToggleGroupItem
        value="left".into()
        attr:class=toggle_group_item_classes
        attr:aria-label="Left aligned"
      >
        <TextAlignLeftIcon/>
      </ToggleGroupItem>
      <ToggleGroupItem
        value="center".into()
        attr:class=toggle_group_item_classes
        attr:aria-label="Center aligned"
      >
        <TextAlignCenterIcon/>
      </ToggleGroupItem>
      <ToggleGroupItem
        value="right".into()
        attr:class=toggle_group_item_classes
        attr:aria-label="Right aligned"
      >
        <TextAlignRightIcon/>
      </ToggleGroupItem>
    </ToggleGroupRoot>
  }
}

#[component]
pub fn TextAlignLeftIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M2 4.5C2 4.22386 2.22386 4 2.5 4H12.5C12.7761 4 13 4.22386 13 4.5C13 4.77614 12.7761 5 12.5 5H2.5C2.22386 5 2 4.77614 2 4.5ZM2 7.5C2 7.22386 2.22386 7 2.5 7H7.5C7.77614 7 8 7.22386 8 7.5C8 7.77614 7.77614 8 7.5 8H2.5C2.22386 8 2 7.77614 2 7.5ZM2 10.5C2 10.2239 2.22386 10 2.5 10H10.5C10.7761 10 11 10.2239 11 10.5C11 10.7761 10.7761 11 10.5 11H2.5C2.22386 11 2 10.7761 2 10.5Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
pub fn TextAlignCenterIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M2 4.5C2 4.22386 2.22386 4 2.5 4H12.5C12.7761 4 13 4.22386 13 4.5C13 4.77614 12.7761 5 12.5 5H2.5C2.22386 5 2 4.77614 2 4.5ZM4 7.5C4 7.22386 4.22386 7 4.5 7H10.5C10.7761 7 11 7.22386 11 7.5C11 7.77614 10.7761 8 10.5 8H4.5C4.22386 8 4 7.77614 4 7.5ZM3 10.5C3 10.2239 3.22386 10 3.5 10H11.5C11.7761 10 12 10.2239 12 10.5C12 10.7761 11.7761 11 11.5 11H3.5C3.22386 11 3 10.7761 3 10.5Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
pub fn TextAlignRightIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M2 4.5C2 4.22386 2.22386 4 2.5 4H12.5C12.7761 4 13 4.22386 13 4.5C13 4.77614 12.7761 5 12.5 5H2.5C2.22386 5 2 4.77614 2 4.5ZM7 7.5C7 7.22386 7.22386 7 7.5 7H12.5C12.7761 7 13 7.22386 13 7.5C13 7.77614 12.7761 8 12.5 8H7.5C7.22386 8 7 7.77614 7 7.5ZM4 10.5C4 10.2239 4.22386 10 4.5 10H12.5C12.7761 10 13 10.2239 13 10.5C13 10.7761 12.7761 11 12.5 11H4.5C4.22386 11 4 10.7761 4 10.5Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
pub fn ToolbarDemo() -> impl IntoView {
  view! {
    <ToolbarRoot
      attr:class="flex p-[10px] w-full min-w-max rounded-md bg-white shadow-[0_2px_10px] shadow-blackA4"
      attr:aria-label="Formatting options"
    >
      <ToolbarToggleGroup
        kind=ToggleGroupKind::Multiple {
            value: None,
            default_value: None,
            on_value_change: None,
        }

        attr:aria-label="Text formatting"
      >
        <ToolbarToggleItem
          attr:class="flex-shrink-0 mr-0.5 flex-grow-0 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="bold".into()
          attr:aria-label="Bold"
        >
          <FontBoldIcon/>
        </ToolbarToggleItem>
        <ToolbarToggleItem
          attr:class="flex-shrink-0 mr-0.5 flex-grow-0 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="italic".into()
          attr:aria-label="Italic"
        >
          <FontItalicIcon/>
        </ToolbarToggleItem>
        <ToolbarToggleItem
          attr:class="flex-shrink-0 flex-grow-0 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="strikethrough".into()
          attr:aria-label="Strike through"
        >
          <StrikethroughIcon/>
        </ToolbarToggleItem>
      </ToolbarToggleGroup>
      <ToolbarSeparator attr:class="w-[1px] bg-mauve6 mx-[10px]"/>
      <ToolbarToggleGroup
        kind=ToggleGroupKind::Single {
            value: None,
            default_value: Some("center".into()),
            on_value_change: None,
        }

        attr:aria-label="Text alignment"
      >
        <ToolbarToggleItem
          attr:class="flex-shrink-0 flex-grow-0 mr-0.5 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="left".into()
          attr:aria-label="Left aligned"
        >
          <TextAlignLeftIcon/>
        </ToolbarToggleItem>
        <ToolbarToggleItem
          attr:class="flex-shrink-0 flex-grow-0 mr-0.5 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="center".into()
          attr:aria-label="Center aligned"
        >
          <TextAlignCenterIcon/>
        </ToolbarToggleItem>
        <ToolbarToggleItem
          attr:class="flex-shrink-0 flex-grow-0 basis-auto text-mauve11 h-[25px] px-[5px] rounded inline-flex text-[13px] leading-none items-center justify-center bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
          value="right".into()
          attr:aria-label="Right aligned"
        >
          <TextAlignRightIcon/>
        </ToolbarToggleItem>
      </ToolbarToggleGroup>
      <ToolbarSeparator attr:class="w-[1px] bg-mauve6 mx-[10px]"/>
      <ToolbarLink
        attr:class="bg-transparent mr-[4px] text-mauve11 hidden sm:inline-flex justify-center items-center hover:bg-transparent hover:cursor-pointer flex-shrink-0 flex-grow-0 basis-auto h-[25px] px-[5px] rounded text-[13px] leading-none bg-white ml-0.5 outline-none hover:bg-violet3 hover:text-violet11 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7 first:ml-0 data-[state=on]:bg-violet5 data-[state=on]:text-violet11"
        attr:href="#"
        attr:target="_blank"
      >
        "Edited 2 hours ago"
      </ToolbarLink>
      <ToolbarButton attr:class="px-[10px] text-white ml-auto bg-violet9 flex-shrink-0 flex-grow-0 basis-auto h-[25px] rounded inline-flex text-[13px] leading-none items-center justify-center outline-none hover:bg-violet10 focus:relative focus:shadow-[0_0_0_2px] focus:shadow-violet7">
        "Share"
      </ToolbarButton>
    </ToolbarRoot>
  }
}

#[component]
fn FontBoldIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M5.10505 12C4.70805 12 4.4236 11.912 4.25171 11.736C4.0839 11.5559 4 11.2715 4 10.8827V4.11733C4 3.72033 4.08595 3.43588 4.25784 3.26398C4.43383 3.08799 4.71623 3 5.10505 3C6.42741 3 8.25591 3 9.02852 3C10.1373 3 11.0539 3.98153 11.0539 5.1846C11.0539 6.08501 10.6037 6.81855 9.70327 7.23602C10.8657 7.44851 11.5176 8.62787 11.5176 9.48128C11.5176 10.5125 10.9902 12 9.27734 12C8.77742 12 6.42626 12 5.10505 12ZM8.37891 8.00341H5.8V10.631H8.37891C8.9 10.631 9.6296 10.1211 9.6296 9.29877C9.6296 8.47643 8.9 8.00341 8.37891 8.00341ZM5.8 4.36903V6.69577H8.17969C8.53906 6.69577 9.27734 6.35939 9.27734 5.50002C9.27734 4.64064 8.48047 4.36903 8.17969 4.36903H5.8Z"
        fill="currentColor"
      ></path>
    </svg>
  }
}

#[component]
fn FontItalicIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M5.67494 3.50017C5.67494 3.25164 5.87641 3.05017 6.12494 3.05017H10.6249C10.8735 3.05017 11.0749 3.25164 11.0749 3.50017C11.0749 3.7487 10.8735 3.95017 10.6249 3.95017H9.00587L7.2309 11.05H8.87493C9.12345 11.05 9.32493 11.2515 9.32493 11.5C9.32493 11.7486 9.12345 11.95 8.87493 11.95H4.37493C4.1264 11.95 3.92493 11.7486 3.92493 11.5C3.92493 11.2515 4.1264 11.05 4.37493 11.05H5.99397L7.76894 3.95017H6.12494C5.87641 3.95017 5.67494 3.7487 5.67494 3.50017Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn StrikethroughIcon() -> impl IntoView {
  view! {
    <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        d="M5.00003 3.25C5.00003 2.97386 4.77617 2.75 4.50003 2.75C4.22389 2.75 4.00003 2.97386 4.00003 3.25V7.10003H2.49998C2.27906 7.10003 2.09998 7.27912 2.09998 7.50003C2.09998 7.72094 2.27906 7.90003 2.49998 7.90003H4.00003V8.55C4.00003 10.483 5.56703 12.05 7.50003 12.05C9.43303 12.05 11 10.483 11 8.55V7.90003H12.5C12.7209 7.90003 12.9 7.72094 12.9 7.50003C12.9 7.27912 12.7209 7.10003 12.5 7.10003H11V3.25C11 2.97386 10.7762 2.75 10.5 2.75C10.2239 2.75 10 2.97386 10 3.25V7.10003H5.00003V3.25ZM5.00003 7.90003V8.55C5.00003 9.93071 6.11932 11.05 7.50003 11.05C8.88074 11.05 10 9.93071 10 8.55V7.90003H5.00003Z"
        fill="currentColor"
        fill-rule="evenodd"
        clip-rule="evenodd"
      ></path>
    </svg>
  }
}

#[component]
fn RadioGroupDemo() -> impl IntoView {
  view! {
    <form>
      <RadioGroupRoot
        attr:class="flex flex-col gap-2.5"
        default_value="default".into()
        attr:aria-label="View density"
      >
        <div class="flex items-center">
          <RadioGroupItem
            attr:class="bg-white w-[25px] h-[25px] rounded-full shadow-[0_2px_10px] shadow-blackA4 hover:bg-violet3 focus:shadow-[0_0_0_2px] focus:shadow-black outline-none cursor-default"
            value="default".into()
            attr:id="r1"
          >
            <RadioGroupIndicator attr:class="flex items-center justify-center w-full h-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-violet11"/>
          </RadioGroupItem>
          <label class="dark:text-white text-[15px] leading-none pl-[15px]" for="r1">
            Default
          </label>
        </div>
        <div class="flex items-center">
          <RadioGroupItem
            attr:class="bg-white w-[25px] h-[25px] rounded-full shadow-[0_2px_10px] shadow-blackA4 hover:bg-violet3 focus:shadow-[0_0_0_2px] focus:shadow-black outline-none cursor-default"
            value="comfortable".into()
            attr:id="r2"
          >
            <RadioGroupIndicator attr:class="flex items-center justify-center w-full h-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-violet11"/>
          </RadioGroupItem>
          <label class="dark:text-white text-[15px] leading-none pl-[15px]" for="r2">
            Comfortable
          </label>
        </div>
        <div class="flex items-center">
          <RadioGroupItem
            attr:class="bg-white w-[25px] h-[25px] rounded-full shadow-[0_2px_10px] shadow-blackA4 hover:bg-violet3 focus:shadow-[0_0_0_2px] focus:shadow-black outline-none cursor-default"
            value="compact".into()
            attr:id="r3"
          >
            <RadioGroupIndicator attr:class="flex items-center justify-center w-full h-full relative after:content-[''] after:block after:w-[11px] after:h-[11px] after:rounded-[50%] after:bg-violet11"/>
          </RadioGroupItem>
          <label class="dark:text-white text-[15px] leading-none pl-[15px]" for="r3">
            Compact
          </label>
        </div>
      </RadioGroupRoot>
    </form>
  }
}

#[component]
fn SwitchDemo() -> impl IntoView {
  view! {
    <form>
      <div class="flex items-center">
        <label
          class="mauve-11 dark:text-white text-[15px] leading-none pr-[15px]"
          for="airplane-mode"
        >
          "Airplane mode"
        </label>
        <SwitchRoot
          attr:class="transition w-[42px] h-[25px] bg-blackA6 rounded-full relative shadow-[0_2px_10px] shadow-blackA4 focus:shadow-[0_0_0_2px] focus:shadow-black data-[state=checked]:bg-black/75 outline-none cursor-default"
          attr:id="airplane-mode"
          attr:style="-webkit-tap-highlight-color: rgba(0, 0, 0, 0)"
        >
          <SwitchThumb attr:class="block w-[21px] h-[21px] bg-white rounded-full shadow-[0_2px_2px] shadow-blackA4 transition-transform duration-100 translate-x-0.5 will-change-transform data-[state=checked]:translate-x-[19px]"/>
        </SwitchRoot>
      </div>
    </form>
  }
}

#[component]
fn TabsDemo() -> impl IntoView {
  view! {
    <TabsRoot
      attr:class="flex flex-col w-[300px] shadow-[0_2px_10px] shadow-blackA2"
      default_value="tab1".into()
    >
      <TabsList
        attr:class="shrink-0 flex border-b border-mauve6"
        attr:aria-label="Manage your account"
      >
        <TabsTrigger
          attr:class="bg-white px-5 h-[45px] flex-1 flex items-center justify-center text-[15px] leading-none text-mauve11 select-none first:rounded-tl-md last:rounded-tr-md hover:text-violet11 data-[state=active]:text-violet11 data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current data-[state=active]:focus:relative data-[state=active]:focus:shadow-[0_0_0_2px] data-[state=active]:focus:shadow-black outline-none cursor-default"
          value="tab1".into()
        >
          "Account"
        </TabsTrigger>
        <TabsTrigger
          attr:class="bg-white px-5 h-[45px] flex-1 flex items-center justify-center text-[15px] leading-none text-mauve11 select-none first:rounded-tl-md last:rounded-tr-md hover:text-violet11 data-[state=active]:text-violet11 data-[state=active]:shadow-[inset_0_-1px_0_0,0_1px_0_0] data-[state=active]:shadow-current data-[state=active]:focus:relative data-[state=active]:focus:shadow-[0_0_0_2px] data-[state=active]:focus:shadow-black outline-none cursor-default"
          value="tab2".into()
        >
          "Password"
        </TabsTrigger>
      </TabsList>
      <TabsContent
        attr:class="grow p-5 bg-white rounded-b-md outline-none focus:shadow-[0_0_0_2px] focus:shadow-black"
        value="tab1".into()
      >
        <p class="mb-5 text-mauve11 text-[15px] leading-normal">
          "Make changes to your account here. Click save when you're done."
        </p>
        <fieldset class="mb-[15px] w-full flex flex-col justify-start">
          <label class="text-[13px] leading-none mb-2.5 text-violet12 block" for="name">
            "Name"
          </label>
          {|| {
              let name_node_ref = NodeRef::<Input>::new();
              name_node_ref
                  .on_load(|node| {
                      node.set_default_value("Pedro Duarte");
                  });
              view! {
                <input
                  class="grow shrink-0 rounded px-2.5 text-[15px] leading-none text-violet11 shadow-[0_0_0_1px] shadow-violet7 h-[35px] focus:shadow-[0_0_0_2px] focus:shadow-violet8 outline-none"
                  id="name"
                  node_ref=name_node_ref
                />
              }
          }}

        </fieldset>
        <fieldset class="mb-[15px] w-full flex flex-col justify-start">
          <label class="text-[13px] leading-none mb-2.5 text-violet12 block" for="username">
            "Username"
          </label>

          {|| {
              let username_node_ref = NodeRef::<Input>::new();
              username_node_ref
                  .on_load(|node| {
                      node.set_default_value("@peduarte");
                  });
              view! {
                <input
                  class="grow shrink-0 rounded px-2.5 text-[15px] leading-none text-violet11 shadow-[0_0_0_1px] shadow-violet7 h-[35px] focus:shadow-[0_0_0_2px] focus:shadow-violet8 outline-none"
                  id="username"
                  node_ref=username_node_ref
                />
              }
          }}

        </fieldset>
        <div class="flex justify-end mt-5">
          <button class="inline-flex items-center justify-center rounded px-[15px] text-[15px] leading-none font-medium h-[35px] bg-green4 text-green11 hover:bg-green5 focus:shadow-[0_0_0_2px] focus:shadow-green7 outline-none cursor-default">
            "Save changes"
          </button>
        </div>
      </TabsContent>
      <TabsContent
        attr:class="grow p-5 bg-white rounded-b-md outline-none focus:shadow-[0_0_0_2px] focus:shadow-black"
        value="tab2".into()
      >
        <p class="mb-5 text-mauve11 text-[15px] leading-normal">
          "Change your password here. After saving, you'll be logged out."
        </p>
        <fieldset class="mb-[15px] w-full flex flex-col justify-start">
          <label class="text-[13px] leading-none mb-2.5 text-violet12 block" for="currentPassword">
            "Current password"
          </label>
          <input
            class="grow shrink-0 rounded px-2.5 text-[15px] leading-none text-violet11 shadow-[0_0_0_1px] shadow-violet7 h-[35px] focus:shadow-[0_0_0_2px] focus:shadow-violet8 outline-none"
            id="currentPassword"
            type="password"
          />
        </fieldset>
        <fieldset class="mb-[15px] w-full flex flex-col justify-start">
          <label class="text-[13px] leading-none mb-2.5 text-violet12 block" for="newPassword">
            "New password"
          </label>
          <input
            class="grow shrink-0 rounded px-2.5 text-[15px] leading-none text-violet11 shadow-[0_0_0_1px] shadow-violet7 h-[35px] focus:shadow-[0_0_0_2px] focus:shadow-violet8 outline-none"
            id="newPassword"
            type="password"
          />
        </fieldset>
        <fieldset class="mb-[15px] w-full flex flex-col justify-start">
          <label class="text-[13px] leading-none mb-2.5 text-violet12 block" for="confirmPassword">
            "Confirm password"
          </label>
          <input
            class="grow shrink-0 rounded px-2.5 text-[15px] leading-none text-violet11 shadow-[0_0_0_1px] shadow-violet7 h-[35px] focus:shadow-[0_0_0_2px] focus:shadow-violet8 outline-none"
            id="confirmPassword"
            type="password"
          />
        </fieldset>
        <div class="flex justify-end mt-5">
          <button class="inline-flex items-center justify-center rounded px-[15px] text-[15px] leading-none font-medium h-[35px] bg-green4 text-green11 hover:bg-green5 focus:shadow-[0_0_0_2px] focus:shadow-green7 outline-none cursor-default">
            "Change password"
          </button>
        </div>
      </TabsContent>
    </TabsRoot>
  }
}

#[component]
fn SliderDemo() -> impl IntoView {
  view! {
    <form>
      <SliderRoot
        attr:class="relative flex items-center select-none touch-none w-[200px] h-5"
        default_value=vec![50.0f64].into()
        max=100.0.into()
        step=1.0.into()
      >
        <SliderTrack attr:class="bg-blackA7 relative grow rounded-full h-[3px]">
          <SliderRange attr:class="absolute bg-white rounded-full h-full">
            {().into_view()}
          </SliderRange>
        </SliderTrack>
        <SliderThumb
          attr:class="block w-5 h-5 bg-white shadow-[0_2px_10px] shadow-blackA4 rounded-[10px] hover:bg-violet3 focus:outline-none focus:shadow-[0_0_0_5px] focus:shadow-blackA5"
          attr:aria-label="Volume"
        >
          {().into_view()}
        </SliderThumb>
      </SliderRoot>
    </form>
  }
}

#[component]
fn ScrollAreaDemo() -> impl IntoView {
  let tags = (1..=50)
    .rev()
    .map(|num| format!("v1.2.0-beta.{num}"))
    .collect::<Vec<_>>();

  view! {
    <ScrollAreaRoot attr:class="w-[200px] h-[225px] rounded overflow-hidden shadow-[0_2px_10px] shadow-blackA4 bg-white">
      <ScrollAreaViewport attr:class="w-full h-full rounded">
        <div class="py-[15px] px-5">
          <div class="text-violet11 text-[15px] leading-[18px] font-medium">"Tags"</div>
          <For each=move || tags.clone() key=|n| n.clone() let:data>
            <div class="text-mauve12 text-[13px] leading-[18px] mt-2.5 pt-2.5 border-t border-t-mauve6">
              {data}
            </div>
          </For>
        </div>
      </ScrollAreaViewport>
      <ScrollAreaScrollbar
        attr:class="flex select-none touch-none p-0.5 bg-blackA3 transition-colors duration-[160ms] ease-out hover:bg-blackA5 data-[orientation=vertical]:w-2.5 data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-2.5"
        orientation=Orientation::Vertical.into()
      >
        <ScrollAreaThumb attr:class="flex-1 bg-mauve10 rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]"/>
      </ScrollAreaScrollbar>
      <ScrollAreaScrollbar
        attr:class="flex select-none touch-none p-0.5 bg-blackA3 transition-colors duration-[160ms] ease-out hover:bg-blackA5 data-[orientation=vertical]:w-2.5 data-[orientation=horizontal]:flex-col data-[orientation=horizontal]:h-2.5"
        orientation=Orientation::Horizontal.into()
      >
        <ScrollAreaThumb attr:class="flex-1 bg-mauve10 rounded-[10px] relative before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:w-full before:h-full before:min-w-[44px] before:min-h-[44px]"/>
      </ScrollAreaScrollbar>
      <ScrollAreaCorner attr:class="bg-blackA5"/>
    </ScrollAreaRoot>
  }
}
