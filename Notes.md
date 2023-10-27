# Notes

- Do I need a formal state machine for the UI, with a diagram of all the states and transitions?
- After messages modify the state, if a get_representation() fails, I'm gonna need great error handling so that the user knows exactly what in the history is broken and how they need to fix it
- When specifying a face to extrude, simple indices aren't sufficient because the number of faces will change, and they are sorted by size. This will cause weird jumps. What heuristics should be used to prevent jumping? Maybe use the face that has the most points in common with the original face?
- Extrusions of two adjacent faces should probably merge together into a single solid. Should this happen before the extrusion by merging faces or after by merging solids? Also setting merge scope will be important: need to distinguish between "new", "add", and "remove".

# UI Ideas

Constant: can _always_ zoom, pan, rotate

Default (no special interaction)

Create new Step/Edit existing Step: dropdown in left window, looks like a form. Form inputs might be:

- Set name
- Select a plane (for sketch to live on)
- Select a vector (to extrude in)
- Select a point (to extrude to)
- Select face(s) (to extrude, or extrude to)
- Select a solid (merge scope)
- Choose an option: add/new/remove
- Choose a direction: forward/reverse

When editing an extrusion: have an arrow that you can drag which controls the length

When editing a sketch, there's mini state machines for each kind of feature. Hotkeys to trigger:

- New Circle:
  - place center point (smart snapping to sensible points)
  - drag to set initial radius, which is not a constraint (smart snapping)
- New Line Segment
  - place first point (smart snapping)
  - place second point (smart snapping)
- New Rectangle:
  - Center point or lower left, upper right?
  - place first point (smart snapping)
  - place second point (smart snapping)
- Round Corner:
  - Select corner(s)
  - Input to set radius
- Constraint placing:
  - enter "contraint mode"
  - click on a line, assume length constraint
  - mouse switches to showing where to place the constraint
  - after placing it, brings up text box that allows you to enter precise dimensions

Qs:

- Should I keep a history of sketch steps?
- How do I show how unsatisfied the constraints are in a sketch?
