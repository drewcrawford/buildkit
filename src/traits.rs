use std::path::{Path, PathBuf};

use crate::build_settings::Configuration;

///Implement this trait to bring in your compiler
pub trait CompileStep {
    ///The extension to scan for.  We will use this to create individual `CompileStep`.
    const SOURCE_FILE_EXTENSION: &'static str;

    ///Compile one file, placing the output in the intermediate dir.
    ///
    /// # args
    /// * `path`: Path to the source file
    /// * `intermediate_dir`: Output location for object files.
    /// * `configuration`: Holds build settings
    /// * `dependency_path`: Output file contining discovered dependencies.  If you know what sourcefiles
    ///    you consulted during the compile (including headers, etc.) write that info to this file.
    ///    For more information, see [this documentation](https://www.gnu.org/software/make/manual/html_node/Automatic-Prerequisites.html).
    /// # Returns
    /// * Returns a path to the compiled object file, should be located in the intermediate dir.
    fn compile_one(path: &Path, intermediate_dir: &Path, configuration: &Configuration,dependency_path: &Path) -> PathBuf;
}

///Implement this trait to bring in your linker
pub trait LinkStep {
    fn link_all(object_files: &[PathBuf], out_dir: &Path, lib_name: &str, configuration: &Configuration) -> PathBuf;
}
