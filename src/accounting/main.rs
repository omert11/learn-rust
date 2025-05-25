use crate::accounting::customer::main as customer_actions;
use crate::accounting::transaction::main as transaction_actions;
use crate::utils::ui::ui_select;

pub fn main() {
    let actions = vec!["Customer Actions", "Transaction Actions", "Exit"];

    loop {
        println!("--------------------------------");
        println!("--------------------------------");
        let action = ui_select("Select an action", actions.clone());
        println!("--------------------------------");
        println!("--------------------------------");
        match action {
            0 => customer_actions(),
            1 => transaction_actions(),
            2 => break,
            _ => (),
        }
    }
}
