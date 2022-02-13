use diesel::table;

table! {
    questions (sort_order) {
        sort_order -> Integer,
        question -> Text,
    }
}
