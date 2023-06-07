mod foreign_crate {
    #[derive(PartialEq, Clone)]
    pub enum StrValue {
        Str(String),
    }
    pub enum Old {
        Num(f64),
        Bool(bool),
        None,
        Str(StrValue),
    }
}
pub type Old = foreign_crate::Old;

#[derive(PartialEq)]
pub enum New {
    Num(f64),
    Bool(bool),
    None,
    Str(foreign_crate::StrValue),
}

impl From<&Old> for New {
    fn from(value: &Old) -> Self {
        match value {
            Old::Num(a) => New::Num(*a),
            Old::Bool(a) => New::Bool(*a),
            Old::None => New::None,
            Old::Str(a) => New::Str(a.to_owned()),
        }
    }
}

impl<'a> From<&'a &New> for &'a New {
    fn from(value: &'a &New) -> Self {
        value
    }
}

impl PartialEq<Old> for New {
    fn eq(&self, other: &Old) -> bool {
        let other: New = other.into();
        self == &other
    }
}
impl PartialEq<New> for Old {
    fn eq(&self, other: &New) -> bool {
        other == self
    }
}
impl Eq for New {}
