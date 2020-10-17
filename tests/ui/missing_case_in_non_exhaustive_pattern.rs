// edition:2018
#![warn(clippy::missing_case_in_non_exhaustive_pattern)]

#[non_exhaustive]
pub enum E {
    First,
    Second,
    Third,
}

fn main() {
    let e = E::First;
    match e {
        E::First => {},
        E::Second => {},
        _ => {},
    }
}
