use crate::utils::span_lint_and_help;
use rustc_ast::ast::*;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub MISSING_CASE_IN_NON_EXHAUSTIVE_PATTERN,
    style,
    "default lint description"
}

declare_lint_pass!(MissingCaseInNonExhaustivePattern => [MISSING_CASE_IN_NON_EXHAUSTIVE_PATTERN]);

impl EarlyLintPass for MissingCaseInNonExhaustivePattern {
    fn check_pat(&mut self, cx: &EarlyContext<'_>, pat: &Pat) {
        span_lint_and_help(
            cx,
            MISSING_CASE_IN_NON_EXHAUSTIVE_PATTERN,
            pat.span,
            "function named `foo`",
            None,
            "consider using a more meaningful name",
        );
    }
}
