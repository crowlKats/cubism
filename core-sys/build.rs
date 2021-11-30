fn main() {
  println!(
    "cargo:rustc-link-search=native=lib/{}/",
    std::env::var("TARGET").unwrap()
  );

  let bindings = bindgen::builder()
    .header("lib/Live2DCubismCore.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .dynamic_link_require_all(true)
    .generate()
    .unwrap();

  let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .unwrap();
}
