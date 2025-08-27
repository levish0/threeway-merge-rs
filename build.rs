fn main() {
    let mut build = cc::Build::new();
    
    // Add xdiff source directory to include path
    build.include("src/xdiff");
    
    // Add all xdiff C source files
    build.file("src/xdiff/xdiffi.c");
    build.file("src/xdiff/xemit.c"); 
    build.file("src/xdiff/xhistogram.c");
    build.file("src/xdiff/xmerge.c");
    build.file("src/xdiff/xpatience.c");
    build.file("src/xdiff/xprepare.c");
    build.file("src/xdiff/xutils.c");
    
    // Compile and link
    build.compile("xdiff");
    
    println!("cargo:rustc-link-lib=static=xdiff");
    println!("cargo:rerun-if-changed=src/xdiff/");
}