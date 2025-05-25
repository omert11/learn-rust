use bigdecimal::BigDecimal;
use learn_rust::models::NewTransaction;

use crate::accounting::crud::{add_transaction, list_transactions};
use crate::accounting::customer::select_customer;
use crate::utils::ui::{get_input, ui_select};

fn print_transactions() {
    let customer = select_customer();
    let transactions = list_transactions(customer.id);
    println!("{}", customer.to_string());
    println!(
        "{}",
        transactions
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn add_transaction_ui() {
    let customer = select_customer();
    let amount = get_input::<BigDecimal>("Enter amount");
    let description = get_input::<String>("Enter description");
    let before_balance = customer.balance.clone();
    let after_balance = before_balance.clone() + amount.clone();
    let transaction_type = match amount.clone() > BigDecimal::from(0) {
        true => "deposit".to_string(),
        false => "withdraw".to_string(),
    };
    let transaction = NewTransaction {
        customer_id: customer.id,
        before_balance,
        after_balance,
        amount,
        transaction_type,
        description: description,
    };
    add_transaction(transaction);
}

pub fn main() {
    let actions = vec!["List Customer Transactions", "Add Transaction", "Exit"];
    let action = ui_select("Select an action", actions);
    println!("--------------------------------");
    println!("--------------------------------");
    match action {
        0 => print_transactions(),
        1 => add_transaction_ui(),
        _ => (),
    }
}
