#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

pub trait HasSpan {
    fn span(&self) -> Span;
}

impl Span {
    pub fn new(lo: usize, hi: usize) -> Self {
        Self { lo, hi }
    }

    pub fn to(&self, span: Span) -> Self {
        Self {
            lo: self.lo.min(span.lo),
            hi: span.hi.max(span.hi),
        }
    }
}
