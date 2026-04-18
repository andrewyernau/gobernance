#[derive(Debug, Clone, Copy)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}

#[derive(Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state(state: UsState) -> String {
    if state.existed_in(1900) {
        format!("{state:?} is pretty old, for America!")
    } else {
        format!("{state:?} is relatively new.")
    }
}

fn describe_state_quarter_with_if_let(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    Some(describe_state(state))
}

fn describe_state_quarter_with_let_else(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    Some(describe_state(state))
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alaska);
    let penny = Coin::Penny;
    let _other_examples = [Coin::Nickel, Coin::Dime];

    if let Coin::Quarter(state) = quarter {
        println!("Captured with if let: {state:?}");
    }

    if let Some(desc) = describe_state_quarter_with_if_let(quarter) {
        println!("if let path: {desc}");
    }

    if let Some(desc) = describe_state_quarter_with_let_else(Coin::Quarter(UsState::Alabama)) {
        println!("let else path: {desc}");
    }

    if describe_state_quarter_with_let_else(penny).is_none() {
        println!("Non-quarter coins return None.");
    }
}
