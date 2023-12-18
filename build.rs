/* 
use std::path::PathBuf;
use std::env;
use miette::IntoDiagnostic;
use path_slash::PathBufExt;

fn main() -> miette::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").into_diagnostic()?);
    let install_lib_dir = out_dir.join("lib");
    let install_include_dir = out_dir.join("include");
    let target = env::var("TARGET").into_diagnostic()?;
    let is_ios = target.contains("apple-ios");

    // Ensure correct include paths for bindings
    let include_paths = vec![
        PathBuf::from("include"),
        PathBuf::from("abseil-cpp"), // Uncommented for potential necessary headers
        PathBuf::from("ink-stroke-modeler"), // Uncommented for potential necessary headers
        install_include_dir.clone(),
    ];

    // Abseil and Ink Stroke Modeler Configuration
    if is_ios {
        let ios_platform = "OS64";

        // Abseil configuration
        let mut absl_config = cmake::Config::new("abseil-cpp");
        absl_config
            .profile("Release")
            .define("CMAKE_CXX_STANDARD", "20")
            .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
            .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
            .define("CMAKE_PREFIX_PATH", &out_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_PREFIX", &out_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_LIBDIR", &install_lib_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_INCLUDEDIR", &install_include_dir.to_slash_lossy().to_string())
            .define("PLATFORM", ios_platform)
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("ABSL_PROPAGATE_CXX_STD", "ON")
            .cxxflag("-DABSL_FORCE_WAITER_MODE=4")
            .build();

        // Ink Stroke Modeler configuration
        let mut ink_config = cmake::Config::new("ink-stroke-modeler");
        ink_config
            .profile("Release")
            .define("CMAKE_CXX_STANDARD", "20")
            .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
            .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
            
            .define("CMAKE_PREFIX_PATH", &out_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_PREFIX", &out_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_LIBDIR", &install_lib_dir.to_slash_lossy().to_string())
            .define("CMAKE_INSTALL_INCLUDEDIR", &install_include_dir.to_slash_lossy().to_string())
            .define("PLATFORM", ios_platform)
            .define("INK_STROKE_MODELER_FIND_DEPENDENCIES", "ON")
            .define("INK_STROKE_MODELER_BUILD_TESTING", "OFF")
            .define("INK_STROKE_MODELER_ENABLE_INSTALL", "ON")
            .define("CMAKE_EXE_LINKER_FLAGS", "-framework CoreFoundation")
            .build();
    }

    // Autocxx Builder for generating bindings
    let mut builder = autocxx_build::Builder::new(PathBuf::from("src/lib.rs"), include_paths.iter())
        .extra_clang_args(&["-std=gnu++20"])
        .build()?;
    
    builder
        .flag_if_supported("-std=gnu++20")
        .files(include_paths.iter())
        .compile("ink-stroke-modeler-rs")
       ;

    // Enhanced Logging for Linking
    println!(
        "cargo:rustc-link-search=native={}",
        install_lib_dir.display()
    );

    for lib in std::fs::read_dir(&install_lib_dir).into_diagnostic()? {
        let lib = lib.into_diagnostic()?;
        let lib_name = lib.file_name().to_string_lossy().to_string();

        if lib_name.starts_with("libabsl") || lib_name.starts_with("libink_stroke_modeler") {
            println!("Linking library: {}", lib_name); // Added logging
            println!(
                "cargo:rustc-link-lib=static={}",
                lib.path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .trim_start_matches("lib")
            );
        }
    }

    // Re-run when files are modified
    for source in include_paths.iter() {
        println!("cargo:rerun-if-changed={}", source.display());
    }

    Ok(())
}
*/

use std::path::PathBuf;
use std::env;
use miette::IntoDiagnostic;
use path_slash::PathBufExt;

