use diesel::Queryable;

#[derive(Queryable)]
pub struct Question {
    pub sort_order: i32,
    pub question: String,
}
