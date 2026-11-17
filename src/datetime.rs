//! Test helpers for `jiff::Timestamp`.

pub(crate) fn parse(value: &str) -> Result<jiff::Timestamp, jiff::Error> {
    value.parse()
}
