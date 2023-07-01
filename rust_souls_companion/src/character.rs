use rand::distributions::{Distribution, Uniform};

pub(crate) struct CardPacks {
    pub(crate) base: bool,
    pub(crate) fs_plus: bool,
    pub(crate) requiem: bool,
    pub(crate) gold_box: bool,
    pub(crate) promos: bool,
    pub(crate) customs: bool,
    pub(crate) friends: bool
}

impl CardPacks {
    pub(crate) fn new() -> CardPacks {
        CardPacks { base: false, fs_plus: false, requiem: false, gold_box: false, promos: false, customs: false, friends: false }
    }
}

pub(crate) struct Character {
    name: String,
    image_id: String
}

impl Character {
    pub(crate) fn new(name: &str, image_id: &str) -> Character {
        Character { name: name.to_string(), image_id: image_id.to_string(), }
    }

    pub(crate) fn get_name(&self) -> &String { &self.name }
    pub(crate) fn get_image_id(&self) -> &String { &self.image_id }

    pub(crate) fn generate_character(base: bool, fs_plus: bool, requiem: bool, gold_box: bool, promos: bool, customs: bool, friends: bool) -> Character {
        let sum = (11 * base as i32) + (4 * fs_plus as i32) + (22 * requiem as i32) + (4 * gold_box as i32) + (3 * promos as i32) + (11 * customs as i32) + (5 * friends as i32);
        if sum == 0 { return Character::new("", "0") }

        let mut choice = Uniform::from(0..sum).sample(&mut rand::thread_rng());

        if base {
            if choice > 10 { choice -= 11; }
            else {
                return match choice {
                    0 => Character::new("isaac", "1"),
                    1 => Character::new("maggy", "2"),
                    2 => Character::new("cain", "3"),
                    3 => Character::new("judas", "4"),
                    4 => Character::new("blue baby", "5"),
                    5 => Character::new("eve", "6"),
                    6 => Character::new("samson", "7"),
                    7 => Character::new("lazarus", "8"),
                    8 => Character::new("lilith", "9"),
                    9 => Character::new("the forgotten", "10"),
                    10 => Character::new("eden", "11"),
                    _ => panic!()
                };
            }
        }

        if fs_plus {
            if choice > 3 { choice -= 4; }
            else {
                return match choice {
                    0 => Character::new("bum-bo", "12"),
                    1 => Character::new("dark judas", "13"),
                    2 => Character::new("guppy", "14"),
                    3 => Character::new("whore of babylon", "15"),
                    _ => panic!()
                };
            }
        }

        if requiem {
            if choice > 21 { choice -= 22; }
            else {
                return match choice {
                    0 => Character::new("bethany", "16"),
                    1 => Character::new("jacob & esau", "17"),
                    2 => Character::new("the broken", "18"),
                    3 => Character::new("the dauntless", "19"),
                    4 => Character::new("the hoarder", "20"),
                    5 => Character::new("the deceiver", "21"),
                    6 => Character::new("the soiled", "22"),
                    7 => Character::new("the curdled", "23"),
                    8 => Character::new("the savage", "24"),
                    9 => Character::new("the benighted", "25"),
                    10 => Character::new("the enigma", "26"),
                    11 => Character::new("the capricious", "27"),
                    12 => Character::new("the baleful", "28"),
                    13 => Character::new("the harlot", "29"),
                    14 => Character::new("the miser", "30"),
                    15 => Character::new("the empty", "31"),
                    16 => Character::new("the fettered", "32"),
                    17 => Character::new("the zealot", "33"),
                    18 => Character::new("the deserter", "34"),
                    19 => Character::new("flash isaac", "35"),
                    20 => Character::new("eden", "36"),
                    21 => Character::new("eden", "37"),
                    _ => panic!()
                };
            }
        }

        if gold_box {
            if choice > 3 { choice -= 4; }
            else {
                return match choice {
                    0 => Character::new("azazel", "38"),
                    1 => Character::new("the lost", "39"),
                    2 => Character::new("the keeper", "40"),
                    3 => Character::new("apollyon", "41"),
                    _ => panic!()
                };
            }
        }

        if promos {
            if choice > 2 { choice -= 3; }
            else {
                return match choice {
                    0 => Character::new("steven", "42"),
                    1 => Character::new("eden", "43"),
                    2 => Character::new("eden", "44"),
                    _ => panic!()
                };
            }
        }

        if customs {
            if choice > 10 { choice -= 11; }
            else {
                return match choice {
                    0 => Character::new("sodom and gomorrah", "45"),
                    1 => Character::new("icarus", "46"),
                    2 => Character::new("tapeworm", "47"),
                    3 => Character::new("zaccheus", "48"),
                    4 => Character::new("the librarian", "49"),
                    5 => Character::new("edith", "50"),
                    6 => Character::new("the heartsick", "51"),
                    7 => Character::new("mei", "52"),
                    8 => Character::new("the stranger", "53"),
                    9 => Character::new("the tainted", "54"),
                    10 => Character::new("the turbulent", "55"),
                    _ => panic!()
                };
            }
        }

        if friends {
            return match choice {
                0 => Character::new("harper", "56"),
                1 => Character::new("kacie", "57"),
                2 => Character::new("kezo", "58"),
                3 => Character::new("king david", "59"),
                4 => Character::new("viet", "60"),
                _ => panic!()
            };
        }

        Character::new("error", "0")
    }
}