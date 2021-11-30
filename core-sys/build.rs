fn main() {
  println!("cargo:rustc-link-search=native=lib/{}/", std::env::var("TARGET").unwrap());

  let bindings = bindgen::builder()
    .header("lib/Live2DCubismCore.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .default_enum_style(bindgen::EnumVariation::Rust {
      non_exhaustive: false,
    })
    .dynamic_link_require_all(true)
    .bitfield_enum("csmFlags")
    .generate()
    .unwrap();

  let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}
