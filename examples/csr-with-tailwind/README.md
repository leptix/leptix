<p align="center" dir="auto">
    <img src="assets/logos.svg"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">Leptos Primitives (CSR Tailwind Example)</h1>

<p align="center" dir="auto">
    This example showcases every currently implemented component using TailwindCSS for styling
</p>

<hr />

## How to run

1. Clone this repository

```
git clone https://github.com/Upbolt/leptos_primitives.git
cd leptos_primitives/examples/csr-with-tailwind
```

2. To use TailwindCSS, you'll need [Node.js](https://nodejs.org) installed (npm & npx)

```
npm install
```

3. Now at this point I'd like to admit this is probably not the best solution, but what I do is create 3 terminal processes and run these in each (if you have a better method, please let me know!)

```
trunk serve
```

```
npx tailwindcss -i styles/input.css -o styles/output.css
```

```
cd styles && miniserve . -p 8081
```

The last terminal command requires [miniserve](https://github.com/svenstaro/miniserve)
