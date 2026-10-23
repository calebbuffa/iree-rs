//! Generate only the selected API families against CMake's actual configuration.
use super::config::Configuration;
use regex::Regex;
use std::{collections::BTreeMap, error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const FAMILIES: &[(&str, &[&str])] = &[
    ("base", &[]),
    ("local", &["CPU"]),
    ("task", &["TASK"]),
    ("local-sync", &["LOCAL_SYNC"]),
    ("local-task", &["TASK", "LOCAL_TASK"]),
    ("vulkan", &["VULKAN"]),
    ("cuda", &["CUDA"]),
    ("hip", &["HIP"]),
    ("amdgpu", &["AMDGPU"]),
    ("metal", &["METAL"]),
    ("null", &["NULL"]),
    ("webgpu", &["WEBGPU"]),
    ("compiler", &["COMPILER"]),
];

pub fn generate(out: &Path, build: &Path, features: &Configuration, target: &str) -> Result<()> {
    let mut arguments = fs::read_to_string(build.join("iree-bindgen-args.txt"))?;
    let sysroot = build.join("iree-bindgen-sysroot.txt");
    if sysroot.is_file() {
        arguments.push_str(&fs::read_to_string(sysroot)?);
    }
    let clang: Vec<_> = arguments
        .lines()
        .filter(|line| !line.is_empty() && *line != "-D" && !line.starts_with("-DIREE_RS_"))
        .collect();
    let symbols =
        Regex::new(r"(?m)^(?:pub (?:struct|union|type|const)|[ \t]*pub static(?: mut)?)\s+(\w+)")?;
    let functions = Regex::new(r"(?m)^[ \t]+pub fn (iree\w+)\(")?;
    let aliases = Regex::new(r"(?m)^pub use self::\w+ as (\w+);")?;
    let fences = Regex::new(r"(?m)^(\s*///) ```\s*$")?;
    let mut shared = BTreeMap::<&str, Vec<String>>::new();
    let mut index = String::new();
    let mut tests = String::new();
    let mut forwarders = Vec::new();
    for &(family, enabled) in FAMILIES {
        let selected = match family {
            "base" => true,
            "local" => {
                features.has("cpu") || features.has("local-sync") || features.has("local-task")
            }
            _ => features.native_capabilities().contains(&family),
        };
        if !selected {
            continue;
        }
        let header = out.join(format!("{family}.h"));
        let mut flags: Vec<_> = FAMILIES
            .iter()
            .flat_map(|(_, flags)| *flags)
            .copied()
            .collect();
        flags.sort_unstable();
        flags.dedup();
        let mut contents = flags
            .iter()
            .map(|flag| {
                format!(
                    "#define IREE_RS_{flag} {}\n",
                    u8::from(enabled.contains(flag))
                )
            })
            .collect::<String>();
        contents.push_str("#include \"wrapper.h\"\n");
        fs::write(&header, contents)?;
        let mut builder = bindgen::Builder::default()
            .header(header.to_string_lossy())
            .clang_args(&clang)
            .clang_arg(format!("--target={target}"))
            .clang_args(["-x", "c", "-std=c11"])
            .use_core()
            .derive_default(true)
            .prepend_enum_name(false)
            .formatter(bindgen::Formatter::Prettyplease)
            .wrap_static_fns(true)
            .wrap_static_fns_path(out.join(family))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .allowlist_item("(iree_.*|iree[A-Z].*|IREE_.*)")
            .blocklist_item("IREE_RS_.*")
            // Declared but not implemented by the pinned upstream release.
            .blocklist_function("iree_vm_invocation_(create|retain|release|query_status|outputs|await|cancel)|iree_async_query_affinities|iree_hal_executable_cache_(type|isa|retain_ref|move_ref|deref|check_deref|check_deref_or_null)")
            .raw_line("use super::*;");
        for dependency in match family {
            "base" => &[][..],
            "local-sync" | "local-task" => &["base", "local", "task"][..],
            _ => &["base"][..],
        } {
            if let Some(names) = shared.get(dependency)
                && !names.is_empty()
            {
                builder = builder.blocklist_item(format!("({})", names.join("|")));
            }
        }
        // Keep the declaration formatting predictable for export/link discovery,
        // without requiring rustfmt to be installed by consumers.
        let text = builder.generate()?.to_string();
        let text = fences.replace_all(&text, "$1 ```text");
        fs::write(out.join(format!("{family}.rs")), text.as_bytes())?;
        let mut exports = symbols
            .captures_iter(&text)
            .map(|item| item[1].to_owned())
            .collect::<Vec<_>>();
        exports.extend(
            functions
                .captures_iter(&text)
                .map(|item| item[1].to_owned()),
        );
        exports.extend(aliases.captures_iter(&text).map(|item| item[1].to_owned()));
        exports.sort_unstable();
        exports.dedup();
        let module = family.replace('-', "_");
        index.push_str(&format!("#[path = \"{family}.rs\"]\npub mod {module};\n"));
        for name in &exports {
            if !name.starts_with("__") {
                index.push_str(&format!("pub use {module}::{name};\n"));
            }
        }
        shared.insert(family, exports);
        tests.push_str(&format!("#[test]\nfn link_{module}() {{\n"));
        for function in functions.captures_iter(&text) {
            tests.push_str(&format!(
                "std::hint::black_box(super::{} as *const ());\n",
                &function[1]
            ));
        }
        tests.push_str("}\n");
        let c = out.join(format!("{family}.c"));
        if c.is_file() {
            forwarders.push(c);
        }
    }
    fs::write(
        out.join("bindings.rs"),
        format!("{index}\n#[cfg(test)] mod link_tests;\n"),
    )?;
    fs::write(out.join("link_tests.rs"), tests)?;
    let mut compiler = cc::Build::new();
    compiler.compiler(fs::read_to_string(build.join("iree-c-compiler.txt"))?.trim());
    compiler.files(forwarders).std("c11").warnings(false);
    for argument in clang {
        if let Some(include) = argument.strip_prefix("-I") {
            compiler.include(include);
        } else if let Some(define) = argument.strip_prefix("-D") {
            let (name, value) = define
                .split_once('=')
                .map_or((define, None), |(name, value)| (name, Some(value)));
            compiler.define(name, value);
        } else {
            compiler.flag(argument);
        }
    }
    if target.contains("msvc") {
        compiler.flag("/Zc:preprocessor");
    } else {
        compiler.flag("-std=c11");
    }
    compiler.compile("iree_rs_inline");
    Ok(())
}
