use super::{ObjectKind, ObjectType};

pub struct Boolean(bool);

impl ObjectType for Boolean {
    fn kind(&self) -> ObjectKind {
        return ObjectKind::BOOLEAN;
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
