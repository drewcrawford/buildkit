use std::path::{Path, PathBuf};

use crate::build_settings::Configuration;
use std::ffi::{OsStr};

///Implement this trait to bring in your compiler.
///
/// A CompileStep compiles one sourcefile to one object file.
pub trait CompileStep {
    ///The extension to scan for.  We will use this to create individual `CompileStep`.
    const SOURCE_FILE_EXTENSION: &'static str;

    ///Compile one file, placing the output in the intermediate dir.
    ///
    /// # args
    /// * `path`: Path to the source file
    /// * `intermediate_dir`: Output location for object files.  To get a path for storing your object file, consider calling `suggest_intermediate_file`.
    /// * `configuration`: Holds build settings
    /// * `dependency_path`: Output file contining discovered dependencies.  If you know what sourcefiles
    ///    you consulted during the compile (including headers, etc.) write that info to this file.
    ///    For more information, see [this documentation](https://www.gnu.org/software/make/manual/html_node/Automatic-Prerequisites.html).
    /// * `flags`: Compiler flags.
    /// # Returns
    /// * Returns a path to the compiled object file, should be located in the intermediate dir.
    ///
    fn compile_one<'a>(path: &Path,intermediate_dir: &Path, configuration: &Configuration,dependency_path: &Path,flags: impl Iterator<Item=&'a String>) -> PathBuf;
}

///Implement this trait to bring in your linker
pub trait LinkStep {
    fn link_all(object_files: &[PathBuf], out_dir: &Path, lib_name: &str, configuration: &Configuration) -> PathBuf;
}

/**
Quick helper function to compute a path for output files (e.g., `-o <somewhere>`).


* `input_file`: Path to the input sourcefile, as seen in [CompileStep::compile_one].  We will take the file_stem from this.
* `intermediate_dir`: Path for the output directory, as seen in [CompileStep::compile_one].  We will use this folder for output.
* `file_extension`: Preferred file extension for object files, e.g. `o`.  This is specific to the [CompileStep].
*/
pub fn suggest_intermediate_file(input_file: &Path, intermediate_dir: PathBuf, file_extension: &OsStr) -> PathBuf {
    let file_base_name = input_file.file_stem().unwrap().to_owned();
    let mut file_with_extension = file_base_name;
    file_with_extension.push(".");
    file_with_extension.push(file_extension);

    let mut new_name = intermediate_dir;
    new_name.push(file_with_extension);
    new_name
}
