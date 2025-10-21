pub const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "Category One",
        &[
            ("Question One", "Answer One"),
            ("Question Two", "Answer Two"),
            ("Question Three", "Answer Three"),
            ("Question Four", "Answer Four"),
            ("Question Five", "Answer Five"),
            ("Question Six", "Answer Six"),
        ],
    ),
    (
        "Category Two",
        &[
            ("Question One", "Answer One"),
            ("Question Two", "Answer Two"),
            ("Question Three", "Answer Three"),
            ("Question Four", "Answer Four"),
            ("Question Five", "Answer Five"),
            ("Question Six", "Answer Six"),
        ],
    ),
    (
        "Category Three",
        &[
            ("Question One", "Answer One"),
            ("Question Two", "Answer Two"),
            ("Question Three", "Answer Three"),
            ("Question Four", "Answer Four"),
            ("Question Five", "Answer Five"),
            ("Question Six", "Answer Six"),
        ],
    ),
    (
        "Category Four",
        &[
            ("Question One", "Answer One"),
            ("Question Two", "Answer Two"),
            ("Question Three", "Answer Three"),
            ("Question Four", "Answer Four"),
            ("Question Five", "Answer Five"),
            ("Question Six", "Answer Six"),
        ],
    ),
    (
        "Category Five",
        &[
            ("Question One", "Answer One"),
            ("Question Two", "Answer Two"),
            ("Question Three", "Answer Three"),
            ("Question Four", "Answer Four"),
            ("Question Five", "Answer Five"),
            ("Question Six", "Answer Six"),
        ],
    ),
];

pub fn get_question_and_answer(category: usize, row: usize) -> (&'static str, &'static str) {
    CATEGORIES[category].1[row]
}
