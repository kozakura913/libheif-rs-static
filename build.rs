fn main() {
	let libwebp_dst = cmake::Config::new("libwebp").define("BUILD_SHARED_LIBS", "false").define("CMAKE_BUILD_TYPE", "Release").build();
	println!("cargo:rustc-link-search=native={}/lib", libwebp_dst.display());
	println!("cargo:rustc-link-lib=static=sharpyuv");
	let libde265_dst = cmake::Config::new("libde265").define("BUILD_SHARED_LIBS", "false").build();
	println!("cargo:rustc-link-search=native={}/lib", libde265_dst.display());
	println!("cargo:rustc-link-lib=static=de265");
	//let openjpeg_dst = cmake::Config::new("openjpeg").define("BUILD_SHARED_LIBS", "false").define("BUILD_STATIC_LIBS", "true").build();
	//println!("cargo:rustc-link-search=native={}/lib", openjpeg_dst.display());
	//println!("cargo:rustc-link-lib=static=openjp2");
	//let mut build_opts=cc::Build::new();
	//build_opts.includes(std::path::Path::new(&format!("{}/include/webp",libwebp_dst.display())));
	//build_opts.includes(std::path::Path::new(&format!("{}/include",libde265_dst.display())));
	let dst = cmake::Config::new("libheif")
		.define("WITH_OpenJPEG_DECODER", "false")
		.define("WITH_OpenJPEG_ENCODER", "false")
		.define("WITH_LIBSHARPYUV", "true")
		.define("WITH_AOM_DECODER", "false")
		.define("WITH_AOM_ENCODER", "false")
		.define("WITH_X265", "false")
		.define("WITH_OpenH264_DECODER", "false")
		.define("BUILD_SHARED_LIBS", "false")
		.define("LIBDE265_INCLUDE_DIR", format!("{}/include",libde265_dst.display()))
		.define("LIBDE265_LIBRARY", format!("{}/lib/libde265.a",libde265_dst.display()))
		//.define("OpenJPH_INCLUDE_DIR", format!("{}/lib",openjpeg_dst.display()))
		.define("LIBSHARPYUV_INCLUDE_DIR", format!("{}/include/webp",libwebp_dst.display()))
		.define("LIBSHARPYUV_LIBRARY", format!("{}/lib/libsharpyuv.a",libwebp_dst.display()))
		//.init_cxx_cfg(build_opts)
		.build();
	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	match std::env::var("LIBHEIF_LINK_CXX").as_ref().map(|s|s.as_str()){
		Ok("static")=>println!("cargo:rustc-link-lib=static=stdc++"),
		Ok("dynamic")=>println!("cargo:rustc-link-lib=dylib=stdc++"),
		Ok(_)=>{},
		Err(_)=>println!("cargo:rustc-link-lib=dylib=stdc++"),
	}
	println!("cargo:rustc-link-lib=static=heif");
	//${OUT_DIR}=./target/debug/build/heif-*/out
	let bindings = bindgen::Builder::default().header(format!("{}/include/libheif/heif.h",dst.display())).clang_arg(format!("-I{}/include/",dst.display())).parse_callbacks(Box::new(bindgen::CargoCallbacks::new())).generate().expect("Unable to generate bindings");
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings.write_to_file(out_path.join("libheif.rs")).expect("Couldn't write bindings!");
}
