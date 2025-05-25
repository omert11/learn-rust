use bigdecimal::BigDecimal;
use learn_rust::models::{Customer, NewCustomer};

use crate::accounting::crud::{add_customer, delete_customer, list_customers, update_customer};
use crate::utils::ui::{get_input, ui_select};

fn print_customers() {
    let customers = list_customers();
    println!(
        "{}",
        customers
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn select_customer() -> Customer {
    let customers = list_customers();
    let customer_names = customers
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>();
    let index = ui_select(
        "Select a customer",
        customer_names
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<&str>>(),
    );
    customers[index as usize].clone()
}

fn add_customer_ui() {
    let name = get_input::<String>("Enter customer name");
    let balance = get_input::<BigDecimal>("Enter customer balance");
    let customer = NewCustomer { name, balance };
    add_customer(customer);
}

fn update_customer_ui() {
    let customer = select_customer();
    let name = get_input::<String>("Enter customer name");
    let balance = get_input::<BigDecimal>("Enter customer balance");
    let customer = Customer {
        id: customer.id,
        name,
        balance,
        created_at: customer.created_at,
        updated_at: customer.updated_at,
    };
    update_customer(customer);
}

fn delete_customer_ui() {
    let customer = select_customer();
    delete_customer(customer);
}

pub fn main() {
    let actions = vec![
        "List customers",
        "Add customer",
        "Update customer",
        "Delete customer",
    ];
    let action = ui_select("Select an action", actions);
    println!("--------------------------------");
    println!("--------------------------------");
    match action {
        0 => print_customers(),
        1 => add_customer_ui(),
        2 => update_customer_ui(),
        3 => delete_customer_ui(),
        _ => (),
    }
}
