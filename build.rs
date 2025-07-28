use std::process::Command;

fn main() {
    // Determine platform-specific asm file and format
    #[cfg(target_os = "windows")]
    let (asm_file, asm_format) = ("src/asm/math_win64.asm", "win64");

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let (asm_file, asm_format) = ("src/asm/math_unix.asm", if cfg!(target_os = "macos") { "macho64" } else { "elf64" });

    // Assemble asm to object file
    let status = Command::new("nasm")
        .args(&["-f", asm_format, asm_file, "-o", "src/asm/math.obj"])
        .status()
        .expect("Failed to assemble ASM file");
    assert!(status.success(), "NASM assembling failed");

    // Compile C file and link ASM object
    cc::Build::new()
        .file("src/ffi.c")
        .object("src/asm/math.obj")
        .compile("libcomplex_math.a");
}
