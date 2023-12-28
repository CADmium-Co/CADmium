# CADmium

This project aims to create a new CAD program from scratch. It is small, it runs in a web browser, and the source code is available for free here on Github.

Legacy CAD programs have taken many thousands of years of collective engineering time to get where they are so this program will never be able to compete on breadth of features. But CADmium is intended to capture 80% of the most common CAD use cases while doing less than 10% of the work. For now I am targetting the home hobbyist who just wants to design a widget for their 3D printer, not a company that wants to design a car or airplane.

If you're looking for:

- A simple, modern, parametric CAD UI that runs in a browser
- That can export solids as .step, .obj, or .cadmium (a json-based CAD format that this project is inventing)
- That can export sketches as .svg or .dxf
- That works without an internet connection

Then this project may be for you!

**Status**: Early prototype. This tool is not yet an MVP, but is being developed in the open.

## Overall Plan

I am currently racing toward an MVP, which I expect to release as version 0.1 in February, 2024. After that I hope to get a small group of users together to help guide me toward a version 1.0 release which will be more fully featured and which may serve as a solid foundation for hobbyists: all of it free, source available, and local-only.

Beyond that, I will try to monetize by offering a hosted version of the software as a paid product. To achieve this I will introduce a notion of users, accounts, sharing, and a public commons. All of the actual CAD functionality, including the ability to read and modify .cadmium files, will always be available for free here on Github in a local-only form.

## Licensing

The entire project is available under the MIT license, but I can't guarantee that it will be forever. I may switch to an [FSL](https://fsl.software/) license soon.

## Technology

