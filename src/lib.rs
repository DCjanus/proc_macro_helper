#[macro_use]
extern crate quote;
extern crate syn;

mod attribute;
mod field;
mod struct_;

pub mod prelude {
    pub use attribute::Attribute;
    pub use attribute::AttributeType;
    pub use field::Field;
    pub use struct_::Struct;
}
