//! Generate only the selected API families against CMake's actual configuration.
use super::config::Configuration;
use super::manifest::Installation;
#[path = "binding_support.rs"]
mod support;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
    fs,
    path::Path,
};
use support::{Inventory, document_native_examples, normalize_bindings};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct NativeComments;

impl bindgen::callbacks::ParseCallbacks for NativeComments {
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(document_native_examples(comment))
    }
}

const FAMILIES: &[(&str, &[&str])] = &[
    ("base", &["BASE"]),
    ("local", &["CPU"]),
    ("task", &["TASK"]),
    ("tokenizer", &["TOKENIZER"]),
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
// The compiler entry header below bounds the reviewed public C API families.
// Each generated function participates in the executable link-address tests.
const MLIR_INTEROP_FUNCTIONS: &str = "mlir.*";
const COMPILER_TOOL_FUNCTIONS: &str = "iree(CompilerRunMain|OptRunMain|MlirLspServerRunMain|CompilerRunLldMain|LinkRunMain|ReduceRunMain)";
const SUPPORT_SOURCES: &[&str] = &["runtime/src/iree/hal/utils/profile_file.c"];

pub fn generate(
    out: &Path,
    installed: &Installation,
    features: &Configuration,
    target: &str,
) -> Result<()> {
    generate_impl(out, installed, features, target, false, &[])
}

pub fn generate_documentation(
    out: &Path,
    installed: &Installation,
    target: &str,
    includes: &[String],
) -> Result<()> {
    let features = Configuration::new(super::documentation::features(target)?);
    generate_impl(out, installed, &features, target, true, includes)
}

fn generate_impl(
    out: &Path,
    installed: &Installation,
    features: &Configuration,
    target: &str,
    documentation: bool,
    includes: &[String],
) -> Result<()> {
    let build = &installed.build;
    let flags_available = installed.has_static_symbols(&["iree_flags_parse_checked"])?;
    let native = Path::new(env!("CARGO_MANIFEST_DIR")).join("native");
    let mut arguments = fs::read_to_string(build.join("iree-bindgen-args.txt"))?;
    for include in includes {
        arguments.push('\n');
        arguments.push_str(include);
    }
    if features.has("bare-metal") {
        // Upstream emits this as a compile option, not a target definition.
        // Keep both generated layouts and C forwarders on the same no-lock ABI.
        arguments.push_str("\n-DIREE_SYNCHRONIZATION_DISABLE_UNSAFE=1\n");
    }
    if features.uses_compiler() {
        // Public MLIR dialect C headers include TableGen output that is not a
        // transitive include directory of the unified runtime.
        let generated = build.join("iree/llvm-project/tools/mlir/include");
        if !generated
            .join("mlir/Dialect/Linalg/Passes.capi.h.inc")
            .is_file()
        {
            return Err(format!(
                "native compiler installation lacks generated MLIR C headers in {}",
                generated.display()
            )
            .into());
        }
        arguments.push_str(&format!("\n-I{}\n", generated.display()));
    }
    let sysroot = build.join("iree-bindgen-sysroot.txt");
    if sysroot.is_file() {
        arguments.push_str(&fs::read_to_string(sysroot)?);
    }
    // Override cached wrapper mirrors in both generation and the audit replay.
    arguments = format!("-I{}\n{arguments}", native.display());
    let clang: Vec<_> = arguments
        .lines()
        .filter(|line| !line.is_empty() && *line != "-D" && !line.starts_with("-DIREE_NATIVE_"))
        .collect();
    let parse_arguments = clang
        .iter()
        .map(|argument| (*argument).to_owned())
        .chain([
            format!("--target={target}"),
            "-x".into(),
            "c".into(),
            "-std=c11".into(),
        ])
        .collect::<Vec<_>>();
    let capabilities = features.native_capabilities();
    let mut flags: Vec<_> = FAMILIES
        .iter()
        .flat_map(|(_, flags)| *flags)
        .copied()
        .collect();
    flags.extend([
        "COMPILER_TOOLS",
        "BASE",
        "FLAGS",
        "MPI",
        "IO_INDEX",
        "IO_PARAMETERS",
        "IO_FORMAT_GGUF",
        "IO_FORMAT_IRPA",
        "IO_FORMAT_SAFETENSORS",
    ]);
    flags.sort_unstable();
    flags.dedup();
    let mut shared = BTreeMap::<&str, BTreeSet<String>>::new();
    let mut root_helpers = BTreeSet::new();
    let mut index = String::new();
    let mut tests = String::new();
    let mut forwarders = Vec::new();
    let mut audit_headers = Vec::new();
    let mut support_sources = SUPPORT_SOURCES.to_vec();
    for (functions, source) in [
        (
            &[
                "iree_hal_profile_statistics_sink_base",
                "iree_hal_profile_statistics_sink_create",
                "iree_hal_profile_statistics_sink_dropped_record_count",
                "iree_hal_profile_statistics_sink_find_function_name",
                "iree_hal_profile_statistics_sink_for_each_row",
                "iree_hal_profile_statistics_sink_fprint",
                "iree_hal_profile_statistics_sink_release",
                "iree_hal_profile_statistics_sink_retain",
                "iree_hal_profile_statistics_sink_row_count",
                "iree_hal_profile_statistics_sink_scale_duration_to_ns",
            ][..],
            "runtime/src/iree/hal/utils/statistics_sink.c",
        ),
        (
            &[
                "iree_hal_file_cache_create",
                "iree_hal_file_cache_retain",
                "iree_hal_file_cache_release",
                "iree_hal_file_cache_trim",
                "iree_hal_file_cache_lookup",
            ][..],
            "runtime/src/iree/hal/utils/file_cache.c",
        ),
        (
            &[
                "iree_hal_mpi_is_configured",
                "iree_hal_mpi_channel_provider_create",
                "iree_hal_mpi_channel_provider_isa",
                "iree_hal_mpi_channel_provider_symbols",
            ][..],
            "runtime/src/iree/hal/utils/mpi_channel_provider.c",
        ),
        (
            &["iree_hal_mpi_library_load", "iree_hal_mpi_result_to_status"][..],
            "runtime/src/iree/hal/utils/libmpi.c",
        ),
    ] {
        if source.contains("mpi") && !features.has("mpi") {
            continue;
        }
        if installed.needs_static_support(functions)? {
            support_sources.push(source);
        }
    }
    if capabilities.contains(&"tokenizer") {
        support_sources.extend([
            "runtime/src/iree/tokenizer/normalizer/passthrough.c",
            "runtime/src/iree/tokenizer/normalizer/deferred.c",
            "runtime/src/iree/tokenizer/segmenter/passthrough.c",
            "runtime/src/iree/tokenizer/decoder/passthrough.c",
        ]);
    }
    for &(family, enabled) in FAMILIES {
        let selected = match family {
            "base" => true,
            "local" => {
                features.has("cpu") || features.has("local-sync") || features.has("local-task")
            }
            _ => capabilities.contains(&family),
        };
        if !selected {
            continue;
        }
        let header = out.join(format!("{family}.h"));
        let mut contents = flags
            .iter()
            .map(|flag| {
                format!(
                    "#define IREE_NATIVE_{flag} {}\n",
                    u8::from(match *flag {
                        "FLAGS" => flags_available && family == "base",
                        "COMPILER_TOOLS" => features.has("compiler-tools") && family == "compiler",
                        "MPI" => features.has("mpi") && family == "base",
                        "IO_INDEX" => features.has("io-index") && family == "base",
                        "IO_PARAMETERS" => features.has("io-parameters") && family == "base",
                        "IO_FORMAT_GGUF" => features.has("io-format-gguf") && family == "base",
                        "IO_FORMAT_IRPA" => features.has("io-format-irpa") && family == "base",
                        "IO_FORMAT_SAFETENSORS" =>
                            features.has("io-format-safetensors") && family == "base",
                        _ => enabled.contains(flag),
                    })
                )
            })
            .collect::<String>();
        contents.push_str("#include \"wrapper.h\"\n");
        fs::write(&header, contents)?;
        audit_headers.push(serde_json::json!({
            "family": family,
            "header": header,
        }));
        let mut builder = bindgen::Builder::default()
            .header(header.to_string_lossy())
            .clang_args(&parse_arguments)
            .use_core()
            .derive_default(true)
            .wrap_unsafe_ops(true)
            .prepend_enum_name(false)
            .formatter(bindgen::Formatter::Prettyplease)
            .wrap_static_fns(true)
            .wrap_static_fns_path(out.join(family))
            // The entry header is generated in OUT_DIR, not a source input.
            .parse_callbacks(Box::new(
                bindgen::CargoCallbacks::new().rerun_on_header_files(false),
            ))
            .parse_callbacks(Box::new(NativeComments))
            .allowlist_item("(iree_.*|iree[A-Z].*|IREE_.*)")
            .blocklist_item("IREE_NATIVE_.*")
            // These are implementation utilities, even if public entry headers
            // include them transitively. No Rust workflow shares their state.
            .blocklist_function("iree_(base64|json|sysfs|prng|spsc|mpsc|atomic_freelist|atomic_slist|fpu_state)_.*")
            .blocklist_function("iree_(cpu|processor|file_path|path|uri)_.*")
            .blocklist_function("iree_(abort|debug_break|breakpoint)")
            .blocklist_function("iree_(arena|arena_block_pool|block_pool)_.*")
            .blocklist_function("iree_hal_(resource_set|deferred_command_buffer|deferred_work_queue|collective_batch|profile_event_ring|stream_tracing|memory_trace|cpu_slab_provider|passthrough_pool|pool_set|slab_cache|local_executable|local_profile|inline_command_buffer|inline_dispatch|transient_buffer|cpu_buffer|sync_event|device_queue_emulated|queue_emulated)_.*")
            .blocklist_function("iree_hal_memory_(arena|fixed_block|tlsf|trace)_.*")
            .blocklist_function("iree_hal_device_queue_tlsf_pool")
            .blocklist_function("iree_(elf|fatelf)_.*")
            .blocklist_item("iree_vm_bytecode_block(_.*)?")
            .blocklist_item("iree_vm_bytecode_(dispatch_.*|module_state_t|import_t|frame_storage_t)")
            .blocklist_item("iree_vm_registers_t")
            .blocklist_function("iree_vm_bytecode_(module_flatbuffer_verify|function_verify|disassemble_op|trace_disassembly|disassemble_function)")
            // Declared but not implemented by the pinned upstream release.
            .blocklist_function("iree_vm_invocation_(create|retain|release|query_status|outputs|await|cancel)|iree_async_query_affinities")
            // modules/hal/types.h declares cache VM adapters, but types.c neither
            // defines their registration nor implements them in IREE 3.13.0.
            // Keep the implemented generic HAL cache API available.
            .blocklist_item("iree_hal_executable_cache_(registration|type|isa|retain_ref|move_ref|deref|check_deref|check_deref_or_null)")
            .raw_line("use super::*;");
        if family == "compiler" {
            builder = builder.blocklist_function("mlirLogicalResult.*");
        }
        if family == "compiler" {
            builder = builder.allowlist_function(format!(
                "{MLIR_INTEROP_FUNCTIONS}|{COMPILER_TOOL_FUNCTIONS}"
            ));
            if !features.has("compiler-tools") {
                builder = builder.blocklist_function(COMPILER_TOOL_FUNCTIONS);
            }
        }
        if !installed.threading || features.has("bare-metal") {
            builder = builder.blocklist_function("iree_thread_.*");
        }
        if !flags_available {
            builder = builder.blocklist_function("iree_flags?_.*");
        }
        if !features.has("mpi") {
            builder = builder.blocklist_item("iree_hal_mpi_.*|IREE_HAL_MPI_.*");
        }
        if family == "amdgpu" {
            // HSA info queries expose enum and output types through void pointers.
            builder = builder.allowlist_type("hsa_.*").allowlist_var("HSA_.*");
        }
        for dependency in match family {
            "base" => &[][..],
            "local-sync" | "local-task" => &["base", "local", "task"][..],
            _ => &["base"][..],
        } {
            if let Some(names) = shared.get(dependency)
                && !names.is_empty()
            {
                builder = builder.blocklist_item(format!(
                    "({})",
                    names
                        .iter()
                        .map(|name| name.strip_prefix("r#").unwrap_or(name).to_owned())
                        .collect::<Vec<_>>()
                        .join("|")
                ));
            }
        }
        // Prettyplease keeps inspection and external audit output stable.
        // Export discovery uses syntax rather than depending on that formatting.
        let text = normalize_bindings(&builder.generate()?.to_string())?;
        let inventory = Inventory::parse(&text)?;
        if family == "base" {
            println!("cargo:rustc-check-cfg=cfg(iree_native_flags)");
        }
        if family == "base" && flags_available {
            // Some archives contain a no-op checked parser when CLI is disabled.
            // Admit the safe API only when the matched headers enable the registry.
            let cli_enabled = syn::parse_file(&text)?.items.into_iter().any(|item| {
                let syn::Item::Const(item) = item else {
                    return false;
                };
                if item.ident != "IREE_FLAGS_ENABLE_CLI" {
                    return false;
                }
                let syn::Expr::Lit(value) = *item.expr else {
                    return false;
                };
                matches!(value.lit, syn::Lit::Int(value) if value.base10_digits() == "1")
            });
            if cli_enabled {
                println!("cargo:rustc-cfg=iree_native_flags");
            }
        }
        fs::write(out.join(format!("{family}.rs")), text.as_bytes())?;
        let exports = inventory.exports;
        let module = match family {
            "base" => "raw_base".into(),
            "compiler" => "raw_compiler".into(),
            _ => family.replace('-', "_"),
        };
        let availability = if documentation {
            match family {
                "base" => String::new(),
                "local" => {
                    "any(feature = \"cpu\", feature = \"local-sync\", feature = \"local-task\")"
                        .into()
                }
                "task" => {
                    "any(feature = \"task\", feature = \"cpu\", feature = \"local-task\")".into()
                }
                "metal" => "all(feature = \"metal\", target_vendor = \"apple\")".into(),
                "amdgpu" => "all(feature = \"amdgpu\", target_os = \"linux\")".into(),
                "webgpu" => "all(feature = \"webgpu\", target_arch = \"wasm32\")".into(),
                name => format!("feature = {name:?}"),
            }
        } else {
            String::new()
        };
        let attribute = if availability.is_empty() {
            String::new()
        } else {
            format!("#[cfg_attr(docsrs, doc(cfg({availability})))]\n")
        };
        index.push_str(&format!(
            "{attribute}#[path = \"{family}.rs\"]\npub mod {module};\n"
        ));
        for name in &exports {
            if name.starts_with("__") && !root_helpers.insert(name.clone()) {
                continue;
            }
            let visibility = if name.starts_with("__") { "" } else { "pub " };
            let extra_availability = if documentation {
                documentation_availability(name)
            } else {
                None
            };
            let extra_attribute = extra_availability.map_or_else(String::new, |feature| {
                format!("#[cfg_attr(docsrs, doc(cfg(feature = {feature:?})))]\n")
            });
            let platform_attribute = if documentation && name == "iree_native_load_compiler" {
                "#[cfg_attr(docsrs, doc(cfg(target_env = \"msvc\")))]\n"
            } else {
                ""
            };
            index.push_str(&format!(
                "{attribute}{extra_attribute}{platform_attribute}{visibility}use {module}::{name};\n"
            ));
        }
        shared.insert(family, exports);
        tests.push_str(&format!("#[test]\nfn link_{module}() {{\n"));
        for function in inventory.functions {
            tests.push_str(&format!(
                "std::hint::black_box(super::{function} as *const ());\n",
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
        if documentation {
            index
        } else {
            format!("{index}\n#[cfg(test)] mod link_tests;\n")
        },
    )?;
    if documentation {
        return Ok(());
    }
    let binding_source = clang
        .iter()
        .filter_map(|argument| argument.strip_prefix("-I"))
        .map(Path::new)
        .find(|include| include.join("iree/runtime/api.h").is_file())
        .and_then(Path::parent)
        .and_then(Path::parent)
        .ok_or("native binding configuration has no IREE runtime source include")?;
    fs::write(
        out.join("api-audit.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schema": 1,
            "target": target,
            "source": binding_source,
            "installation_source": installed.source,
            "fingerprint": installed.fingerprint,
            "version": installed.version,
            "capabilities": features.native_capabilities(),
            "compiler_loading": if features.has("compiler") && target.contains("msvc") {
                "msvc-direct-link-explicit-load"
            } else {
                "direct-link"
            },
            "support_sources": support_sources,
            "binding_support_sources": Vec::<std::path::PathBuf>::new(),
            "cxx_binding_support_sources": Vec::<std::path::PathBuf>::new(),
            "clang_arguments": parse_arguments,
            "headers": audit_headers,
        }))?,
    )?;
    fs::write(out.join("link_tests.rs"), tests)?;
    let mut compiler = cc::Build::new();
    compiler.include(&native);
    compiler.compiler(fs::read_to_string(build.join("iree-c-compiler.txt"))?.trim());
    compiler.files(forwarders).std("c11").warnings(false);
    let hal_interop = native.join("hal_interop.c");
    println!("cargo:rerun-if-changed={}", hal_interop.display());
    compiler.file(hal_interop);
    if features.has("compiler") && target.contains("msvc") {
        let compiler_runtime = native.join("compiler_runtime.c");
        println!("cargo:rerun-if-changed={}", compiler_runtime.display());
        compiler.file(compiler_runtime);
    }
    // These optional upstream utilities are outside the unified target closure.
    // Use their unmodified sources and the installation's compiler/configuration.
    for source in support_sources {
        let source = binding_source.join(source);
        println!("cargo:rerun-if-changed={}", source.display());
        compiler.file(source);
    }
    let mut included = HashSet::new();
    for argument in &clang {
        if let Some(include) = argument.strip_prefix("-I") {
            if included.insert(include) {
                compiler.include(include);
            }
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
    compiler.compile("iree_native_inline");
    Ok(())
}

fn documentation_availability(name: &str) -> Option<&'static str> {
    if name.starts_with("iree_hal_mpi_") || name.starts_with("IREE_HAL_MPI_") {
        Some("mpi")
    } else if name.starts_with("iree_io_parameter_index_provider")
        || name.starts_with("iree_io_parameter_provider")
        || name.starts_with("iree_io_parameters_")
        || name.starts_with("iree_io_parameter_module_")
        || name.starts_with("iree_io_parameters_module_")
    {
        Some("io-parameters")
    } else if name.starts_with("iree_io_parameter_index")
        || name.starts_with("iree_io_parameter_")
        || name.starts_with("iree_io_scope_map")
    {
        Some("io-index")
    } else if name.starts_with("iree_io_build_irpa") || name.starts_with("iree_io_parse_irpa") {
        Some("io-format-irpa")
    } else if name.starts_with("iree_io_parse_gguf") {
        Some("io-format-gguf")
    } else if name.starts_with("iree_io_parse_safetensors") {
        Some("io-format-safetensors")
    } else if [
        "ireeCompilerRunMain",
        "ireeOptRunMain",
        "ireeMlirLspServerRunMain",
        "ireeCompilerRunLldMain",
        "ireeLinkRunMain",
        "ireeReduceRunMain",
    ]
    .contains(&name)
    {
        Some("compiler-tools")
    } else {
        None
    }
}
