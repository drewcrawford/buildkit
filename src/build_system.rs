use std::ffi::OsString;
use std::fs::create_dir_all;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::build_settings::{BuildSettings, BuildSettingsBuilder, PathType};
use crate::traits::{CompileStep, LinkStep};

///Actual build system, specialized via compiler/linker
pub struct BuildSystem<Compiler,Linker> {
    compiler: PhantomData<Compiler>,
    linker: PhantomData<Linker>,
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

impl<Compiler: CompileStep,Linker: LinkStep> BuildSystem<Compiler,Linker> {
    ///Compiles/links using the settings specified.
    ///
    /// Returns a path to the final product.
    pub fn build(settings: &BuildSettings) -> PathBuf {
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
            let result = Compiler::compile_one(&source_file, &settings.intermediate_path,  &settings.configuration, &dependency_path);
            compile_results.push(result);

            super::dependency_parser::tell_cargo_about_dependencies(&dependency_path);
        }


        Linker::link_all(&compile_results, &settings.product_path,&settings.product_name,  &settings.configuration)
    }

    ///Build using no special settings.  Usually the entrypoint from `build.rs`
    pub fn build_rs(exe_path: PathBuf) -> PathBuf {
        let settings = BuildSettingsBuilder::new().product_path(PathType::EXERelative(exe_path)).finish();
        Self::build(&settings)
    }
}
