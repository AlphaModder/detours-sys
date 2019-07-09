extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag_if_supported("/WX")
        .flag_if_supported("/MT")
        .flag_if_supported("/Gy")
        .flag_if_supported("/Gm-")
        .flag_if_supported("/Zl")
        .flag_if_supported("/Od")
        .define("WIN32_MEAN_AND_LEAN", None)
        .define("_WIN32_WINNT", Some("0x501"))
        .include("./Detours/src")
        .file("./Detours/src/detours.cpp")
        .file("./Detours/src/modules.cpp")
        .file("./Detours/src/disasm.cpp")
        .file("./Detours/src/image.cpp")
        .file("./Detours/src/creatwth.cpp")
        .file("./Detours/src/disolx86.cpp")
        .file("./Detours/src/disolx64.cpp")
        .file("./Detours/src/disolia64.cpp")
        .file("./Detours/src/disolarm.cpp")
        .file("./Detours/src/disolarm64.cpp")
        .compile("detours.lib");
}