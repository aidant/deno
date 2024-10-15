use std::{env, path::Path};

fn main() {
  use_android_ndk_compiler_rt()
}

fn use_android_ndk_compiler_rt() {
  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

  if target_arch == "aarch64" && target_os == "android" {
    let host_os = match env::consts::OS {
      "linux" => "linux",
      "macos" => "darwin",
      "windows" => "windows",
      os => panic!("Unable to build for Android on {os}, the Android NDK only provides prebuilt binaries for macOS, Linux, and 64-bit Windows"),
    };

    let android_home = env::var("ANDROID_HOME")
      .or_else(|err| std::env::var("ANDROID_SDK_ROOT").map_err(|_| err))
      .expect("ANDROID_HOME not set");

    let android_ndk_version =
      env::var("ANDROID_NDK_VERSION").unwrap_or("27.1.12297006".to_string());
    let android_clang_version =
      env::var("ANDROID_CLANG_VERSION").unwrap_or("18".to_string());

    /*
      Despite the x86_64 tag in the Darwin name, those are fat binaries that
      include M1 support. The paths were not updated to reflect that support
      because doing so would have broken existing builds that encode those
      paths.
    */
    let toolchain_dir = format!("toolchains/llvm/prebuilt/{host_os}-x86_64");
    let clang_dir = format!("lib/clang/{android_clang_version}");
    let compiler_rt_dir = "lib/linux";

    let rustc_link_search = format!("{android_home}/ndk/{android_ndk_version}/{toolchain_dir}/{clang_dir}/{compiler_rt_dir}");

    if !Path::new(&rustc_link_search).is_dir() {
      panic!(
        r#"Unable to build for Android, the Android NDK included clang compiler-rt directory could not be found. Try setting "ANDROID_NDK_VERSION" and/or "ANDROID_CLANG_VERSION""#
      )
    }

    println!("cargo::rustc-link-search={rustc_link_search}");
    println!("cargo::rustc-link-lib=static=clang_rt.builtins-{target_arch}-{target_os}");
  }
}
