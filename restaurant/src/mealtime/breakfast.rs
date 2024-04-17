use crate::temporal::season::Season;

pub struct Breakfast {
    toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn new(toast: String) -> Breakfast {
        Breakfast {
            toast,
            seasonal_fruit: String::from(Season::get_some_season_fruit_name()),
        }
    }

    pub fn get_toast(&self) -> &String {
        &self.toast
    }

    pub fn get_seasonal_fruit(&self) -> &String {
        &self.seasonal_fruit
    }
}
