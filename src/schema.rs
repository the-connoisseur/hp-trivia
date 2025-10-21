// @generated automatically by Diesel CLI.

diesel::table! {
    answered_questions (id) {
        id -> Integer,
        category -> Integer,
        question -> Integer,
    }
}
