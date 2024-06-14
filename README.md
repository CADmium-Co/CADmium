# CADmium

This project aims to create a new CAD program from scratch. It is small, it runs in a web browser, and the source code is available for free here on Github.

Legacy CAD programs have taken many thousands of years of collective engineering time to get where they are so this program will never be able to compete on breadth of features. But CADmium is intended to capture 80% of the most common CAD use cases while doing less than 10% of the work. For now we are targeting the home hobbyist who just wants to design a widget for their 3D printer, not a company that wants to design a car or airplane, although that will come later.

If you're looking for:

- A simple, modern, parametric CAD UI that runs in a browser
- That can export solids as .step, .obj, or .cadmium (a json-based CAD format that this project is inventing)
- That can export sketches as .svg or .dxf
- That works without an internet connection

Then this project may be for you!

**Status**: Early prototype. This tool is not yet an MVP, but is being developed in the open. ~~Please do not share this to HN or Reddit or things like that.~~ ha, well I guess [that ship has sailed](https://news.ycombinator.com/item?id=40428827)!

## Overall Plan

**Demos:** We are currently racing toward our first demo release, [V0.0.1](https://github.com/orgs/CADmium-Co/projects/1?pane=info). This is a good first exercise for us to decide on build and release processes.

After that we will do a few more demo (V0.0.\*) releases, aggregating features until it feels pretty usable.

**Alpha:** When it feels like we've reached an MVP that people might actually want to use, it's time to release an Alpha version (V0.1.0) and actively solicit feedback from users. We'll use that feedback to make more improvements, re-inventing things if necessary to achieve a great workflow.

Beyond that, we'll see!

## Technology

The boundary representation engine under the hood is [truck](https://github.com/ricosjp/truck), which is written in rust and is not dependent on any legacy b-rep engine.

Leveraging truck, we wrote a small rust library called [cadmium](https://github.com/CADmium-Co/CADmium/tree/main/src/rust/cadmium) which provides structs for projects, workspaces, sketches, extrusions, and constraints. Our goal is that this rust library provides all the same functionality as the UI for anyone who prefers code-first CAD. This library is able to save and load projects to disk as json. We have also built a set of javascript bindings so that the whole thing can be compiled to wasm and run in a browser.

The UI is built with [SvelteKit](https://kit.svelte.dev/) and [Tailwind](https://tailwindcss.com/). It is hosted with Github Pages. We use [three.js](https://threejs.org/) for rendering, which in this case uses WebGL under the hood. We use [Threlte](https://github.com/threlte/threlte) to manage the scene graph declaratively.

Native builds use [Tauri](https://tauri.app/), which is a Rust-based wrapper around OS-specific native webviews that allows us to build a native app from the same codebase.

## License

This software is offered under the [Elastic License 2.0](https://www.elastic.co/licensing/elastic-license). In summary, you can do whatever you like with this software except offer it as a service to third parties.

## Running The Code

If you're just trying to kick the tires, [click here](https://CADmium-Co.github.io/CADmium/) to view the live web demo.

To build locally using pnpm workspace & turbo:

```shell
git clone https://github.com/Cadmium-Co/CADmium.git
cd CADmium
pnpm install
pnpm dev
```

### Native Builds

```shell
# Development
pnpm tauri dev

# Generate binaries and installers
pnpm tauri build
```

Tauri can [generate icons](https://tauri.app/v1/guides/features/icons/) for the native build with the following command:

```shell
pnpm tauri icon applications/web/public/cadmium_logo_min.svg
```

## Tooling setup

### pnpm

We use pnpm to manage the monorepo. Please follow the instructions here to install: https://pnpm.io/installation#using-a-standalone-script

If you're new to node you can use pnpm to manage nodejs:

```shell
# https://pnpm.io/cli/env#use
pnpm env use --global 20
```

### rust

First install rust using rustup: https://rustup.rs

Then install wasm-pack

```shell
cargo install wasm-pack
```

## Running Tests

```shell
pnpm test
```

Playwright is used for e2e testing. You may be prompted with a command to install it.

For manjaro/archlinux folks it may report missing dependencies. On manjaro the missing dependencies are solved [thanks to this comment](https://github.com/microsoft/playwright/issues/2621#issuecomment-931530175):

```shell
yay -S aur/enchant1.6 aur/icu66 aur/libwebp052
```

Watch vitest unit tests only:

```shell
cd applications/web
pnpm test:unit -w
```

### rust

To build and run the Rust tests:

```shell
cargo test
```

### rust examples

Simple exaples using the rust code can be found in `packages/cadmium/examples`

Run simple rust example with:
```
cargo run --example project_simple_extrusion
```

Will produce example.obj file and example.step output files, the .step file can be examined in a CAD viewer.

## git blame

To ignore commits used purely for formatting changes, to preserve correct authorship, set your local git config:

```shell
git config blame.ignoreRevsFile .git-blame-ignore-revs
```

## Contributing

We are actively seeking contributors! Please join the [Discord](https://discord.gg/qJCsKJeyZv) and come help out!

Most especially, we need help in the following areas:

**Design:** The tool must look and feel good and we are not designers. We would love contributions in the form of:

- Advice, mockups, or tailwindcss examples of how to make different elements look and behave better
- In particular, help picking a color palette that works well and is unique
- Help figuring out how to implement dark mode

**Rust:** This is our first project in Rust. We need help from experienced Rustaceans to:

- Figure out how to better lay out the rust code
- Point out any glaring issues with how I'm using the language (We've thus far completely avoided Lifetimes, Traits, Rc, RefCell, etc and that may be hampering things)

**Svelte:** This is our first project using Svelte. We'd love an experienced set of eyes to:

- Look over the basic structure and tell us if we're making any big mistakes
- Help us migrate to Svelte 5 when it comes out

If you feel like you would be willing and able to help, please join [our discord](https://discord.gg/qJCsKJeyZv)!

## Immediate TODOs for V0.0.1 release (The Demo)

Github project for tracking progress is [here](https://github.com/orgs/CADmium-Co/projects/1/views/1?pane=info)

- [ ] Sketching
  - [ ] Implement a standalone first-order 2D constraint solver
  - [ ] Integrate that new solver into sketch.rs
  - [ ] Ability to create a sketch on the face of a solid
  - [ ] Ability to create and modify constraints in the UI
  - [ ] Entities: line, rect, circle, arc, 2D fillet
  - [ ] Constraints: horiz, vert, parallel, perp, length, radius, coincident, maybe a few others
- [ ] Extrusion
  - [ ] Configure an extrusion to create new solid or subtract from existing solid
  - [ ] Modes: New, Add, Cut
  - [ ] Control: depth, offset
  - [ ] Ability to extrude multiple faces (from one sketch) at a time while not extruding every face in the sketch
- [ ] Revolution
  - [ ] Same as Extrusions
- [ ] Boolean
  - [ ] Union, Intersection, Subtraction
- [ ] Project
  - [x] Ability to rename the project
  - [ ] Ability to delete steps
  - [ ] bind ctrl + s to .cadmium export, and ctrl + o to .cadmium import
- [ ] Units
  - [ ] Make it clear that the whole file uses millimeter units
- [ ] Files
  - [ ] Save and load CADmium files
  - [ ] Export as STEP, OBJ, STL, SVG, DXF
- [ ] Marketing
  - [ ] Youtube video demonstrating how to make:
    - [ ] A simple cube
    - [ ] A plate with screw holes
    - [ ] Something pretty complex [like this](https://cad.onshape.com/documents/2382065421b11eec78db785a/w/3a8d47849c9b8d3e2c1584d6/e/63184eaca6140916d236e50b?renderMode=0&uiState=66507497cd1f34699629cbb5)
