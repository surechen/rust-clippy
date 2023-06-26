mod falliable_memory_allocation;
mod mem_unsafe_functions;
mod passing_string_to_c_functions;
mod untrusted_lib_loading;

use clippy_utils::def_path_def_ids;
use rustc_hir as hir;
use rustc_hir::def_id::{DefId, DefIdSet};
use rustc_hir::intravisit;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_tool_lint, impl_lint_pass};
use rustc_span::def_id::LocalDefId;
use rustc_span::Span;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for direct usage of external functions that modify memory
    /// without concerning about memory safety, such as `memcpy`, `strcpy`, `strcat` etc.
    ///
    /// ### Why is this bad?
    /// These function can be dangerous when used incorrectly,
    /// which could potentially introduce vulnerablities such as buffer overflow to the software.
    ///
    /// ### Example
    /// ```rust
    /// extern "C" {
    ///     fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    /// }
    /// let ptr = unsafe { memcpy(dest, src, size); }
    /// // Or use via libc
    /// let ptr = unsafe { libc::memcpy(dest, src, size); }
    #[clippy::version = "1.70.0"]
    pub MEM_UNSAFE_FUNCTIONS,
    nursery,
    "use of potentially dangerous external functions"
}

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.70.0"]
    pub UNTRUSTED_LIB_LOADING,
    nursery,
    "attempt to load dynamic library from untrusted source"
}

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.70.0"]
    pub PASSING_STRING_TO_C_FUNCTIONS,
    nursery,
    "passing string or str to extern C function"
}

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.70.0"]
    pub FALLIABLE_MEMORY_ALLOCATION,
    nursery,
    "memory allocation without checking arguments and result"
}

#[derive(Clone, Default)]
pub struct GuidelineLints {
    mem_uns_fns: Vec<String>,
    mem_uns_fns_ty_ids: DefIdSet,
}

impl GuidelineLints {
    pub fn new(mem_uns_fns: Vec<String>) -> Self {
        Self {
            mem_uns_fns,
            mem_uns_fns_ty_ids: DefIdSet::new(),
        }
    }
}

impl_lint_pass!(GuidelineLints => [
    MEM_UNSAFE_FUNCTIONS,
    UNTRUSTED_LIB_LOADING,
    PASSING_STRING_TO_C_FUNCTIONS,
    FALLIABLE_MEMORY_ALLOCATION,
]);

impl<'tcx> LateLintPass<'tcx> for GuidelineLints {
    fn check_fn(
        &mut self,
        _cx: &LateContext<'tcx>,
        _kind: intravisit::FnKind<'tcx>,
        _decl: &'tcx hir::FnDecl<'_>,
        _body: &'tcx hir::Body<'_>,
        _span: Span,
        _def_id: LocalDefId,
    ) {
    }

    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        // Resolve function names to def_ids from configuration
        for uns_fns in &self.mem_uns_fns {
            // Path like function names such as `libc::foo` or `aa::bb::cc::bar`,
            // this only works with dependencies.
            if uns_fns.contains("::") {
                let path: Vec<&str> = uns_fns.split("::").collect();
                for did in def_path_def_ids(cx, path.as_slice()) {
                    self.mem_uns_fns_ty_ids.insert(did);
                }
            }
            // Plain function names, then we should take its libc variant into account
            else if let Some(did) = libc_fn_def_id(cx, uns_fns) {
                self.mem_uns_fns_ty_ids.insert(did);
            }
        }
    }

    fn check_item(&mut self, _cx: &LateContext<'tcx>, item: &'tcx hir::Item<'_>) {
        mem_unsafe_functions::check_foreign_item(item, &self.mem_uns_fns, &mut self.mem_uns_fns_ty_ids);
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'_>) {
        mem_unsafe_functions::check(cx, expr, &self.mem_uns_fns_ty_ids);
    }
}

fn libc_fn_def_id(cx: &LateContext<'_>, fn_name: &str) -> Option<DefId> {
    let path = &["libc", fn_name];
    def_path_def_ids(cx, path).next()
}