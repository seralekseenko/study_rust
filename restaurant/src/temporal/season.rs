use chrono::{Datelike, Local}; // DateLike is an interface for DateTime<Tz>, NaiveDate, etc.
use rand::Rng;

pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

impl Season {
    pub fn get_some_season_fruit_name() -> &'static str {
        let today = Local::now().date_naive();
        let current_season = Season::from_date(today);
        return current_season.random_fruit();
    }

    fn from_date(date: chrono::NaiveDate) -> Self {
        match date.month() {
            12 | 1 | 2 => Season::Winter,
            3 | 4 | 5 => Season::Spring,
            6 | 7 | 8 => Season::Summer,
            9 | 10 | 11 => Season::Autumn,
            _ => unreachable!(),
        }
    }

    fn random_fruit(&self) -> &'static str {
        let fruits = match self {
            Season::Winter => vec!["apples", "pears", "oranges"],
            Season::Spring => vec!["strawberries", "cherries", "apricots"],
            Season::Summer => vec!["peaches", "nectarines", "watermelon"],
            Season::Autumn => vec!["grapes", "figs", "pomegranates"],
        };
        let index = rand::thread_rng().gen_range(0..fruits.len());
        fruits[index]
    }
}
