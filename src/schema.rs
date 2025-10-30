// @generated automatically by Diesel CLI.

diesel::table! {
    answered_questions (id) {
        id -> Integer,
        category -> Integer,
        question -> Integer,
    }
}

diesel::table! {
    scores (house) {
        house -> Integer,
        score -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(answered_questions, scores,);
