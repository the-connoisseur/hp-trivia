pub const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "General Knowledge",
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
        "Spells And Potions",
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
        "Magical Creatures",
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
        "Quidditch",
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
        "Who Said It?",
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
