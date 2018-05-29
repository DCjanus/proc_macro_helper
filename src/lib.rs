#[deny(warnings)]
#[macro_use]
extern crate quote;
extern crate syn;

mod attribute;
mod enum_;
mod field;
mod struct_;
mod variant;

pub mod prelude {
    pub use attribute::Attribute;
    pub use enum_::Enum;
    pub use field::Field;
    pub use struct_::Struct;
    pub use variant::Variant;
}
