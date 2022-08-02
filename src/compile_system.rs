use crate::{CompileStep, CompileSettings, CompileSettingsBuilder, PathType};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::str::FromStr;
use std::fs::create_dir_all;

/**
Implements a compile phase.  This compiles multiple sourcefiles into multiple object files.

This is the first phase of a [super::build_system::BuildSystem].  Alternatively, it may
be used on its own, for tasks that are one-file-per-product.
*/
pub struct CompileSystem<Compiler: CompileStep> {
    compiler: PhantomData<Compiler>
}

impl<Compiler: CompileStep> CompileSystem<Compiler> {
    pub(crate) fn compile_all(settings: &CompileSettings) -> Vec<PathBuf> {
        let source_files = settings.source_strategy.resolve::<Compiler>();
        if source_files.is_empty() { panic!("Nothing to compile!") }
        //todo: multithread this?
        //todo: Incremental compiles?
        //create intermediate path if it does not exist
        create_dir_all(&settings.intermediate_path).unwrap();
        let mut dependency_path = settings.intermediate_path.clone();
        dependency_path.push("dependency");
        let mut compile_results = Vec::new();
        for source_file in source_files {
            let flags_iter = settings.flags.iter().map(|e| e as &str);
            let result = Compiler::compile_one(&source_file,&settings.product_path,  &settings.configuration, &dependency_path,flags_iter);
            compile_results.push(result);

            super::dependency_parser::tell_cargo_about_dependencies(&dependency_path);
        }
        compile_results
    }
}

impl<Compiler: CompileStep> CompileSystem<Compiler> {
    ///Compiles using the settings specified.
    ///
    /// Returns a path to the final product.
    pub fn build(settings: &CompileSettings) -> Vec<PathBuf> {
        CompileSystem::<Compiler>::compile_all(&settings)
    }

    ///Build using no special settings.  Usually the entrypoint from `build.rs`
    ///
    /// `exe_path`: The path, relative to the built exe file, where the output should be located.
    /// For example if your target is built in `target\debug\my.exe` and `exe_path` is `assets\product.dll`, the final location will be
    /// `target\debug\assets\product.dll`.  The intermediate directories will be created if they do not already exist.
    pub fn build_rs(exe_path: PathBuf) -> Vec<PathBuf> {
        let settings = CompileSettingsBuilder::new().intermediate_path(PathType::EXERelative(exe_path)).finish();
        Self::build(&settings)
    }
}

///Walks a directory, looking for sourcefiles
///
/// Returns its output in its argument, because it makes the memory
/// faster for recursion
pub fn dir_walk(base: &Path, extension: &str, output: &mut Vec<PathBuf>) {
    for item in std::fs::read_dir(base).expect(&format!("Problem reading dir at {:?}",base)) {
        let path = item.unwrap().path();
        if path.is_dir() {
            dir_walk(&path, extension, output);
        }
        else if path.is_file() { //I'm not 100% sure what other options there are, but ok
            if path.extension() == Some(&OsString::from_str(extension).unwrap()) {
                output.push(path);
            }
        }
    }
}