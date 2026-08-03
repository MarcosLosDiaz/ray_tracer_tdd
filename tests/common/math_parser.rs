//! Math expression parsing helpers for cucumber step definitions.

#![allow(dead_code)]

pub const PI: f64 = std::f64::consts::PI;

/// Parses square root expressions (e.g. `√3`, `√2`, `√14`).
pub fn parse_sqrt(val: &str) -> f64 {
    let s = val.trim();
    let is_negative = s.starts_with('-');
    let raw = if is_negative { &s[1..] } else { s };
    let rad_str = raw.strip_prefix('√').unwrap_or(raw).trim();
    let rad: f64 = rad_str.parse().expect("Invalid radicand in sqrt expression");
    let result = rad.sqrt();
    if is_negative { -result } else { result }
}

/// Parses numerator term (`π`, `-π`, `√3`, `-√2`, `-4.2`, etc.)
fn parse_term(term: &str) -> f64 {
    let t = term.trim();
    if t == "π" {
        PI
    } else if t == "-π" {
        -PI
    } else if t.contains('√') {
        parse_sqrt(t)
    } else {
        t.parse::<f64>().unwrap_or_else(|_| panic!("Failed to parse float term: '{}'", t))
    }
}

/// Helper function to parse fractions, delegating numerator/denominator parsing.
pub fn parse_fraction(val: &str) -> f64 {
    let s = val.trim();
    let (num_str, denom_str) = s.split_once('/').expect("parse_fraction expected a string containing '/'");
    let num = parse_term(num_str);
    let denom = parse_term(denom_str);
    num / denom
}

/// Helper entrypoint alias for step definitions
pub fn parse_expr(val: &str) -> f64 {
    let s = val.trim();
    if s.contains('/') {
        parse_fraction(s)
    } else {
        parse_term(s)
    }
}