fn main() -> miette::Result<()> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").into_diagnostic()?);
    let install_lib_dir = out_dir.join("lib");
    let install_include_dir = out_dir.join("include");
    let target = env::var("TARGET").into_diagnostic()?;
    let is_ios = target.contains("apple-ios");
    

    let bindings_files = vec![
        PathBuf::from("build.rs"),
        PathBuf::from("src/lib.rs"),
        PathBuf::from("include/extras.h"),
    ];
    let bindings_cpp_sources = vec![PathBuf::from("src/extras.cc")];
    if is_ios {
        let ios_platform = "OS64"; 
    let _absl_cmake_install_dir = cmake::Config::new("abseil-cpp")
        // this avoids having to link to `dbghelp` on windows-mingw
        .profile("Release")
        .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
        .define("PLATFORM", ios_platform)
        .define("CMAKE_CXX_STANDARD", "20")
        // Rust needs -fPIE or -fPIC
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        .define("CMAKE_PREFIX_PATH", &out_dir.to_slash_lossy().to_string())
        .define(
            "CMAKE_INSTALL_PREFIX",
            &out_dir.to_slash_lossy().to_string(),
        )
        .define(
            "CMAKE_INSTALL_LIBDIR",
            &install_lib_dir.to_slash_lossy().to_string(),
        )
        .define(
            "CMAKE_INSTALL_INCLUDEDIR",
            &install_include_dir.to_slash_lossy().to_string(),
        )
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ABSL_PROPAGATE_CXX_STD", "ON")
        // this forces absl stdcpp waiter implementation (see `absl/synchronization/internal/waiter.h`).
        // this possibly circumvents build failure with mingw. see: https://github.com/abseil/abseil-cpp/issues/1510
        .cxxflag("-DABSL_FORCE_WAITER_MODE=4")


        
      
     

        .build();

    let _ink_stroke_modeler_cmake_install_dir = cmake::Config::new("ink-stroke-modeler")
        // this avoids having to link to `dbghelp` on windows-mingw
        .profile("Release")
        .define("CMAKE_CXX_STANDARD", "20")
        .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
        .define("PLATFORM", ios_platform)
        // Rust needs -fPIE or -fPIC
        .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
        // This takes priority in cmake's find_package() when searching for absl to use our compiled version
        // instead of the system-provided package
        .define("CMAKE_PREFIX_PATH", &out_dir.to_slash_lossy().to_string())
        .define(
            "CMAKE_INSTALL_PREFIX",
            &out_dir.to_slash_lossy().to_string(),
        )
        .define(
            "CMAKE_INSTALL_LIBDIR",
            &install_lib_dir.to_slash_lossy().to_string(),
        )
        .define(
            "CMAKE_INSTALL_INCLUDEDIR",
            &install_include_dir.to_slash_lossy().to_string(),
        )
        .define("INK_STROKE_MODELER_FIND_DEPENDENCIES", "ON")
        .define("INK_STROKE_MODELER_BUILD_TESTING", "OFF")
        .define("INK_STROKE_MODELER_ENABLE_INSTALL", "ON")
        .define("CMAKE_EXE_LINKER_FLAGS", "-framework CoreFoundation")
        .build();



        


    }
    let include_paths = vec![
        PathBuf::from("include"),
       // PathBuf::from("absl-cpp"),
        //PathBuf::from("ink-stroke-modeler"),
        install_include_dir,
    ];

    let mut builder =
        autocxx_build::Builder::new(PathBuf::from("src/lib.rs"), include_paths.iter())
            .extra_clang_args(&["-std=gnu++20"])
            .build()?;
    builder
        //.flag_if_supported("-v")
        .flag_if_supported("-std=gnu++20")
        // include paths already passed in by the autocxx builder
       // .includes(include_paths.iter())
        //.cpp_set_stdlib(Some("stdc++"))
        .files(bindings_cpp_sources.iter())
        .try_compile("ink-stroke-modeler-rs")
        .into_diagnostic()?;

    // Linking

    println!(
        "cargo:rustc-link-search=native={}",
        install_lib_dir.display()
    );

    for lib in std::fs::read_dir(install_lib_dir).into_diagnostic()? {
        let lib = lib.into_diagnostic()?;
        let lib_name = lib.file_name().to_string_lossy().to_string();

        if lib_name.starts_with("libabsl") || lib_name.starts_with("libink_stroke_modeler") {
            println!(
                "cargo:rustc-link-lib=static={}",
                lib.path()
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .trim_start_matches("lib")
            );
        }
    }

    // Re-run when files are modified
    for source in bindings_files.iter().chain(bindings_cpp_sources.iter()) {
        println!("cargo:rerun-if-changed={}", source.display());
    }

    Ok(())
}

