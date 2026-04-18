pub mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Self {
                toast: toast.to_string(),
                seasonal_fruit: String::from("melocoton"),
            }
        }

        pub fn seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Appetizer {
        pub fn label(&self) -> &'static str {
            match self {
                Self::Soup => "sopa",
                Self::Salad => "ensalada",
            }
        }
    }

    pub fn fix_incorrect_order() -> &'static str {
        cook_order();
        super::front_of_house::serving::serve_order()
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() -> Vec<String> {
    let mut log = vec![hosting::add_to_waitlist().to_string()];
    log.push(front_of_house::serving::take_order().to_string());

    let mut meal = back_of_house::Breakfast::summer("centeno");
    meal.toast = String::from("integral");

    log.push(format!("desayuno con tostada de {}", meal.toast));
    log.push(format!("fruta de temporada: {}", meal.seasonal_fruit()));

    let starters = [back_of_house::Appetizer::Soup, back_of_house::Appetizer::Salad];
    for starter in starters {
        log.push(format!("entrante: {}", starter.label()));
    }

    log.push(back_of_house::fix_incorrect_order().to_string());
    log.push(hosting::seat_at_table().to_string());
    log
}

#[cfg(test)]
mod tests {
    use super::eat_at_restaurant;

    #[test]
    fn package_exposes_a_small_public_api() {
        let log = eat_at_restaurant();

        assert!(log.iter().any(|line| line.contains("lista de espera")));
        assert!(log.iter().any(|line| line.contains("pedido servido")));
        assert!(log.iter().any(|line| line.contains("fruta de temporada")));
    }
}
