# Contributing to Leptos Primitives
Contributions are of course always welcome. Unfortunately web technology is volatile and sometimes lacks desired 
features/behavior that you have to end up implementing yourself. 

## Guidelines
There aren't any super strict or formal guidelines, the only things we ask of you are the following:
* Prefer idiomatic Rust and patterns; as it usually describes your code's intent the most universally.
* Stay consistent and concise; try not to make things more complicated than they need to be.
* Be human; we're all just trying to build cool things using awesome tools. Don't be afraid to share either!

## Working Locally
See our [example](https://github.com/Upbolt/leptos_primitives/examples/csr-with-tailwind) (with TailwindCSS)

There's probably a better way of handling the resulting CSS file from Tailwind, but what I currently do is:

(Assuming your current directory is already in leptos_primitives)

1. ```cd examples/csr-with-tailwind``` (for all three processes)
2. ```trunk serve``` (in one terminal process)
3. ```npx tailwindcss -i styles/input.css -o styles/output.css --watch``` (in another terminal process)
4. ```cd styles && miniserve . -p 8081``` (in yet another terminal process)

Find [miniserve](https://github.com/svenstaro/miniserve) here

(If you have a better way of handling styles instead of using miniserve, don't hesitate to let me know in an issue!)

## Preparing Pull Requests
Prefer many small commits/PRs describing changes rather than submitting single large changes, this makes it easier for
everybody to digest.
