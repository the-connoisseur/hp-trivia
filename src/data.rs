pub const CATEGORIES: &[(&str, &[(&str, &str)])] = &[
    (
        "General Knowledge",
        &[
            ("What do you call someone who was born into a wizarding family but hasn't got any magical powers?", "A Squib"),
            ("What is the eavesdropping tool that Fred and George develop called?", "Extendable Ears"),
            ("Where do the Weasleys go on vacation when they win the lottery?", "Egypt"),
            ("What is the name of Gilderoy Lockhart's autobiography?", "Magical Me"),
            ("What are the exams that seventh years at Hogwarts have to take called? (the full form, not the abbreviation)", "Nastily Exhausting Wizarding Tests (N.E.W.T's)"),
            ("What department in the Ministry of Magic does Arthur Weasley work at?\nBONUS: What is his assistant's name?", "The Misuse of Muggle Artifacts Office\nPerkins"),
        ],
    ),
    (
        "Spells And Potions",
        &[
            ("Which spell of the Half-Blood Prince does Harry use on Draco in their sixth year?", "Sectumsempra"),
            ("Lacewing flies, powdered bicorn horn, and shredded boomslang skin are ingredients for what?", "Polyjuice Potion"),
            ("What spell turns off the light cast by Lumos?", "Nox"),
            ("Which book contains the recipe for Polyjuice Potion?", "Moste Potente Potions"),
            ("What spell closes and locks doors?", "Colloportus"),
            ("What potion smells of freshly mown grass, new parchment, and ______ to Hermione?\nBONUS: Fill in the blank", "Amortentia\nRon Weasley's hair"),
        ],
    ),
    (
        "Magical Creatures",
        &[
            ("Hippogriffs are a cross between which two animals?", "Horses and eagles"),
            ("What creature pulls the carriages at Hogwarts, that are only able to be seen by some?", "Thestral"),
            ("Question Three", "Answer Three"),
            ("What breed of dragon guards the high security vaults in Gringotts?", "Ukrainian Ironbelly"),
            ("What is Ron's pet owl's name? BONUS: Who names him?", "Pigwidgeon\nGinny"),
            ("What breeds are the four dragons in the Triwizard Tournament, and who gets which one?", "Swedish Short Snout - Cedric, Chinese Fireball - Krum, \nCommon Welsh Green - Fleur, Hungarian Horntail - Harry"),
        ],
    ),
    (
        "Quidditch",
        &[
            ("How many players are on a Quidditch team?", "Seven"),
            ("What is the ball that tries to knock players off their brooms?", "Bludger"),
            ("Who gifts Harry the Firebolt?", "Sirius Black"),
            ("What is Ron's favorite Quidditch team?", "Chudley Cannons"),
            ("Who succeeds Oliver Wood as the captain of the Gryffindor quidditch team?", "Angelina Johnson"),
            ("Which two countries play in the finals of the Quidditch World cup?\nBONUS: What are their mascots", "Ireland and Bulgaria\nLeprechauns and Veelas"),
        ],
    ),
    (
        "Witches, Wizards, etc.",
        &[
            ("Who said it?\n\"It is our choices, Harry, that show what we truly are, far more than our abilities\"", "Albus Dumbledore"),
            ("Who is Slytherin house's ghost?", "The Bloody Baron"),
            ("Who does Hermione attempt to turn into when she takes Polyjuice Potion in the second year?", "Millicent Bulstrode"),
            ("Who said it?\n\"Oh Potter, you rotter, oh, what have you done,\nYou're killing off students, you think it's good fun\"", "Peeves"),
            ("Who was the headmaster of Hogwarts before Albus Dumbledore?", "Armando Dippet"),
            ("Which Hogwarts professor is murdered by Voldemort at the start of the seventh year?\nBONUS: What subject did they teach?", "Charity Burbage\nMuggle Studies"),
        ],
    ),
];

pub fn get_question_and_answer(category: usize, row: usize) -> (&'static str, &'static str) {
    CATEGORIES[category].1[row]
}
