lazy_static::lazy_static! {
    static ref WORDS: Vec<&'static str> = {
        let str = include_str!("../../resources/words.txt");
        str.lines().collect()
    };
}

pub fn create_words_password(length: i8, separator: &str) -> String {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut words = vec![];

    for _ in 0..length {
        let pos = rng.gen_range(0, WORDS.len() - 1);
        words.push(WORDS[pos].to_owned());
    }

    words.join(separator)
}
