use std::path::PathBuf;
use std::str::FromStr;

use crate::{CompileSettings};
use crate::compile_settings::CompileSettingsBuilder;

#[derive(Copy,Clone)]
pub enum Configuration {
    Debug,
    Release
}

pub struct BuildSettings {
    pub(crate) compile_settings: CompileSettings,
    ///The "final path" for the product
    pub(crate) product_path: PathBuf,
    ///The product name, e.g. libname or similar
    pub(crate) product_name: String,
}



impl BuildSettings {
    ///builder-pattern for `BuildSettings`
    pub fn build() -> BuildSettingsBuilder { BuildSettingsBuilder::new() }

    ///Automatically builds all build settings.
    pub fn auto() -> BuildSettings { BuildSettingsBuilder::new().finish() }
}

///Builder pattern for [BuildSettings]
///
/// `https://doc.rust-lang.org/1.0.0/style/ownership/builders.html`
#[derive(Clone)]
pub struct BuildSettingsBuilder{
    compile_settings: Option<CompileSettings>,
    product_path: Option<PathType>,
    //todo: Allow other types to be set
}


impl BuildSettingsBuilder {
    pub fn new() -> Self {
        BuildSettingsBuilder{ compile_settings: None, product_path: None}
    }

    ///Specify where products are stored
    pub fn product_path(&mut self, path: PathType) -> &mut BuildSettingsBuilder {
        self.product_path = Some(path);
        self
    }
    pub fn compile_settings(&mut self, settings: CompileSettings) -> &mut BuildSettingsBuilder {
        self.compile_settings = Some(settings);
        self
    }
    pub fn finish(&self) -> BuildSettings {
        let compile_settings = match &self.compile_settings {
            //use 'link' version when part of `BuildSettings`
            None => {CompileSettingsBuilder::new()._finish(true)}
            Some(settings) => {settings.clone()}
        };

        let product_path: PathBuf = match &self.product_path {
            Some(path) => path.path().to_path_buf(),
            None => {
                PathType::EXERelative(PathBuf::new()).path().to_path_buf()
            }
        };



        let product_name = std::env::var("CARGO_PKG_NAME").unwrap();
        BuildSettings {
            compile_settings,
            product_path,
            product_name,
        }
    }
}

#[non_exhaustive]
#[derive(Clone)]
pub enum PathType {
    ///Path will take on some path relative to exe in target directory as part of a build process
    EXERelative(PathBuf),
    ///Path will be as specified
    Exact(PathBuf),
}

impl PathType {
    pub(crate) fn path(&self) -> PathBuf {
        match self {
            PathType::EXERelative(relative) => {
                let out_dir = std::env::var("OUT_DIR").expect("Must set `OUT_DIR` if not setting product_path or using PathType::EXERelative");
                let mut product_path = PathBuf::from_str(&out_dir).unwrap();
                product_path.pop(); //out
                product_path.pop(); //target_name
                product_path.pop(); //'build'
                product_path.push(relative);
                product_path
            }
            PathType::Exact(exact) => exact.to_path_buf(),
        }
    }
}
