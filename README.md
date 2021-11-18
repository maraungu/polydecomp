# Welcome to POLYDECOMP!

Polydecomp is a web app for drawing [simple polygons](https://en.wikipedia.org/wiki/Simple_polygon) and viewing their Delaunay triangulation and convex decomposition.

## Intro

The user interface of the app uses the [egui UI library](https://github.com/emilk/egui) and the [eframe framework](https://github.com/emilk/eframe_template) for web apps.  The `main` and `lib` files (for running natively and for compiling to `wasm`, respectively) are in the `egui-app`, while the `egui-lib` folder contains the app interface and drawing files.  The backend runs the constrained Delaunay triangulation algorithm of the [spade](https://docs.rs/crate/spade/1.8.2) crate and relies on an implementation of the [Hertel-Mehlhorn](https://people.mpi-inf.mpg.de/~mehlhorn/ftp/FastTriangulation.pdf) (HM) algorithm for convex decomposition of a simple polygon to be found in the `polygon` folder.

## Setup

To run natively, navigate to the root directory of the project and type `cargo run --release`.

To view in the browser, see the GitPage of the project here:

To test locally and/or recompile to `wasm`, follow the instructions [here](https://github.com/emilk/eframe_template#compiling-for-the-web).

## How to use
1. Draw a simple polygon by clicking on the canvas or load one of the availabe polygons.
2. Click to show the triangulation.  This runs the triangulation algorithm on the polygon and display the result on the canvas.
3. Click to show the essential edges of the triangulation or to show the convex parts of the polygon.  This runs the HM algorithm and shows the result.  The essential edges are those edges of the triangulation which are not edges of the original polygon, but whose removal would make the angle they bisect concave.  The convex parts are obtained by gluing the triangles along the non-essential edges of the triangulation.

Note: Since the convex decomposition algorithm relies on an existing triangulation of the polygon, one must always first run the triangulation before the essential edges or the convex parts.

