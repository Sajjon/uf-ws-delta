use one::One;
use std::sync::Arc;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Two(bool);

uniffi::custom_newtype!(Two, bool);

#[uniffi::export]
pub fn new_two_default() -> Two {
    Two::default()
}

#[uniffi::export]
pub fn new_two(value: bool) -> Two {
    Two(value)
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Record)]
pub struct AlphaRecord {
    one: One,
    two: Two,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, uniffi::Object)]
#[uniffi::export(Eq, Debug)]
pub struct AlphaObject {
    one: One,
    two: Two,
}

#[uniffi::export]
impl AlphaObject {
    #[uniffi::constructor]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[uniffi::constructor]
    pub fn new(one: One, two: Two) -> Self {
        Self { one, two }
    }
}

impl From<AlphaObject> for AlphaRecord {
    fn from(value: AlphaObject) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}
impl From<AlphaRecord> for AlphaObject {
    fn from(value: AlphaRecord) -> Self {
        Self {
            one: value.one,
            two: value.two,
        }
    }
}

#[uniffi::export]
pub fn new_record(one: One, two: Two) -> AlphaRecord {
    AlphaRecord { one, two }
}

#[uniffi::export]
pub fn new_record_default() -> AlphaRecord {
    AlphaRecord::default()
}

#[uniffi::export]
pub fn record_ref_record(value: &AlphaRecord) -> AlphaRecord {
    value.clone()
}

#[uniffi::export]
pub fn record_record(value: AlphaRecord) -> AlphaRecord {
    value
}

#[uniffi::export]
pub fn object_ref_object(value: &AlphaObject) -> AlphaObject {
    value.clone()
}

#[uniffi::export]
pub fn object_object(value: Arc<AlphaObject>) -> Arc<AlphaObject> {
    value
}

#[uniffi::export]
pub fn record_object(value: AlphaRecord) -> AlphaObject {
    value.into()
}

#[uniffi::export]
pub fn object_record(value: Arc<AlphaObject>) -> AlphaRecord {
    let raw = Arc::<AlphaObject>::into_raw(value);
    let inner = unsafe { raw.as_ref() }.unwrap().clone();
    AlphaRecord::from(inner)
}
