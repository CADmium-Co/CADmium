# CADmium Core

This is the heart of CADmium. It's a wrapper around [ISOtope](https://github.com/CADmium-Co/ISOtope)
2D constraint solver and [Truck](https://github.com/ricosjp/truck) 3D CAD kernel.

It essentially holds the state for both of these libraries, provides a central
way to interact with them and provides a consistent way to reconstruct objects
with them.

It's written in Rust and its main build target is WebAssembly.

## Building

After doing `pnpm install` in the root of the repository, you can build this
package by running `pnpm build` in this directory.

There's also a `pnpm dev` command that will watch for changes and rebuild the
project as needed.

## Testing

You can run the tests for this package by running `pnpm test` in this directory.

## Design

The whole library is designed around the concept of parent-child relationships.

At the root of the library is the `project::Project` struct which describes the
whole project. This struct holds all the workbenches.

A `workbench::Workbench` is the collection of all the history of a part of the project,
as well as the result of the whole history, which is an array of points, planes,
sketches and features (i.e. objects).

A project has multiple workbenches and a workbench has multiple objects and sketches.

Each `isketch::ISketch` (distinct from `isotope::sketch::Sketch`) holds an
`isotope::sketch::Sketch` and is able to act on it.

Each `solid::point::Point3` is a point in 3D space, created manually, outside
of any sketch.

Each `solid::plane::Plane` is a plane in 3D space, created either manually or
on top of a face of a solid.

Each `solid::solid::Solid` is a solid in 3D space created by one or more features
(e.g. extrude, revolve, etc.).

### Messages

The library is designed to be used in a message-passing way.

Throughout the library, you'll see a lot of structs that implement
the `MessageHandler` trait and are part of the `message::Message` enum.

These can be thought as commands (e.g. `isketch::AddPoint` or `solid::extrusion::Add`)
that act on the state of the library.

Each `MessageHandler` has a `Parent` type, which is what it acts on.

For example an `isketch::AddPoint` message acts on an `isketch::ISketch`
to mutate its `isotope::sketch::Sketch` and a `workbench::AddSketch`
acts on a `workbench::Workbench` to mutate it list of sketches.

Each `MessageHandler` can return one of the following:

- `Ok(Some(IDType))`: The handling was successful and the ID of the new item
  is returned (e.g. for `isketch::AddPoint`).
- `Ok(None)`: The handling was successful and no new item was created (e.g. for
  `isketch::DeletePrimitive`).
- `Err(Error)`: The handling was unsuccessful and an error is returned.

### Rendering the current state

Essentially each `workbench::Workbench` represents the current state of the part
of the project the user is working on and after each message, it gets updated.

Calling `project.get_workbench_by_id(id)` should have all the information needed
to render the current state of that part.

### History

Every time a message is handled that changes the state of a `workbench::Workbench`,
a `step::Step` is created and added to the `workbench::Workbench.history`.

Each `step::Step` holds a `message::Message`, a name, an ID (which is just the index
of the step in the history) and a `suppressed` boolean.

When showing the history to the user, the `name` should be used to describe the step,
the `suppressed` boolean should be used to show if the step is suppressed,
and the `message::Message` variant should be used to show what the step did,
its arguments and its possible actions (e.g. `solid::extrusion::Add` should show
the extrusion icon with the ability to change length, direction, etc.).
