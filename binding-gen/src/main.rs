extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("../clipper/wrapper.h")
        .whitelist_type("Polygons")
        .whitelist_type("ClipType")
        .whitelist_type("JoinType")
        .whitelist_type("EndType")
        .whitelist_type("PolyType")
        .whitelist_type("PolyFillType")
        .whitelist_type("Vertice")
        .whitelist_type("Path")
        .whitelist_type("Polygon")
        .whitelist_function("clean")
        .whitelist_function("simplify")
        .whitelist_function("execute")
        .whitelist_function("offset")
        .whitelist_function("free_path")
        .whitelist_function("free_polygon")
        .whitelist_function("free_polygons")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from("../src/");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings!");
}
