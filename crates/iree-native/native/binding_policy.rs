//! Normalize the pinned bindgen helper templates, not native declarations.
use std::collections::BTreeMap;
use syn::{
    Expr, FnArg, ImplItem, Item, Pat, ReturnType, Type,
    visit_mut::{self, VisitMut},
};

/// Keep bindgen's ABI declarations, while documenting its unsafe helper templates
/// and using ordinary integer casts for integer bitfield accessors.
///
/// Unknown unsafe helper templates fail generation rather than receiving a
/// blanket safety justification.
pub fn normalize_bindings(source: &str) -> syn::Result<String> {
    let mut file = syn::parse_file(source)?;
    let aliases = file
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Type(item) => Some((item.ident.to_string(), *item.ty.clone())),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let bitfield_records = file
        .items
        .iter()
        .filter_map(|item| {
            let Item::Struct(item) = item else {
                return None;
            };
            item.fields
                .iter()
                .any(|field| {
                    field
                        .ident
                        .as_ref()
                        .is_some_and(|name| name.to_string().starts_with("_bitfield_"))
                })
                .then(|| item.ident.to_string())
        })
        .collect::<std::collections::BTreeSet<_>>();
    for item in &mut file.items {
        let Item::Impl(item) = item else { continue };
        let owner = type_name(&item.self_ty).unwrap_or_default();
        let default = item.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|segment| segment.ident == "Default")
        });
        for member in &mut item.items {
            let ImplItem::Fn(method) = member else {
                continue;
            };
            let name = method.sig.ident.to_string();
            let bitfield = bitfield_records.contains(&owner) || name.starts_with("new_bitfield_");
            let safety = if default {
                Some(
                    "The entire storage is initialized to zero before assume_init; bindgen emits this Default only for C layouts whose fields admit the all-zero representation.",
                )
            } else if owner == "__BindgenBitfieldUnit" && method.sig.unsafety.is_some() {
                Some(
                    "The caller guarantees valid initialized bitfield storage and an in-bounds bit range; byte addressing and nested raw accesses stay within that storage.",
                )
            } else if bitfield && method.sig.unsafety.is_some() {
                Some(
                    "The caller guarantees valid initialized bitfield bytes, with exclusive access for writes; addr_of! does not form a reference to the record, and bindgen's fixed bit offsets and widths stay within the field.",
                )
            } else if owner == "__IncompleteArrayField" {
                Some(
                    "The caller guarantees the trailing allocation contains len initialized, aligned elements and that the returned slice obeys Rust's lifetime and aliasing requirements.",
                )
            } else if owner == "__BindgenUnionField" {
                Some(
                    "The caller guarantees this union member is initialized and valid as T, properly aligned, and borrowed with the returned reference's lifetime and exclusivity.",
                )
            } else {
                None
            };
            if method.sig.unsafety.is_some() {
                let contract = match owner.as_str() {
                    "__BindgenBitfieldUnit" => {
                        "The storage pointer must address allocated storage valid for the accessed bytes, initialized for reads and exclusively accessible for writes. The bit index or entire bit range must fit the storage; bit_width must not exceed 64. No alignment beyond byte alignment is required."
                    }
                    "__IncompleteArrayField" => {
                        "The trailing allocation must contain len aligned, initialized T elements, with a total size no greater than isize::MAX. They must remain valid for the returned slice's lifetime, without conflicting access (and with exclusive access for a mutable slice)."
                    }
                    "__BindgenUnionField" => {
                        "The backing union storage must contain a valid, initialized T, aligned and live for the returned reference's lifetime. Mutable access requires exclusivity; shared access forbids mutation for that lifetime."
                    }
                    _ if bitfield => {
                        "The pointer must address allocated record storage with bitfield bytes valid and initialized for reads, and exclusively accessible for writes. It must remain valid throughout the operation. Byte access does not require the record pointer to be aligned."
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &method.sig,
                            format!("unreviewed unsafe bindgen helper {owner}::{name}"),
                        ));
                    }
                };
                method.attrs.push(syn::parse_quote!(#[doc = "# Safety"]));
                method.attrs.push(syn::parse_quote!(#[doc = #contract]));
            }
            let mut variables = BTreeMap::new();
            for input in &method.sig.inputs {
                if let FnArg::Typed(argument) = input
                    && let Pat::Ident(pattern) = &*argument.pat
                {
                    variables.insert(pattern.ident.to_string(), *argument.ty.clone());
                }
            }
            let expected = match &method.sig.output {
                ReturnType::Type(_, ty) => Some(*ty.clone()),
                ReturnType::Default => None,
            };
            let mut policy = HelperPolicy {
                aliases: &aliases,
                variables,
                expected,
                bitfield,
                safe_bitfield: bitfield && method.sig.unsafety.is_none(),
                byte_offset: owner == "__BindgenBitfieldUnit",
                safety,
                error: None,
            };
            policy.visit_block_mut(&mut method.block);
            if let Some(error) = policy.error {
                return Err(syn::Error::new_spanned(
                    &method.sig,
                    format!("{owner}::{name}: {error}"),
                ));
            }
        }
    }
    // syn retains attributes, not comments. These private expression markers
    // become ordinary safety comments only after prettyplease has formatted them.
    Ok(prettyplease::unparse(&file).replace("/// __IREE_BINDGEN_SAFETY__:", "// SAFETY:"))
}

