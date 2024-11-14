fn main() {
	let libheif_libs_dir=std::env::var("LIBHEIF_LIBS_DIR").ok();
	let out_dir=std::env::var("OUT_DIR").unwrap();
	if libheif_libs_dir.is_none(){
		let dir=std::path::PathBuf::from(&out_dir);
		std::process::Command::new("git").current_dir(&dir).args(["clone","-b","v1.4.0","--depth","1","https://chromium.googlesource.com/webm/libwebp.git"]).spawn().unwrap().wait_with_output().unwrap();
		let mut libwebp=dir.clone();
		libwebp.push("libwebp");
		let libwebp_dst = cmake::Config::new(libwebp).define("BUILD_SHARED_LIBS", "false").define("CMAKE_BUILD_TYPE", "Release").build();
		if !libwebp_dst.exists(){
			panic!("cmake libwebp error");
		}
		std::process::Command::new("git").current_dir(&dir).args(["clone","-b","v1.0.15","--depth","1","https://github.com/strukturag/libde265"]).spawn().unwrap().wait_with_output().unwrap();
		let mut libde265=dir.clone();
		libde265.push("libde265");
		let libde265_dst = cmake::Config::new(libde265).define("BUILD_SHARED_LIBS", "false").build();
		if !libde265_dst.exists(){
			panic!("cmake libde265 error");
		}
		std::process::Command::new("git").current_dir(&dir).args(["clone","-b","v1.19.3","--depth","1","https://github.com/strukturag/libheif"]).spawn().unwrap().wait_with_output().unwrap();
		let mut libheif=dir.clone();
		libheif.push("libheif");
		//let openjpeg_dst = cmake::Config::new("openjpeg").define("BUILD_SHARED_LIBS", "false").define("BUILD_STATIC_LIBS", "true").build();
		let dst = cmake::Config::new(libheif)
			.define("WITH_OpenJPEG_DECODER", "false")
			.define("WITH_OpenJPEG_ENCODER", "false")
			.define("WITH_LIBSHARPYUV", "true")
			.define("WITH_AOM_DECODER", "false")
			.define("WITH_AOM_ENCODER", "false")
			.define("WITH_X265", "false")
			.define("WITH_OpenH264_DECODER", "false")
			.define("BUILD_SHARED_LIBS", "false")
			.define("LIBDE265_INCLUDE_DIR", format!("{}",std::env::var("OUT_DIR").unwrap()+"/include"))
			.define("LIBDE265_LIBRARY", format!("{}/lib/libde265.a",std::env::var("OUT_DIR").unwrap()))
			//.define("OpenJPH_INCLUDE_DIR", format!("{}/lib",openjpeg_include_dir)
			.define("LIBSHARPYUV_INCLUDE_DIR", std::env::var("OUT_DIR").unwrap()+"/include/webp")
			.define("LIBSHARPYUV_LIBRARY", format!("{}/lib/libsharpyuv.a",std::env::var("OUT_DIR").unwrap()))
			.build();
			if !dst.exists(){
				panic!("cmake libheif error");
			}
	}
	let libheif_libs_dir=libheif_libs_dir.unwrap_or_else(||std::env::var("OUT_DIR").unwrap()+"/lib");
	println!("cargo:rustc-link-search=native={}", libheif_libs_dir);
	println!("cargo:rustc-link-lib=static=sharpyuv");
	println!("cargo:rustc-link-lib=static=de265");
	//println!("cargo:rustc-link-lib=static=openjp2");
	println!("cargo:rustc-link-lib=static=heif");
	match std::env::var("LIBHEIF_LINK_CXX").as_ref().map(|s|s.as_str()){
		Ok("static")=>println!("cargo:rustc-link-lib=static=stdc++"),
		Ok("dynamic")=>println!("cargo:rustc-link-lib=dylib=stdc++"),
		Ok(_)=>{},
		Err(_)=>println!("cargo:rustc-link-lib=dylib=stdc++"),
	}
	//${OUT_DIR}=./target/debug/build/heif-*/out
	//let bindings = bindgen::Builder::default().header(format!("{}/include/libheif/heif.h",&out_dir)).clang_arg(format!("-I{}/include/",&out_dir)).parse_callbacks(Box::new(bindgen::CargoCallbacks::new())).generate().expect("Unable to generate bindings");
	//let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	//bindings.write_to_file(out_path.join("libheif.rs")).expect("Couldn't write bindings!");
}
