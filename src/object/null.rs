use super::{ObjectKind, ObjectType};

pub struct Null;

impl ObjectType for Null {
    fn kind(&self) -> ObjectKind {
        return ObjectKind::NULL;
    }

    fn inspect(&self) -> String {
        format!("null")
    }
}
