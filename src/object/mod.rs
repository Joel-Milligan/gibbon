pub mod boolean;
pub mod integer;
pub mod null;

pub enum ObjectKind {
    INTEGER,
    BOOLEAN,
    NULL,
}

pub trait ObjectType {
    fn kind(&self) -> ObjectKind;
    fn inspect(&self) -> String;
}
