use std::marker::PhantomData;
use std::path::{PathBuf};

use crate::build_settings::{BuildSettings, BuildSettingsBuilder, PathType};
use crate::traits::{CompileStep, LinkStep};
use crate::CompileSystem;

///A build system with separate compile and link steps.
///
/// Compare with [super::CompileSystem], a version with only compile steps.
pub struct BuildSystem<Compiler,Linker> {
    compiler: PhantomData<Compiler>,
    linker: PhantomData<Linker>,
}

impl<Compiler: CompileStep,Linker: LinkStep> BuildSystem<Compiler,Linker> {
    ///Compiles/links using the settings specified.
    ///
    /// Returns a path to the final product.
    pub fn build(settings: &BuildSettings) -> PathBuf {
        let compile_results = CompileSystem::<Compiler>::compile_one(&settings.compile_settings);
        Linker::link_all(&compile_results, &settings.product_path,&settings.product_name,  &settings.compile_settings.configuration)
    }

    ///Build using no special settings.  Usually the entrypoint from `build.rs`
    pub fn build_rs(exe_path: PathBuf) -> PathBuf {
        let settings = BuildSettingsBuilder::new().product_path(PathType::EXERelative(exe_path)).finish();
        Self::build(&settings)
    }
}
