# CADmium

This is an attempt to create a new CAD program from scratch. Legacy CAD programs have taken many thousands of years of collective engineering time to get where they are so this program will never be able to compete on breadth of features. CADmium is intended to capture 80% of the most common CAD use cases while doing less than 20% of the work.

Features:
- Simple, modern parametric CAD UI
- Runs in a browser
- Export as STEP, STL, or OBJ
- Functions without an internet connection (once you've loaded the page)

**Status**: Early prototype. This tool is not yet minimally functional, but is being developed in the open.

## Technology

The boundary representation engine under the hood is [truck](https://github.com/ricosjp/truck), which is written in rust and is not dependent on any legacy b-rep engine.

Leveraging truck, I wrote a small rust library called [cadmium](https://github.com/MattFerraro/CADmium/tree/main/src/rust/cadmium) which provides structs for projects, workspaces, sketches, extrusions, and constraints. My goal is that this rust library provides all the same functionality as the UI for anyone who prefers code-first CAD. This library is able to save and load projects to disk as json. I have also built a set of javascript bindings so that the whole thing can be compiled to wasm and run in a browser.

The UI is built with SvelteKit and Tailwind. It is [hosted](https://cadmium-nine.vercel.app/tailwind) with Vercel. I use [three.js](https://threejs.org/) for rendering, which in this case uses WebGL under the hood.

## Licensing

The entire project is available under the MIT license.