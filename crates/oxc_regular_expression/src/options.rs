#[derive(Clone, Copy, Debug, Default)]
pub struct ParserOptions {
    /// Used to adjust Span positions to fit the global source code.
    pub span_offset: u32,
    /// Unicode mode(`u` or `v` flag) enabled or not.
    pub unicode_mode: bool,
    /// Extended Unicode mode(`v` flag) enabled or not.
    pub unicode_sets_mode: bool,
    // TODO: Add `handle_escape_with_quote_type` like option to support `new RegExp("with \"escape\"")`
}
