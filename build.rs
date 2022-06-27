fn main() {
    cc::Build::new()
        .file("c_code/triangle.c")
        .file("c_code/tricall_report.c")
        .file("c_code/interface_triangle.c")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-but-set-variable")
        .compile("c_code_interface_triangle");
    cc::Build::new()
        .cpp(true)
        .file("c_code/predicates.cxx")
        .file("c_code/tetgen.cxx")
        .file("c_code/interface_tetgen.cpp")
        .flag("-Wno-int-to-pointer-cast")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-but-set-variable")
        .compile("c_code_interface_tetgen");
}
