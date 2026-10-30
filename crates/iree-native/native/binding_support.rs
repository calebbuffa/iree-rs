//! Syntax-aware inventory and comment policy for generated bindings.
use std::collections::BTreeSet;
use syn::{ForeignItem, Item, UseTree, Visibility};
#[path = "binding_policy.rs"]
mod policy;
pub use policy::normalize_bindings;

/// Generated public names needed for family sharing and root re-exports.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Inventory {
    /// Public declarations, aliases, and private-root bindgen helper imports.
    pub exports: BTreeSet<String>,
    /// Native IREE/MLIR functions that require executable link smoke tests.
    pub functions: BTreeSet<String>,
}

impl Inventory {
    /// Inspect Rust syntax, independently of bindgen's whitespace or formatter.
    pub fn parse(source: &str) -> syn::Result<Self> {
        let mut inventory = Self::default();
        for item in syn::parse_file(source)?.items {
            let (visibility, name) = match item {
                Item::Struct(item) => (item.vis, item.ident),
                Item::Union(item) => (item.vis, item.ident),
                Item::Enum(item) => (item.vis, item.ident),
                Item::Type(item) => (item.vis, item.ident),
                Item::Const(item) => (item.vis, item.ident),
                Item::Static(item) => (item.vis, item.ident),
                Item::ForeignMod(module) => {
                    for item in module.items {
                        match item {
                            ForeignItem::Fn(item) if public(&item.vis) => {
                                let name = item.sig.ident.to_string();
                                if name.starts_with("iree")
                                    || name.starts_with("IREE_")
                                    || name.starts_with("mlir")
                                {
                                    inventory.functions.insert(name.clone());
                                    inventory.exports.insert(name);
                                }
                            }
                            ForeignItem::Static(item) if public(&item.vis) => {
                                inventory.exports.insert(item.ident.to_string());
                            }
                            _ => {}
                        }
                    }
                    continue;
                }
                Item::Use(item) if public(&item.vis) => {
                    collect_aliases(&item.tree, &mut inventory.exports);
                    continue;
                }
                _ => continue,
            };
            if public(&visibility) {
                inventory.exports.insert(name.to_string());
            }
        }
        Ok(inventory)
    }
}

fn public(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Public(_))
}

fn collect_aliases(tree: &UseTree, names: &mut BTreeSet<String>) {
    match tree {
        UseTree::Rename(alias) => {
            names.insert(alias.rename.to_string());
        }
        UseTree::Path(path) => collect_aliases(&path.tree, names),
        UseTree::Group(group) => {
            for tree in &group.items {
                collect_aliases(tree, names);
            }
        }
        _ => {}
    }
}

/// Untyped upstream examples are C or pseudocode, never Rust doctests.
///
/// Preserve closing fences and explicitly typed examples. This runs on source
/// comments through bindgen, not on Rust declarations after code generation.
pub fn document_native_examples(comment: &str) -> String {
    let mut fence = None;
    let examples: String = comment
        .split_inclusive('\n')
        .map(|line| {
            let trimmed = line.trim_start();
            let Some(marker @ ('`' | '~')) = trimmed.chars().next() else {
                return line.to_owned();
            };
            let width = trimmed.chars().take_while(|c| *c == marker).count();
            if width < 3 {
                return line.to_owned();
            }
            let suffix = &trimmed[width..];
            if let Some((opening, length)) = fence {
                if opening == marker && width >= length && suffix.trim().is_empty() {
                    fence = None;
                }
                return line.to_owned();
            }
            fence = Some((marker, width));
            if suffix.trim().is_empty() {
                let end = line.len() - suffix.len();
                format!("{}text{}", &line[..end], suffix)
            } else {
                line.to_owned()
            }
        })
        .collect();
    separate_list_paragraphs(&examples)
}

/// C comments commonly resume prose immediately after a parameter list. A blank
/// line makes that paragraph unambiguous in Markdown without changing its text.
fn separate_list_paragraphs(comment: &str) -> String {
    let mut output = String::new();
    let mut list_indent = None;
    let mut fence = None;
    for line in comment.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        if let Some(marker @ ('`' | '~')) = trimmed.chars().next() {
            let width = trimmed
                .chars()
                .take_while(|character| *character == marker)
                .count();
            if width >= 3 {
                match fence {
                    None => fence = Some((marker, width)),
                    Some((opening, length))
                        if marker == opening
                            && width >= length
                            && trimmed[width..].trim().is_empty() =>
                    {
                        fence = None;
                    }
                    _ => {}
                }
                list_indent = None;
                output.push_str(line);
                continue;
            }
        }
        if fence.is_none() {
            if trimmed.is_empty() {
                list_indent = None;
            } else if ["- ", "* ", "+ "]
                .iter()
                .any(|marker| trimmed.starts_with(marker))
            {
                list_indent =
                    Some(list_indent.map_or(indent, |previous: usize| previous.min(indent)));
            } else if list_indent.is_some_and(|previous| indent <= previous) {
                output.push_str(if line.ends_with("\r\n") { "\r\n" } else { "\n" });
                list_indent = None;
            }
        }
        output.push_str(line);
    }
    output
}