/* 
use std::env;
use std::path::PathBuf;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let target = env::var("TARGET").into_diagnostic()?;
   let is_ios = target.contains("apple-ios");
    
    let out_dir_str = env::var("OUT_DIR").into_diagnostic()?;
    let out_dir = PathBuf::from(out_dir_str);
    let install_lib_dir = out_dir.join("lib");
    let install_include_dir = out_dir.join("include");

    if is_ios {
        let ios_platform = "OS64"; 
        cmake::Config::new("abseil-cpp")
            .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
            .define("PLATFORM", ios_platform)
            .define("CMAKE_CXX_STANDARD", "20")
            .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
            .define("BUILD_TESTING", "OFF")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("ABSL_PROPAGATE_CXX_STD", "ON")
            .define("CMAKE_INSTALL_PREFIX", out_dir.to_str().unwrap())
            .define("CMAKE_INSTALL_LIBDIR", install_lib_dir.to_str().unwrap())
            .define("CMAKE_INSTALL_INCLUDEDIR", install_include_dir.to_str().unwrap())
            .profile("Release")
            .build();

        let absl_dir = PathBuf::from("/opt/homebrew/opt/abseil/lib/cmake/absl");
        cmake::Config::new("ink-stroke-modeler")
            .define("CMAKE_TOOLCHAIN_FILE", "/Users/tega/Desktop/Project/ink-stroke-modeler-rs/ios.toolchain.cmake")
            .define("PLATFORM", ios_platform)
            .define("CMAKE_CXX_STANDARD", "20")
            .define("CMAKE_POSITION_INDEPENDENT_CODE", "ON")
            .define("absl_DIR", absl_dir.to_str().unwrap())
            .define("CMAKE_PREFIX_PATH", out_dir.to_str().unwrap())
            .define("CMAKE_INSTALL_PREFIX", out_dir.to_str().unwrap())
            .define("CMAKE_INSTALL_LIBDIR", install_lib_dir.to_str().unwrap())
            .define("CMAKE_INSTALL_INCLUDEDIR", install_include_dir.to_str().unwrap())
            .define("CMAKE_EXE_LINKER_FLAGS", "-framework CoreFoundation")
            .profile("Release")
            .build();
    }
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    let include_paths = vec![
        PathBuf::from("include"),
        install_include_dir.clone(),
    ];

    let bindings_cpp_sources = vec![PathBuf::from("src/extras.cc")];

    let mut builder = autocxx_build::Builder::new(PathBuf::from("src/lib.rs"), include_paths.iter())
        .extra_clang_args(&["-std=gnu++20"])
        .build()
        .into_diagnostic()?;

    builder
        .flag_if_supported("-std=gnu++20")
        .files(bindings_cpp_sources.iter())
        .compile("ink-stroke-modeler-rs");

    println!("cargo:rustc-link-search=native={}", install_lib_dir.display());
    for lib in std::fs::read_dir(&install_lib_dir).into_diagnostic()? {
        let lib = lib.into_diagnostic()?;
        let lib_name = lib.file_name().to_string_lossy().to_string();
        if lib_name.starts_with("libabsl") || lib_name.starts_with("libink_stroke_modeler") {
            println!("cargo:rustc-link-lib=static={}", 
                     lib.path().file_stem().unwrap().to_string_lossy().trim_start_matches("lib"));
        }
    }

    let bindings_files = vec![
        PathBuf::from("build.rs"),
        PathBuf::from("src/lib.rs"),
        PathBuf::from("include/extras.h"),
    ];

    for source in bindings_files.iter().chain(bindings_cpp_sources.iter()) {
        println!("cargo:rerun-if-changed={}", source.display());
    }

    Ok(())
}


*/