fn type_name(ty: &Type) -> Option<String> {
    let Type::Path(ty) = ty else { return None };
    ty.path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

struct HelperPolicy<'a> {
    aliases: &'a BTreeMap<String, Type>,
    variables: BTreeMap<String, Type>,
    expected: Option<Type>,
    bitfield: bool,
    safe_bitfield: bool,
    byte_offset: bool,
    safety: Option<&'static str>,
    error: Option<syn::Error>,
}

impl HelperPolicy<'_> {
    fn integer(&self, ty: &Type) -> Option<String> {
        let mut name = type_name(ty)?;
        for _ in 0..self.aliases.len() + 1 {
            if matches!(
                name.as_str(),
                "u8" | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "c_char"
                    | "c_schar"
                    | "c_uchar"
                    | "c_short"
                    | "c_ushort"
                    | "c_int"
                    | "c_uint"
                    | "c_long"
                    | "c_ulong"
                    | "c_longlong"
                    | "c_ulonglong"
            ) {
                return Some(name);
            }
            name = type_name(self.aliases.get(&name)?)?;
        }
        None
    }

    fn expression_integer(&self, expression: &Expr) -> Option<String> {
        match expression {
            Expr::Cast(cast) => self.integer(&cast.ty),
            Expr::Path(path) => {
                self.integer(self.variables.get(&path.path.get_ident()?.to_string())?)
            }
            Expr::Paren(paren) => self.expression_integer(&paren.expr),
            Expr::MethodCall(call) if call.method == "get" => Some("u64".into()),
            Expr::Call(call) => {
                let Expr::Path(path) = &*call.func else {
                    return None;
                };
                (path.path.segments.last()?.ident == "raw_get").then(|| "u64".into())
            }
            _ => None,
        }
    }
}

impl VisitMut for HelperPolicy<'_> {
    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        let previous = self.expected.clone();
        if let Pat::Type(pattern) = &local.pat {
            self.expected = Some(*pattern.ty.clone());
        } else {
            self.expected = None;
        }
        visit_mut::visit_local_mut(self, local);
        if let Pat::Type(pattern) = &local.pat
            && let Pat::Ident(name) = &*pattern.pat
        {
            self.variables
                .insert(name.ident.to_string(), *pattern.ty.clone());
        }
        self.expected = previous;
    }

    fn visit_expr_mut(&mut self, expression: &mut Expr) {
        visit_mut::visit_expr_mut(self, expression);
        if self.bitfield {
            if let Expr::Call(call) = expression
                && let Expr::Path(path) = &*call.func
                && path
                    .path
                    .segments
                    .last()
                    .is_some_and(|part| part.ident == "transmute")
                && call.args.len() == 1
                && let Some(target) = &self.expected
                && self.integer(target).is_some()
                && let Some(argument) = call.args.first()
                && self.expression_integer(argument).is_some()
            {
                *expression = Expr::Cast(syn::ExprCast {
                    attrs: Vec::new(),
                    expr: Box::new(argument.clone()),
                    as_token: Default::default(),
                    ty: Box::new(target.clone()),
                });
            }
            if let Expr::Cast(cast) = expression
                && self.integer(&cast.ty).is_some()
                && self.integer(&cast.ty) == self.expression_integer(&cast.expr)
            {
                *expression = *cast.expr.clone();
            }
        }
        if self.byte_offset
            && let Expr::MethodCall(call) = expression
            && call.method == "offset"
            && let Some(Expr::Cast(cast)) = call.args.first()
            && let Expr::Path(path) = &*cast.expr
            && path.path.is_ident("byte_index")
        {
            call.method = syn::parse_quote!(add);
            call.args = [*cast.expr.clone()].into_iter().collect();
        }
        if let Expr::Unsafe(block) = expression {
            if self.safe_bitfield {
                // Only integer conversions were unsafe in these safe accessors.
                // Reject an unexpected residual unsafe operation below.
                let mut probe = UnsafeOperation(false);
                probe.visit_block_mut(&mut block.block);
                if !probe.0 {
                    *expression =
                        if let [syn::Stmt::Expr(expression, None)] = &block.block.stmts[..] {
                            expression.clone()
                        } else {
                            Expr::Block(syn::ExprBlock {
                                attrs: block.attrs.clone(),
                                label: None,
                                block: block.block.clone(),
                            })
                        };
                    return;
                }
            }
            if let Some(safety) = self.safety {
                let comment = format!(" __IREE_BINDGEN_SAFETY__: {safety}");
                block.attrs.push(syn::parse_quote!(#[doc = #comment]));
            } else {
                self.error = Some(syn::Error::new_spanned(
                    block,
                    "unreviewed unsafe bindgen block",
                ));
            }
        }
    }
}

struct UnsafeOperation(bool);

impl VisitMut for UnsafeOperation {
    fn visit_expr_mut(&mut self, expression: &mut Expr) {
        match expression {
            Expr::Call(_)
            | Expr::Unary(syn::ExprUnary {
                op: syn::UnOp::Deref(_),
                ..
            }) => {
                self.0 = true;
            }
            _ => visit_mut::visit_expr_mut(self, expression),
        }
    }
}
