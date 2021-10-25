use std::io::Read;
use std::path::{Path,PathBuf};
use std::str::FromStr;

pub fn tell_cargo_about_dependencies(dependency_file: &Path) {
    let mut file = std::fs::File::open(dependency_file).unwrap();
    let mut str = String::new();
    let _ = file.read_to_string(&mut str).unwrap();
    for dependency in parse(&str) {
        println!("cargo:rerun-if-changed='{}'",dependency.to_str().unwrap());
    }
}

// Accepts input like `whatever: file.metal file.h` supporting newlines and escapes
fn parse(parse_me: &str) -> Vec<PathBuf> {
    let mut iter = parse_me.chars();
    //first we advance until we find a `:`
    let _ = iter.by_ref().take_while(|p| p != &':').for_each(drop);
    //drop whitespace
    let _ = iter.by_ref().take_while(|p| p != &' ').for_each(drop);

    let mut out = Vec::new();
    //if we immediately parsed `\` in the previoius iteration
    let mut escaped = false;
    let mut current_str = String::new();
    for char in iter {
        //println!("match {}",char);
        match char {
            ' ' => {
                if current_str.len() == 0 {
                    //waiting for next string to start
                }
                else if escaped {
                    current_str.push(char);
                    escaped = false;
                }
                else {
                    out.push(PathBuf::from_str(&current_str).unwrap());
                    current_str = String::new();
                }
            }
            '\\' => {
                if !escaped {
                    escaped = true;
                }
                else { // matching '\\'
                    escaped = false;
                    current_str.push('\\');
                }

            }
            '\n' => {
                escaped = false;
            }
            other => {
                assert!(!escaped, "Unknown escape sequence \\{}",other);
                current_str.push(other);
            }
        }
    }
    out.push(PathBuf::from_str(&current_str).unwrap());
    out
}

#[test] fn test_parse() {
    let txt = r#"depedencies: /Users/drew/Code/winspike/metal-build/tests/test.metal \
  /Users/drew/Code/winspike/metal-build/tests/example1.h \
  /Users/drew/Code/winspike/metal-build/tests/example\ 2.h"#;
    let deps = parse(txt);
    assert_eq!(deps[0], PathBuf::from_str("/Users/drew/Code/winspike/metal-build/tests/test.metal").unwrap());
    assert_eq!(deps[1],  PathBuf::from_str("/Users/drew/Code/winspike/metal-build/tests/example1.h").unwrap());
    assert_eq!(deps[2],  PathBuf::from_str("/Users/drew/Code/winspike/metal-build/tests/example 2.h").unwrap());
    assert_eq!(deps.len(), 3);
}
#[test] fn test_escaped() {
    let txt = r"C:\\path\\file.spirv: C:\\path\\file.o";
    let _ = parse(txt);
}