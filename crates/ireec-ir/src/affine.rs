//! Laws: affine values are immutable, context-owned handles. Composition with
//! identity preserves a map, and every expression references only declared inputs.
use crate::{
    Borrowed, Context, Error, Result,
    callback::print_with,
    types::{count, same_context},
};
use iree_native as n;
use std::{fmt, marker::PhantomData};
macro_rules! handle {
    ($name:ident, $raw:ident, $get_context:ident, $print:ident, $equal:ident) => {
        #[doc = concat!("A context-owned MLIR ", stringify!($name), ".")]
        #[derive(Clone, Copy)]
        pub struct $name<'c> {
            pub(crate) raw: n::$raw,
            _context: PhantomData<&'c ()>,
        }
        impl<'c> $name<'c> {
            pub(crate) fn checked(raw: n::$raw) -> Result<Self> {
                if raw.ptr.is_null() {
                    return Err(Error::Null(stringify!($name)));
                }
                Ok(Self {
                    raw,
                    _context: PhantomData,
                })
            }
            /// Borrow a native handle in its owning context.
            ///
            /// # Safety
            /// `raw` must belong to `context`.
            pub unsafe fn from_raw<O>(_context: &'c Context<'_, O>, raw: n::$raw) -> Result<Self> {
                Self::checked(raw)
            }
            /// Return the native handle.
            pub fn as_raw(&self) -> n::$raw {
                self.raw
            }
            /// Borrow the owning context.
            pub fn context(&self) -> Result<Context<'c, Borrowed>> {
                // SAFETY: the context handle comes from the live receiver and is borrowed only for the receiver-bound lifetime.
                unsafe { Context::borrow_raw(self.context_raw()) }
            }
            pub(crate) fn context_raw(&self) -> n::MlirContext {
                // SAFETY: this live affine handle owns the context returned by the generated native getter.
                unsafe { n::$get_context(self.raw) }
            }
            /// Print canonical assembly.
            pub(crate) fn print(&self) -> String {
                // SAFETY: the receiver keeps its affine handle and context alive until the synchronous printer returns.
                print_with(|callback, data| unsafe { n::$print(self.raw, callback, data) })
            }
        }
        impl PartialEq for $name<'_> {
            fn eq(&self, rhs: &Self) -> bool {
                // SAFETY: both affine wrappers keep their handles and context alive during the native equality check.
                unsafe { n::$equal(self.raw, rhs.raw) }
            }
        }
        impl Eq for $name<'_> {}
        impl fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.print())
            }
        }
        impl fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
    };
}
handle!(
    AffineExpr,
    MlirAffineExpr,
    mlirAffineExprGetContext,
    mlirAffineExprPrint,
    mlirAffineExprEqual
);
handle!(
    AffineMap,
    MlirAffineMap,
    mlirAffineMapGetContext,
    mlirAffineMapPrint,
    mlirAffineMapEqual
);
handle!(
    IntegerSet,
    MlirIntegerSet,
    mlirIntegerSetGetContext,
    mlirIntegerSetPrint,
    mlirIntegerSetEqual
);
/// The algebraic kind of an affine expression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AffineExprKind {
    /// An input dimension.
    Dim,
    /// A symbolic input.
    Symbol,
    /// A signed integer constant.
    Constant,
    /// Addition.
    Add,
    /// Multiplication.
    Mul,
    /// Modulo.
    Mod,
    /// Floor division.
    FloorDiv,
    /// Ceiling division.
    CeilDiv,
}
impl<'c> AffineExpr<'c> {
    /// Construct an affine dim expression.
    pub fn dim<O>(context: &'c Context<'_, O>, position: usize) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineDimExprGet`.
        Self::checked(unsafe { n::mlirAffineDimExprGet(context.raw, count(position)?) })
    }
    /// Construct an affine symbol expression.
    pub fn symbol<O>(context: &'c Context<'_, O>, position: usize) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineSymbolExprGet`.
        Self::checked(unsafe { n::mlirAffineSymbolExprGet(context.raw, count(position)?) })
    }
    /// Construct an affine constant expression.
    pub fn constant<O>(context: &'c Context<'_, O>, value: i64) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineConstantExprGet`.
        Self::checked(unsafe { n::mlirAffineConstantExprGet(context.raw, value) })
    }
    /// Algebraic expression kind.
    pub fn kind(&self) -> AffineExprKind {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsADim`.
        if unsafe { n::mlirAffineExprIsADim(self.raw) } {
            return AffineExprKind::Dim;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsASymbol`.
        if unsafe { n::mlirAffineExprIsASymbol(self.raw) } {
            return AffineExprKind::Symbol;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsAConstant`.
        if unsafe { n::mlirAffineExprIsAConstant(self.raw) } {
            return AffineExprKind::Constant;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsAAdd`.
        if unsafe { n::mlirAffineExprIsAAdd(self.raw) } {
            return AffineExprKind::Add;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsAMul`.
        if unsafe { n::mlirAffineExprIsAMul(self.raw) } {
            return AffineExprKind::Mul;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsAMod`.
        if unsafe { n::mlirAffineExprIsAMod(self.raw) } {
            return AffineExprKind::Mod;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsAFloorDiv`.
        if unsafe { n::mlirAffineExprIsAFloorDiv(self.raw) } {
            return AffineExprKind::FloorDiv;
        }
        AffineExprKind::CeilDiv
    }
    /// Dimension position, absent for other expression kinds.
    pub fn dimension_position(&self) -> Option<usize> {
        (self.kind() == AffineExprKind::Dim)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineDimExprGetPosition`.
            .then(|| (unsafe { n::mlirAffineDimExprGetPosition(self.raw) }) as usize)
    }
    /// Symbol position, absent for other expression kinds.
    pub fn symbol_position(&self) -> Option<usize> {
        (self.kind() == AffineExprKind::Symbol)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineSymbolExprGetPosition`.
            .then(|| (unsafe { n::mlirAffineSymbolExprGetPosition(self.raw) }) as usize)
    }
    /// Constant value, absent for other expression kinds.
    pub fn constant_value(&self) -> Option<i64> {
        (self.kind() == AffineExprKind::Constant)
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineConstantExprGetValue`.
            .then(|| unsafe { n::mlirAffineConstantExprGetValue(self.raw) })
    }
    /// Left and right operands for binary expressions.
    pub fn operands(&self) -> Option<(Self, Self)> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsABinary`.
        if !unsafe { n::mlirAffineExprIsABinary(self.raw) } {
            return None;
        }
        Some((
            Self {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineBinaryOpExprGetLHS`.
                raw: unsafe { n::mlirAffineBinaryOpExprGetLHS(self.raw) },
                ..*self
            },
            Self {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineBinaryOpExprGetRHS`.
                raw: unsafe { n::mlirAffineBinaryOpExprGetRHS(self.raw) },
                ..*self
            },
        ))
    }
    /// Simplify an expression within its declared input space.
    pub fn simplify(&self, dimensions: u32, symbols: u32) -> Result<Self> {
        self.validate_inputs(dimensions as usize, symbols as usize)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirSimplifyAffineExpr`.
        Self::checked(unsafe { n::mlirSimplifyAffineExpr(self.raw, dimensions, symbols) })
    }
    /// Whether this expression is independent of dimensions.
    pub fn is_symbolic_or_constant(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsSymbolicOrConstant`.
        unsafe { n::mlirAffineExprIsSymbolicOrConstant(self.raw) }
    }
    /// Whether all multiplication/division/modulo obey pure affine restrictions.
    pub fn is_pure_affine(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsPureAffine`.
        unsafe { n::mlirAffineExprIsPureAffine(self.raw) }
    }
    /// Largest statically known divisor.
    pub fn largest_known_divisor(&self) -> i64 {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprGetLargestKnownDivisor`.
        unsafe { n::mlirAffineExprGetLargestKnownDivisor(self.raw) }
    }
    /// Whether the expression is known to be a multiple of a positive factor.
    pub fn is_multiple_of(&self, factor: i64) -> Result<bool> {
        if factor <= 0 {
            return Err(Error::Invalid("divisor must be positive".into()));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsMultipleOf`.
        Ok(unsafe { n::mlirAffineExprIsMultipleOf(self.raw, factor) })
    }
    /// Whether the given dimension occurs in the expression.
    pub fn is_function_of_dimension(&self, position: usize) -> Result<bool> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprIsFunctionOfDim`.
        Ok(unsafe { n::mlirAffineExprIsFunctionOfDim(self.raw, count(position)?) })
    }
    fn validate_inputs(&self, dims: usize, symbols: usize) -> Result<()> {
        if self.dimension_position().is_some_and(|p| p >= dims)
            || self.symbol_position().is_some_and(|p| p >= symbols)
        {
            return Err(Error::Invalid(
                "affine expression references an undeclared input".into(),
            ));
        }
        if let Some((lhs, rhs)) = self.operands() {
            lhs.validate_inputs(dims, symbols)?;
            rhs.validate_inputs(dims, symbols)?;
        }
        Ok(())
    }
    /// Substitute the map's results for this expression's dimensions.
    /// Expression symbols and map symbols share their input positions.
    pub fn compose(&self, map: AffineMap<'c>) -> Result<Self> {
        same_context(self.context_raw(), map.context_raw())?;
        self.validate_inputs(map.num_results(), usize::MAX)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprCompose`.
        Self::checked(unsafe { n::mlirAffineExprCompose(self.raw, map.raw) })
    }
    /// Construct and simplify affine add in the same context.
    pub fn add(&self, rhs: Self) -> Result<Self> {
        same_context(self.context_raw(), rhs.context_raw())?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineAddExprGet`.
        Self::checked(unsafe { n::mlirAffineAddExprGet(self.raw, rhs.raw) })
    }
    /// Construct and simplify affine multiply in the same context.
    pub fn multiply(&self, rhs: Self) -> Result<Self> {
        same_context(self.context_raw(), rhs.context_raw())?;
        if !self.is_symbolic_or_constant() && !rhs.is_symbolic_or_constant() {
            return Err(Error::Invalid(
                "affine multiplication needs a symbolic or constant operand".into(),
            ));
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMulExprGet`.
        Self::checked(unsafe { n::mlirAffineMulExprGet(self.raw, rhs.raw) })
    }
    /// Construct and simplify affine modulo in the same context.
    pub fn modulo(&self, rhs: Self) -> Result<Self> {
        self.check_divisor(rhs)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineModExprGet`.
        Self::checked(unsafe { n::mlirAffineModExprGet(self.raw, rhs.raw) })
    }
    /// Construct and simplify affine floor divide in the same context.
    pub fn floor_divide(&self, rhs: Self) -> Result<Self> {
        self.check_divisor(rhs)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineFloorDivExprGet`.
        Self::checked(unsafe { n::mlirAffineFloorDivExprGet(self.raw, rhs.raw) })
    }
    /// Construct and simplify affine ceil divide in the same context.
    pub fn ceil_divide(&self, rhs: Self) -> Result<Self> {
        self.check_divisor(rhs)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineCeilDivExprGet`.
        Self::checked(unsafe { n::mlirAffineCeilDivExprGet(self.raw, rhs.raw) })
    }
    fn check_divisor(&self, rhs: Self) -> Result<()> {
        same_context(self.context_raw(), rhs.context_raw())?;
        if !rhs.is_symbolic_or_constant() || rhs.constant_value().is_some_and(|value| value <= 0) {
            return Err(Error::Invalid(
                "affine divisor must be symbolic or a positive constant".into(),
            ));
        }
        Ok(())
    }
    /// Shift input positions at or above `offset` by `shift`.
    pub fn shift_dimensions(&self, num_inputs: u32, shift: u32, offset: u32) -> Result<Self> {
        if offset > num_inputs || num_inputs.checked_add(shift).is_none() {
            return Err(Error::Invalid("invalid affine input shift".into()));
        }
        self.validate_inputs(num_inputs as usize, usize::MAX)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprShiftDims`.
        Self::checked(unsafe { n::mlirAffineExprShiftDims(self.raw, num_inputs, shift, offset) })
    }
    /// Shift input positions at or above `offset` by `shift`.
    pub fn shift_symbols(&self, num_inputs: u32, shift: u32, offset: u32) -> Result<Self> {
        if offset > num_inputs || num_inputs.checked_add(shift).is_none() {
            return Err(Error::Invalid("invalid affine input shift".into()));
        }
        self.validate_inputs(usize::MAX, num_inputs as usize)?;
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineExprShiftSymbols`.
        Self::checked(unsafe { n::mlirAffineExprShiftSymbols(self.raw, num_inputs, shift, offset) })
    }
}
impl<'c> AffineMap<'c> {
    /// Construct a map over declared dimensions and symbols.
    pub fn new<O>(
        context: &'c Context<'_, O>,
        dims: usize,
        symbols: usize,
        results: &[AffineExpr<'c>],
    ) -> Result<Self> {
        for expr in results {
            same_context(context.raw, expr.context_raw())?;
            expr.validate_inputs(dims, symbols)?;
        }
        let mut raw: Vec<_> = results.iter().map(|expr| expr.raw).collect();
        let dims = count(dims)?;
        let symbols = count(symbols)?;
        // SAFETY: `raw` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        let value = unsafe {
            if raw.is_empty() {
                n::mlirAffineMapZeroResultGet(context.raw, dims, symbols)
            } else {
                n::mlirAffineMapGet(
                    context.raw,
                    dims,
                    symbols,
                    count(raw.len())?,
                    raw.as_mut_ptr(),
                )
            }
        };
        Self::checked(value)
    }
    /// Construct an empty map with no inputs or results.
    pub fn empty<O>(context: &'c Context<'_, O>) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineMapEmptyGet`.
        Self::checked(unsafe { n::mlirAffineMapEmptyGet(context.raw) })
    }
    /// Construct a single constant result.
    pub fn constant<O>(context: &'c Context<'_, O>, value: i64) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineMapConstantGet`.
        Self::checked(unsafe { n::mlirAffineMapConstantGet(context.raw, value) })
    }
    /// Construct a multi-dimensional identity map.
    pub fn identity<O>(context: &'c Context<'_, O>, dims: usize) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineMapMultiDimIdentityGet`.
        Self::checked(unsafe { n::mlirAffineMapMultiDimIdentityGet(context.raw, count(dims)?) })
    }
    /// Project the trailing dimensions of an identity map.
    pub fn minor_identity<O>(
        context: &'c Context<'_, O>,
        dims: usize,
        results: usize,
    ) -> Result<Self> {
        if results > dims {
            return Err(Error::Invalid("minor identity has too many results".into()));
        }
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirAffineMapMinorIdentityGet`.
        Self::checked(unsafe {
            n::mlirAffineMapMinorIdentityGet(context.raw, count(dims)?, count(results)?)
        })
    }
    /// Construct a permutation, rejecting duplicate or out-of-range positions.
    pub fn permutation<O>(context: &'c Context<'_, O>, permutation: &[u32]) -> Result<Self> {
        let mut seen = vec![false; permutation.len()];
        for &position in permutation {
            let flag = seen
                .get_mut(position as usize)
                .ok_or_else(|| Error::Invalid("permutation position out of range".into()))?;
            if *flag {
                return Err(Error::Invalid("duplicate permutation position".into()));
            }
            *flag = true;
        }
        let mut positions = permutation.to_vec();
        // SAFETY: `positions` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        Self::checked(unsafe {
            n::mlirAffineMapPermutationGet(
                context.raw,
                count(positions.len())?,
                positions.as_mut_ptr(),
            )
        })
    }
    /// Query num dimensions.
    pub fn num_dimensions(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetNumDims`.
        (unsafe { n::mlirAffineMapGetNumDims(self.raw) }) as usize
    }
    /// Query num symbols.
    pub fn num_symbols(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetNumSymbols`.
        (unsafe { n::mlirAffineMapGetNumSymbols(self.raw) }) as usize
    }
    /// Query num inputs.
    pub fn num_inputs(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetNumInputs`.
        (unsafe { n::mlirAffineMapGetNumInputs(self.raw) }) as usize
    }
    /// Query num results.
    pub fn num_results(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetNumResults`.
        (unsafe { n::mlirAffineMapGetNumResults(self.raw) }) as usize
    }
    /// Query is identity.
    pub fn is_identity(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsIdentity`.
        unsafe { n::mlirAffineMapIsIdentity(self.raw) }
    }
    /// Query is minor identity.
    pub fn is_minor_identity(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsMinorIdentity`.
        unsafe { n::mlirAffineMapIsMinorIdentity(self.raw) }
    }
    /// Query is permutation.
    pub fn is_permutation(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsPermutation`.
        unsafe { n::mlirAffineMapIsPermutation(self.raw) }
    }
    /// Query is projected permutation.
    pub fn is_projected_permutation(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsProjectedPermutation`.
        unsafe { n::mlirAffineMapIsProjectedPermutation(self.raw) }
    }
    /// Query is empty.
    pub fn is_empty(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsEmpty`.
        unsafe { n::mlirAffineMapIsEmpty(self.raw) }
    }
    /// Results in order.
    pub fn results(&self) -> Vec<AffineExpr<'c>> {
        (0..self.num_results())
            .map(|i| AffineExpr {
                // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetResult`.
                raw: unsafe { n::mlirAffineMapGetResult(self.raw, i as isize) },
                _context: PhantomData,
            })
            .collect()
    }
    /// Constant result, only for a single-constant map.
    pub fn single_constant_result(&self) -> Option<i64> {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapIsSingleConstant`.
        unsafe { n::mlirAffineMapIsSingleConstant(self.raw) }
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetSingleConstantResult`.
            .then(|| unsafe { n::mlirAffineMapGetSingleConstantResult(self.raw) })
    }
    /// Compose this map after `inner`; symbol positions are shared, not concatenated.
    pub fn compose(&self, inner: Self) -> Result<Self> {
        same_context(self.context_raw(), inner.context_raw())?;
        if self.num_dimensions() != inner.num_results() {
            return Err(Error::Invalid(
                "affine composition dimension mismatch".into(),
            ));
        }
        let symbols = self.num_symbols().max(inner.num_symbols());
        let mut results = Vec::new();
        for expr in self.results() {
            // MLIR's expression composition substitutes dimensions and retains symbols.
            // SAFETY: `expr` and `inner` are live native handles; their wrappers keep them valid throughout `mlirAffineExprCompose`.
            let raw = unsafe { n::mlirAffineExprCompose(expr.raw, inner.raw) };
            results.push(AffineExpr::checked(raw)?);
        }
        let mut results: Vec<_> = results.iter().map(|expr| expr.raw).collect();
        // SAFETY: `results` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        let raw = unsafe {
            if results.is_empty() {
                n::mlirAffineMapZeroResultGet(
                    self.context_raw(),
                    count(inner.num_dimensions())?,
                    count(symbols)?,
                )
            } else {
                n::mlirAffineMapGet(
                    self.context_raw(),
                    count(inner.num_dimensions())?,
                    count(symbols)?,
                    count(results.len())?,
                    results.as_mut_ptr(),
                )
            }
        };
        Self::checked(raw)
    }
    /// Select results in the requested order; positions must be in bounds.
    pub fn submap(&self, positions: &[usize]) -> Result<Self> {
        if positions.iter().any(|&p| p >= self.num_results()) {
            return Err(Error::Invalid("submap result out of bounds".into()));
        }
        let mut positions: Vec<_> = positions.iter().map(|&p| p as isize).collect();
        // SAFETY: `positions` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        Self::checked(unsafe {
            n::mlirAffineMapGetSubMap(self.raw, count(positions.len())?, positions.as_mut_ptr())
        })
    }
    /// Select leading results; zero returns none and an oversized count returns this map.
    pub fn major_submap(&self, results: usize) -> Result<Option<Self>> {
        if results == 0 {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetMajorSubMap`.
        let raw = unsafe { n::mlirAffineMapGetMajorSubMap(self.raw, count(results)?) };
        Ok(Some(Self::checked(raw)?))
    }
    /// Select trailing results; zero returns none and an oversized count returns this map.
    pub fn minor_submap(&self, results: usize) -> Result<Option<Self>> {
        if results == 0 {
            return Ok(None);
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapGetMinorSubMap`.
        let raw = unsafe { n::mlirAffineMapGetMinorSubMap(self.raw, count(results)?) };
        Ok(Some(Self::checked(raw)?))
    }
    /// Replace an expression and explicitly declare the resulting input space.
    pub fn replace(
        &self,
        expression: AffineExpr<'c>,
        replacement: AffineExpr<'c>,
        dims: usize,
        symbols: usize,
    ) -> Result<Self> {
        same_context(self.context_raw(), expression.context_raw())?;
        same_context(self.context_raw(), replacement.context_raw())?;
        replacement.validate_inputs(dims, symbols)?;
        fn validate_replaced(
            expr: AffineExpr<'_>,
            needle: AffineExpr<'_>,
            replacement: AffineExpr<'_>,
            dims: usize,
            symbols: usize,
        ) -> Result<()> {
            if expr == needle {
                return replacement.validate_inputs(dims, symbols);
            }
            if let Some((lhs, rhs)) = expr.operands() {
                validate_replaced(lhs, needle, replacement, dims, symbols)?;
                validate_replaced(rhs, needle, replacement, dims, symbols)
            } else {
                expr.validate_inputs(dims, symbols)
            }
        }
        for expr in self.results() {
            validate_replaced(expr, expression, replacement, dims, symbols)?;
        }
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirAffineMapReplace`.
        let raw = unsafe {
            n::mlirAffineMapReplace(
                self.raw,
                expression.raw,
                replacement.raw,
                count(dims)?,
                count(symbols)?,
            )
        };
        let map = Self::checked(raw)?;
        for expr in map.results() {
            expr.validate_inputs(dims, symbols)?;
        }
        Ok(map)
    }
}
impl<'c> AffineMap<'c> {
    /// Jointly remove unused symbols, preserving consistent symbol positions.
    pub fn compress_unused_symbols(maps: &[Self]) -> Result<Vec<Self>> {
        let Some(first) = maps.first() else {
            return Ok(Vec::new());
        };
        for map in maps {
            same_context(first.context_raw(), map.context_raw())?;
            if first.num_symbols() != map.num_symbols() {
                return Err(Error::Invalid("maps must share a symbol space".into()));
            }
        }
        unsafe extern "C" fn collect(
            data: *mut std::ffi::c_void,
            index: isize,
            map: n::MlirAffineMap,
        ) {
            // SAFETY: `data` is registered callback state; its owner keeps it alive until callback removal or return.
            let output = unsafe { &mut *data.cast::<Vec<Option<n::MlirAffineMap>>>() };
            crate::callback::contain(|| {
                if let Some(slot) = usize::try_from(index).ok().and_then(|i| output.get_mut(i)) {
                    *slot = Some(map);
                }
            });
        }
        let mut raw: Vec<_> = maps.iter().map(|map| map.raw).collect();
        let mut output = vec![None; maps.len()];
        // SAFETY: `raw` remain allocated for the synchronous C call and are exclusively borrowed while C writes the results; the validated native count matches the supplied buffer(s).
        unsafe {
            n::mlirAffineMapCompressUnusedSymbols(
                raw.as_mut_ptr(),
                count(raw.len())?,
                std::ptr::from_mut(&mut output).cast(),
                Some(collect),
            )
        };
        output
            .into_iter()
            .map(|raw| Self::checked(raw.ok_or(Error::Null("compressed affine map"))?))
            .collect()
    }
}
impl<'c> IntegerSet<'c> {
    /// Construct the canonical empty set in the given input space.
    pub fn empty<O>(context: &'c Context<'_, O>, dims: usize, symbols: usize) -> Result<Self> {
        // SAFETY: the borrowed `context` keeps its native context alive throughout `mlirIntegerSetEmptyGet`.
        Self::checked(unsafe {
            n::mlirIntegerSetEmptyGet(context.raw, count(dims)?, count(symbols)?)
        })
    }
    /// Construct constraints; true flags mean equality, false means nonnegative.
    pub fn new<O>(
        context: &'c Context<'_, O>,
        dims: usize,
        symbols: usize,
        constraints: &[AffineExpr<'c>],
        equalities: &[bool],
    ) -> Result<Self> {
        if constraints.is_empty() || constraints.len() != equalities.len() {
            return Err(Error::Invalid(
                "integer set needs nonempty constraints and one flag per constraint".into(),
            ));
        }
        for expr in constraints {
            same_context(context.raw, expr.context_raw())?;
            expr.validate_inputs(dims, symbols)?;
        }
        let raw: Vec<_> = constraints.iter().map(|expr| expr.raw).collect();
        // SAFETY: `raw` and `equalities` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Self::checked(unsafe {
            n::mlirIntegerSetGet(
                context.raw,
                count(dims)?,
                count(symbols)?,
                count(raw.len())?,
                raw.as_ptr(),
                equalities.as_ptr(),
            )
        })
    }
    /// Query num dimensions.
    pub fn num_dimensions(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumDims`.
        (unsafe { n::mlirIntegerSetGetNumDims(self.raw) }) as usize
    }
    /// Query num symbols.
    pub fn num_symbols(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumSymbols`.
        (unsafe { n::mlirIntegerSetGetNumSymbols(self.raw) }) as usize
    }
    /// Query num inputs.
    pub fn num_inputs(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumInputs`.
        (unsafe { n::mlirIntegerSetGetNumInputs(self.raw) }) as usize
    }
    /// Query num constraints.
    pub fn num_constraints(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumConstraints`.
        (unsafe { n::mlirIntegerSetGetNumConstraints(self.raw) }) as usize
    }
    /// Query num equalities.
    pub fn num_equalities(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumEqualities`.
        (unsafe { n::mlirIntegerSetGetNumEqualities(self.raw) }) as usize
    }
    /// Query num inequalities.
    pub fn num_inequalities(&self) -> usize {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetNumInequalities`.
        (unsafe { n::mlirIntegerSetGetNumInequalities(self.raw) }) as usize
    }
    /// Query is canonical empty.
    pub fn is_canonical_empty(&self) -> bool {
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetIsCanonicalEmpty`.
        unsafe { n::mlirIntegerSetIsCanonicalEmpty(self.raw) }
    }
    /// Constraint expression and equality flag by position.
    pub fn constraint(&self, index: usize) -> Option<(AffineExpr<'c>, bool)> {
        if index >= self.num_constraints() {
            return None;
        }
        let expr = AffineExpr {
            // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetGetConstraint`.
            raw: unsafe { n::mlirIntegerSetGetConstraint(self.raw, index as isize) },
            _context: PhantomData,
        };
        // SAFETY: `self` retains this valid native handle and its context throughout `mlirIntegerSetIsConstraintEq`.
        Some((expr, unsafe {
            n::mlirIntegerSetIsConstraintEq(self.raw, index as isize)
        }))
    }
    /// Substitute every dimension and symbol into a new declared input space.
    pub fn replace(
        &self,
        dimensions: &[AffineExpr<'c>],
        symbols: &[AffineExpr<'c>],
        result_dims: usize,
        result_symbols: usize,
    ) -> Result<Self> {
        if dimensions.len() != self.num_dimensions() || symbols.len() != self.num_symbols() {
            return Err(Error::Invalid(
                "integer set replacement count mismatch".into(),
            ));
        }
        for expr in dimensions.iter().chain(symbols) {
            same_context(self.context_raw(), expr.context_raw())?;
            expr.validate_inputs(result_dims, result_symbols)?;
        }
        let dims: Vec<_> = dimensions.iter().map(|expr| expr.raw).collect();
        let syms: Vec<_> = symbols.iter().map(|expr| expr.raw).collect();
        // SAFETY: `dims` and `syms` remain allocated for the synchronous C call and are read only during the call; the validated native count matches the supplied buffer(s).
        Self::checked(unsafe {
            n::mlirIntegerSetReplaceGet(
                self.raw,
                dims.as_ptr(),
                syms.as_ptr(),
                count(result_dims)?,
                count(result_symbols)?,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn composing_identity_preserves_maps(dims in 1usize..8, value in -1000i64..1000) {
            let context = Context::new().unwrap();
            let constant = AffineExpr::constant(&context, value).unwrap();
            let dimension = AffineExpr::dim(&context, dims - 1).unwrap();
            let expression = dimension.add(constant).unwrap();
            let map = AffineMap::new(&context, dims, 0, &[expression]).unwrap();
            let identity = AffineMap::identity(&context, dims).unwrap();
            prop_assert_eq!(map.compose(identity).unwrap(), map);
            let output_identity = AffineMap::identity(&context, 1).unwrap();
            prop_assert_eq!(output_identity.compose(map).unwrap(), map);
            prop_assert_eq!(expression.compose(identity).unwrap(), expression);
        }
        #[test]
        fn constants_and_set_constraints_round_trip(value in any::<i64>()) {
            let context = Context::new().unwrap();
            let expression = AffineExpr::constant(&context, value).unwrap();
            prop_assert_eq!(expression.constant_value(), Some(value));
            let dimension = AffineExpr::dim(&context, 0).unwrap();
            let set = IntegerSet::new(&context, 1, 0, &[dimension], &[false]).unwrap();
            prop_assert_eq!(set.constraint(0), Some((dimension, false)));
            prop_assert_eq!(set.num_inequalities(), 1);
        }
    }
    #[test]
    fn invalid_inputs_never_reach_native_assertions() {
        let context = Context::new().unwrap();
        let dimension = AffineExpr::dim(&context, 1).unwrap();
        assert!(AffineMap::new(&context, 1, 0, &[dimension]).is_err());
        assert!(AffineMap::permutation(&context, &[0, 0]).is_err());
        let zero = AffineExpr::constant(&context, 0).unwrap();
        assert!(dimension.floor_divide(zero).is_err());
    }
}
