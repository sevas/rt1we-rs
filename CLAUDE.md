# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A toy raytracer implemented in Rust, following the "Ray Tracing in One Weekend" book/series
(https://github.com/RayTracing/raytracing.github.io/). It is explicitly an educational project —
per the readme, don't treat code here as a reference for idiomatic Rust; ownership patterns and
custom operator impls are a work in progress as the author learns the language.

## Workspace layout

Cargo workspace (resolver v1) with three members:

- `rt1we_renderer` — the library crate. All raytracing logic lives here: vector/geometry math
  (`geometry.rs`), rays and sphere intersection (`ray.rs`), materials/camera/scene/render loop
  (`render.rs`), image buffer (`image.rs`), PPM file I/O (`ppmio.rs`), and angle conversions
  (`trig.rs`).
- `rt1we_sample` — CLI binary that renders a trajectory of frames to PPM files under `out/`
  using the single-threaded `render()` path.
- `rt1we_gui` — `egui`/`eframe` desktop GUI (binary name `rt1we_gui`). Lets you tweak
  resolution/depth/samples interactively and toggle between the scalar `render()` and the
  `rayon`-parallelized `render_parallel()`, displaying the result as an egui texture.

`rt1we_gui` and `rt1we_sample` both depend on `rt1we_renderer` via path dependency; neither
contains raytracing logic itself, only glue/UI code.

## Architecture inside `rt1we_renderer`

- **Math primitives** (`geometry.rs`): `Vec3` (aliased as `Point`/`Color` at the type level)
  with operator overloads (many implemented redundantly for owned values, refs, and mixed
  ref/owned combos — this duplication is intentional/in-progress per the author, not a bug to
  "clean up" reflexively).
- **Scene representation** (`render.rs`): `Hittable` trait implemented by `Sphere` and `Plane`;
  `HittableList` is the flat, non-accelerated (no BVH) container that linearly tests every
  object per ray. `Material` trait (`Lambertian`, `Metal`, `Dieletric`) implements scattering;
  materials are stored as `Vec<Box<dyn Material>>` and referenced from hit records by
  `material_id` index rather than being owned by the geometry directly.
- **Rendering**: `Camera` builds primary rays from normalized `(u, v)` viewport coordinates.
  `ray_color_2` is the recursive path tracer (depth-limited diffuse/reflect/refract bounces).
  Both `render()` (scalar, scanline loop with progress printed to stdout) and
  `render_parallel()` (same scene/camera setup, but parallelized over scanlines with `rayon`'s
  `par_chunks_mut`, writing directly into the pixel buffer) hardcode the same fixed demo scene
  (three spheres + ground) — there is no scene description format; changing the scene means
  editing these functions directly. Gamma correction (sqrt) and clamping to `[0, 0.999]` happen
  at the end of both render paths before quantizing to `u8`.
- **Image I/O**: `ImageRGBA` is a flat `Vec<u8>` RGBA buffer (origin: bottom-left, so `flipv()`
  is used before writing/display to get top-left-origin output). `ppmio.rs` only supports the
  ASCII P3 PPM format (no binary P6).

## Common commands

Build/test everything in the workspace:
```
cargo build
cargo test
```

Run a single test (works across the workspace by test function name):
```
cargo test test_hit_sphere_returns_correct_distance_when_hitting_a_sphere_just_in_front
```

Run a single module's tests (e.g. everything in `render.rs`):
```
cargo test -p rt1we_renderer render::test
```

Run the GUI app (the workspace has no `default-run`, so a bare `cargo run` errors with
"could not determine which binary to run" — pass `-p rt1we_gui`):
```
cargo run -p rt1we_gui
cargo run -p rt1we_gui --release
```

Run the CLI sample renderer (writes PPM frames to `./out/`, so create that dir first):
```
mkdir -p out
cargo run -p rt1we_sample --release
```

Generate docs (crate has doctests in `geometry.rs`):
```
cargo doc --open
```

Coverage (matches CI in `.github/workflows/coverage.yaml`, requires nightly + `cargo-tarpaulin`):
```
cargo +nightly tarpaulin --verbose --all-features --workspace --timeout 120 --out Xml
```

Formatting uses a repo-specific `rustfmt.toml` (`fn_params_layout = "Compressed"`,
`use_small_heuristics = "Max"`) — run `cargo fmt` before committing rather than hand-wrapping
function signatures.

## Python helper scripts (`scripts/`)

Not part of the Rust build; used for inspecting render output:
- `ppm2png.py` — batch-convert a folder of `.ppm` files to `.png`.
- `imview.py` — pyqtgraph-based live viewer that reloads an image file whenever it changes on
  disk (useful for watching `out/latest.ppm` update during a render).
- `find_roots.py`, `make_movie.sh` — auxiliary trajectory/movie tooling referenced by the
  animation frame output in `rt1we_sample`.

## CI

`.github/workflows/rust.yml` runs `cargo build`, `cargo test`, then does a release render
(`cargo run --release`) and uploads `out/` as an artifact. Note: that bare `cargo run --release`
step currently fails — the workspace has two binaries (`rt1we_gui`, `rt1we_sample`) and no
`default-run`, so cargo can't pick one without `-p`/`--bin`; this is a pre-existing CI bug, not
something introduced by removing the (already-inert) `default-run` key from the workspace
manifest. Coverage is tracked via `codecov.yaml`/`codecov.yml` against the `main` branch.
