<p align="center" dir="auto">
    <img src="assets/logo.svg" alt="logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">Leptix</h1>

<p align="center">
    Accessible components for Leptos
</p>

<p align="center" dir="auto">
    <img src="assets/early_dev.svg" alt="This library is in early development."/>
</p>

<p align="center">
    <a href="https://upbolt.github.io/leptos_primitives">Live Demo</a>
</p>

<hr />

## Installation

```toml
# Add the following to your Cargo.toml file under [dependencies]

[dependencies]
# ...
leptix_primitives = { git = "https://github.com/leptix/leptix.git" }
```

## Server-Side Rendering

Enable the `ssr` feature flag under your project's `features` section

```toml
[features]
csr = ...
hydrate = ...
ssr = [
  "leptix_primitives/ssr",

  # the rest of your leptos ssr dependencies ...
  "leptos/ssr",
  "dep:leptox_actix",
  ...
]
```

## Available Components

Note: Dialogs and components that require floating functionality are not yet implemented; tracking issues for them can be found [here](https://github.com/leptix/leptix/issues/4) and [here](https://github.com/leptix/leptix/issues/2) (respectively)

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
use leptix_primitives::components::checkbox::{CheckboxIndicator, CheckboxRoot, CheckedState};

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

## Examples

- [Trunk + TailwindCSS](https://github.com/leptix/leptix/tree/master/examples/csr-with-tailwind)

- [Actix + TailwindCSS](https://github.com/leptix/leptix/tree/master/examples/ssr-with-actix-tailwind)

- [Axum + TailwindCSS](https://github.com/leptix/leptix/tree/master/examples/ssr-with-axum-tailwind)

## Contributing

See [`CONTRIBUTING.md`](/CONTRIBUTING.md) for details on what you should know before you send in pull requests.
