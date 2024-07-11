#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct One {
    value: bool,
}

#[uniffi::export]
pub fn new_one_default() -> One {
    One::default()
}

#[uniffi::export]
pub fn new_one(value: bool) -> One {
    One { value }
}
