<p align="center" dir="auto">
    <img src="assets/logo.svg"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">Leptos Primitives</h1>

<p align="center">
    Accessible and unstyled components for Leptos
</p>

<p align="center" dir="auto">
    <img src="assets/early_dev.svg"/>
</p>

<hr />

## What is this crate?

leptos_primitives is a port of [radix-ui's primitives component library](https://github.com/radix-ui/primitives) for
the [leptos](https://github.com/leptos-rs/leptos) full-stack web framework; everything is
essentially ported one-to-one.

## Installation

leptos_primitives is not the final decided name of this library, so it's yet to be uploaded to [crates.io](https://crates.io)

```toml
# Add the following to your Cargo.toml file under [dependencies]

[dependencies]
# ...
leptos_primitives = { git = "https://github.com/Upbolt/leptos_primitives.git" }
```

## Available Components

| Component Name |
| -------------- |
| Accordion      |
| AspectRatio    |
| Avatar         |
| Checkbox       |
| Collapsible    |
| Label          |
| Progress       |
| RadioGroup     |
| ScrollArea     |
| Separator      |
| Slider         |
| Switch         |
| Tabs           |
| Toggle         |
| ToggleGroup    |
| Toolbar        |

## Usage

These small snippets have been ported one-to-one from radix-ui's documentation site, so where you would have this in JavaScript:

```jsx
import React from "react";
import * as Checkbox from "@radix-ui/react-checkbox";
import { CheckIcon } from "@radix-ui/react-icons";

const CheckboxDemo = () => (
	<form>
		<div className="flex items-center">
			<Checkbox.Root
				className="shadow-blackA4 hover:bg-violet3 flex h-[25px] w-[25px] appearance-none items-center justify-center rounded-[4px] bg-white shadow-[0_2px_10px] outline-none focus:shadow-[0_0_0_2px_black]"
				defaultChecked
				id="c1"
			>
				<Checkbox.Indicator className="text-violet11">
					<CheckIcon />
				</Checkbox.Indicator>
			</Checkbox.Root>
			<label
				className="pl-[15px] text-[15px] leading-none text-white"
				htmlFor="c1"
			>
				Accept terms and conditions.
			</label>
		</div>
	</form>
);
```

You would have this in Rust using Leptos:

```rust
use leptos::*;
use leptos_primitives::components::checkbox::{CheckboxIndicator, CheckboxRoot, CheckedState};

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
```

You may see the full example with all available components right below.

## Example

See the full client-side rendered example using TailwindCSS [here](https://github.com/Upbolt/leptos_primitives/tree/master/examples/csr-with-tailwind)

## Contributing

See [`CONTRIBUTING.md`](/CONTRIBUTING.md) for details on what you should know before you send in pull requests.
