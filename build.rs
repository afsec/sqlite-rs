fn main() {
  println!("cargo:rerun-if-changed=build/build.rs");
  use std::time::SystemTime;
  let now: String = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .ok()
    .unwrap()
    .as_secs()
    .to_string();

  println!("cargo:rustc-env=SQLITERS_BUILT_AT={now}")
}
