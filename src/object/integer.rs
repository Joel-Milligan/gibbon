use super::{ObjectKind, ObjectType};

pub struct Integer(i64);

impl ObjectType for Integer {
    fn kind(&self) -> ObjectKind {
        return ObjectKind::INTEGER;
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
