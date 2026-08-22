# The photographed owl

The wild `Lineage` example loads `owl.glb` from this directory — a
photogrammetry scan of a real owl, because a reader looked at the modelled one
and said it looked nothing like a bird, and chose a scanned model over a better
procedural one when asked.

## Status: supplied

The developer downloaded the model themselves and dropped it in: every
worthwhile source is behind an account wall, so no automated fetch can replace
this file. The file is a rigged, Blender-exported glTF with embedded textures,
no Draco, no KTX2, and **one skeletal animation**, which is what the nest
plays.

## What to put here

| | |
|---|---|
| File | `owl.glb` (binary glTF, textures embedded, no Draco / KTX2) |
| Size | ≤ ~10 MB — pick the coarser LOD of a scan, not the archival one |
| License | CC0 preferred; CC-BY acceptable; **CC BY-NC accepted by the developer with the standing constraint recorded below** |

Known good candidates (free downloads with a Sketchfab account):

- *Snowy Owl* — John Fino (tauricity), taxidermy scan, 8K texture.
  <https://sketchfab.com/3d-models/snowy-owl-with-sound-e68a21c633ba4e1093bf4078c43fcd68>
- *Short Eared Owl* — John Fino (tauricity), taxidermy scan.
  <https://sketchfab.com/3d-models/short-eared-owl-d537a005c8764be083f15011053ba9e2>

When exporting from Sketchfab choose **glTF**, re-pack to a single `.glb` if it
arrives as `.gltf` + textures, and fill in the table below.

## The record (fill in when the file lands)

| | |
|---|---|
| Source | <https://sketchfab.com/3d-models/owl-86ec461ab6404628b10fcdc26ac004dd> (`skfb.ly/pwnos`) |
| Author | kenchoo (Sketchfab); the page credits the asset as "Realistic Owl 3D Model" by WildMesh 3D Free |
| License | **CC BY-NC 4.0** — attribution required, NON-COMMERCIAL only |
| Fetched | 2026-08-07, by the developer, placed by hand |
| Size | 11,051,164 bytes (`owl.glb`, glTF 2.0 binary, textures embedded) |

**THE LICENSE IS NON-COMMERCIAL, AND THAT IS A STANDING CONSTRAINT.** This
file may live in the documentation site only for as long as the site is
non-commercial. The day anything this repository documents is sold, this owl
is the first thing that has to go — swap her for a CC0/CC-BY scan (the
candidates above) or for the modelled owl, and delete this directory. She is
deliberately NOT in `dist/`: no consumer of the packages ever receives a
byte of her, so the constraint binds this site and nothing downstream.

`docs-site/src/main.rs` copies `owl.glb` to the site as
`assets/vendor/owl.glb` only when it exists; nothing anywhere requires it. <!-- allow-reference: the URL the built site serves, not a path in this tree -->

