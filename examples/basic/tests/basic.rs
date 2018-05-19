#[macro_use]
extern crate basic;

#[derive(Table1, Table2)]
pub struct User {
    #[PrimaryKey]
    pub pk: i64,
    pub id: String,
    pub name: String,
    pub headline: String,
    pub avatar_url: String,
    pub gender: i32,
    pub is_org: bool,
    #[PrimaryKey]
    pub url_token: String,
    pub user_type: String,
}

#[test]
fn basic_test() {
    debug_assert_eq!(PRIMARY_FIELDS1, PRIMARY_FIELDS2);
}