The boundary representation engine under the hood is [truck](https://github.com/ricosjp/truck), which is written in rust and is not dependent on any legacy b-rep engine.

Leveraging truck, I wrote a small rust library called [cadmium](https://github.com/MattFerraro/CADmium/tree/main/src/rust/cadmium) which provides structs for projects, workspaces, sketches, extrusions, and constraints. My goal is that this rust library provides all the same functionality as the UI for anyone who prefers code-first CAD. This library is able to save and load projects to disk as json. I have also built a set of javascript bindings so that the whole thing can be compiled to wasm and run in a browser.

The UI is built with [SvelteKit](https://kit.svelte.dev/) and [Tailwind](https://tailwindcss.com/). It is [hosted](https://cadmium-nine.vercel.app/tailwind) with Vercel. I use [three.js](https://threejs.org/) for rendering, which in this case uses WebGL under the hood. I use [Threlte](https://github.com/threlte/threlte) to manage the scene graph decaratively.

## Running The Code

If you're just trying to kick the tires, [click here](https://cadmium-nine.vercel.app/threlte) to view the live web demo.

To build locally:

```
git clone https://github.com/MattFerraro/CADmium.git
cd CADmium
npm run build:wasm
npm run dev
```

You will need rust and wasm-pack working locally. See `vercel_build.sh` for an example of installing these dependencies.

## Contributing

I am not currently able to handle contributions. After the 0.1 release in February I will be looking for help in a few areas:

**Design:** The tool must look and feel good and I am not a designer. I would love contributions in the form of:
- Advice, mockups, or tailwindcss examples of how to make different elements look and behave better.
- In particular, help picking a color palette that works well and is unique
- Help figuring out how to implement dark mode

**Rust:** This is my first project in rust. I need an experienced rustacean's help to:
  - Figure out how to better lay out my rust code so it is more modular. I currently have two huge files where almost all of the logic lives and that makes it difficult to develop
  - Point out any glaring issues with how I'm using the language
  - Give me general feedback on if the message passing implementation can be improved in some way
  - Finally explain to me how Lifetimes work

**Svelte:** This is my first project using Svelte. I'd love an experienced set of eyes to:
  - Look over the basic structure and tell me if I'm making any big mistakes
  - Help me migrate to Svelte 5 when it comes out

**Users:** I need to gather a small userbase of hobbyists and engineers who can use the tool to design a lot of actual parts to help me find
  - Bugs
  - Big opportunities for improvement
  - The limitations of my approach

**Business:** I have quit my job and I am working on this full time. I'd love your help if you know how to:
  - Start and administer a 1-person SAAS business
  - Generate income using a code base which is open source

If you feel like you would be willing and able to help, please join [my discord](https://discord.gg/MQbBNyNQFf)!

## TODO for MVP (0.1 Release)

- [ ] Sketching
  - [ ] Ability to create a new sketch on a plane
  - [ ] Ability to create a sketch on the face of a solid
  - [x] New Rectangle Tool
  - [x] Ability to select and delete points/lines/circles/constraints
  - [x] Bind (l) to line and (c) to circle and (r) to rectangle
  - [x] Allow snapping to Origin
  - [x] Adjust point snapping to be zoom invariant
  - [ ] Faster way to select lots of lines/arcs/circles
  - [x] Show line/circle/arc previews before committing to them
  - [ ] Ability to create and modify constraints
  - [ ] Automatic solving of constraints (hide the step/solve buttons)
  - [ ] Handle point on line constraints and resulting face geometry
  - [ ] Fix crash that occurs when two lines overlap but don't intersect
  - [ ] Allow some lines to be construction lines
- [ ] Extrusion
  - [ ] Ability to create a new extrusion and select faces for it
  - [ ] Configure an extrusion to subtract, add, or create new solids
  - [ ] Configure extrusion depth
  - [ ] Configure extrusion faces
  - [ ] Stabilize face identification so that it doesn't jump around when you make sketch changes
- [ ] Revolution
  - [ ] Same as extrusion
- [ ] Project
  - [ ] Ability to rename the project
  - [ ] Ability to create and delete entire workbenches
  - [ ] bind ctrl + s to .cadmium export, and ctrl + o to .cadmium import
  - [x] Default to completely empty project
  - [ ] Constantly save .cadmium file to localStorage, and offer to re-open the last saved project in case you accidentally close the tab
  - [ ] Natural UX for when a link truly breaks
  - [ ] Orientation cube in upper right (replace Gizmo)
  - [x] Buttons to zoom to Planes
- [x] Debug
  - [x] On wasm crash, show a helpful error in the console
  - [x] On wasm crash, include some way to emit the sequence of events that created the crash so I can easily repro errors that other generate
- [ ] TrackballControls
  - [ ] Zoom camera to cursor, not center
  - [ ] Fix pan speed being mismatched on x and y
- [ ] Export
  - [ ] Proper scaling on .obj and .step exports (currently the units are off)
  - [ ] .dxf export for sketches
- [ ] Units
  - [ ] Make it clear what units the whole file is in
- [ ] Marketing
  - [ ] Youtube video demonstrations
    - [ ] A simple cube
    - [ ] A plate with screw holes
    - [ ] Something pretty complex
  - [ ] A nice website
    - [ ] With Screenshots
    - [ ] Clear call to action
    - [ ] Show the Youtube videos
    - [ ] Create a Patreon or something
  - [ ] Social Media
    - [ ] Twitter Account
    - [ ] Discord

## TODO for Actual Product (1.0 Release)

- [ ] Sketching
  - [ ] Mirror
  - [ ] Fillet
  - [ ] Snip
  - [ ] Circular Pattern
  - [ ] Rectangular Pattern
  - [ ] Center Point Rectangle
  - [ ] Ellipse
  - [ ] 3 Point Circle
  - [ ] Convert to/from construction lines
  - [ ] Project points/lines from Solids
  - [ ] "Smart Constraints" which can be toggled on or off as you draw
- [ ] Extrude
  - [ ] Up to face
  - [ ] With draft angle
- [ ] Solids
  - [ ] Boolean (or Union, Intersection, Subtract)
  - [ ] Mirror
  - [ ] Delete
  - [ ] Transform (or Rotate and Translate?)
  - [ ] Chamfer
  - [ ] Fillet
- [ ] Assemblies
  - [ ] Fixed joint
  - [ ] Rotational joint
  - [ ] Translational joint
  - [ ] Cylindrical joint
  - [ ] Ball joint
  - [ ] Interference Detection
- [ ] Views
  - [ ] Toggle Orthographic/Perspective Camera
  - [ ] Disable/Enable edges
  - [ ] Disable/Enable shading
  - [ ] Section view
- [ ] User Preferences
  - [ ] Dark mode
  - [ ] Configure key and mouse bindings
  - [ ] Save your settings in localStorage
  - [ ] Export/import user settings as json files
- [ ] Units
  - [ ] Allow Projects to be saved in Imperial units as well as Metric (Specified as System?)
  - [ ] Allow Projects to specify preferred units within that System, like [m, cm, mm], or [in, ft] (Specified as Units)
  - [ ] Allow Users to override the display units locally, without it affecting the Project file
- [ ] Electron or Tauri App so you can run local-only
  - [ ] Demonstrate associating .cadmium files with this app so they open on double-click from file explorer
- [ ] Holes
  - [ ] A dedicated history feature for punching holes at standard sizes [M4 tight clearance, 1/4-20 loose clearance, M5 Tap Drill]

## Features to Make Money

- [ ] SAAS - Hobbyist ($x/month)
  - [ ] Optional User accounts
  - [ ] Host files on behalf of users, so they can use any laptop at any time
    - [ ] Use Automerge under the hood so updates sync nicely across devices
    - [ ] Landing page that shows users the projects they have access to
  - [ ] Allow users to share projects with other individual users (read only or read-write)
  - [ ] Allow users to share projects into the public domain
  - [ ] Searchable catalog of public projects
  - [ ] Git integration
  - [ ] Thingiverse or Printables integration (one-click publish?)
- [ ] SAAS - Commercial ($y/month)
  - [ ] Groups which have a set of Group admins and set of Users, and can be the owners of Projects. Every user in the group can see and edit any Project owned by the Group.
  - [ ] Organizations which have a set of Org admins, a set of Users, and a set of Groups
  - [ ] Org admins can control Group membership, designate Group admins, transfer Projects between Groups
  - [ ] Group admins can only control Group membership
  - [ ] Clearance control: Annotate users with a Clearance level. Annotate Projects or Groups as requiring specific Clearance Levels or higher. Regardless of other rules, never allow a user to open a Project or join a Group they do not have sufficient Clearance for.
  - [ ] Disallow any Clearance Controlled projects from being shared publicly
  - [ ] Easy "revoke all access" button for Org Admins to remove employees who leave their companies
    - [ ] Support backups in case a User, Group Admin, or Org Admin causes wide damage
  - [ ] Audit trail: Every time a User access a Project, or an Admin modifies a permission or Clearance, it gets logged in a way that not even Org Admins can modify, although all Org Admins can read the log
  - [ ] Set up a set of servers which exclusively run in [the US, Europe], to comply with certain [US, European] restrictions
- [ ] Generative AI ($z/month)
  - [ ] Take every project that is shared in the public domain and (others which opt in) and train an LLM to predict steps given previous steps
  - [ ] Build a Co-Pilot like visual interface for using that LLM

## Might do, might not do

- [ ] Import
  - [ ] b-rep formats: .step, .x_t, .jt, .iges
  - [ ] mesh formats: .obj, .stl, .3mf, .gltf
  - [ ] project formats: .parasolid, .acis
- [ ] Add CAM functionality, or make a different, dedicated app
- [ ] Add FEA functionality, or make a different, dedicated app
