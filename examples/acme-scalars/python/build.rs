fn main() {
    // On macOS an extension module leaves the Python symbols to be resolved by
    // the interpreter that loads it (`-undefined dynamic_lookup`). maturin
    // passes those linker arguments itself; a plain `cargo build` needs them
    // from the module crate's own build script. No-op on other platforms.
    pyo3_build_config::add_extension_module_link_args();
}
