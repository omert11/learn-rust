use diesel::prelude::*;
use learn_rust::models::{Customer, NewCustomer, NewTransaction, Transaction};
use learn_rust::*;

pub fn add_customer(customer: NewCustomer) -> i16 {
    use learn_rust::schema::customers;

    let connection = &mut establish_connection();

    let result = diesel::insert_into(customers::table)
        .values(&customer)
        .on_conflict(customers::id)
        .do_nothing()
        .returning(customers::id)
        .get_result(connection);

    match result {
        Ok(id) => id,
        Err(e) => {
            println!("Error adding customer: {}", e);
            0
        }
    }
}

pub fn list_customers() -> Vec<Customer> {
    use learn_rust::schema::customers;

    let connection = &mut establish_connection();

    let results = customers::table
        .load::<Customer>(connection)
        .expect("Error loading customers");
    results
}

pub fn update_customer(customer: Customer) -> i16 {
    use learn_rust::schema::customers;

    let connection = &mut establish_connection();

    let result = diesel::update(customers::table)
        .filter(customers::id.eq(customer.id))
        .set(customer)
        .returning(customers::id)
        .get_result(connection);

    match result {
        Ok(id) => id,
        Err(e) => {
            println!("Error updating customer: {}", e);
            0
        }
    }
}

pub fn delete_customer(customer: Customer) -> bool {
    use learn_rust::schema::customers;

    let connection = &mut establish_connection();

    let result = diesel::delete(customers::table)
        .filter(customers::id.eq(customer.id))
        .execute(connection);

    match result {
        Ok(_) => true,
        Err(e) => {
            println!("Error deleting customer: {}", e);
            false
        }
    }
}

pub fn list_transactions(customer_id: i16) -> Vec<Transaction> {
    use learn_rust::schema::transactions;

    let connection = &mut establish_connection();

    let results = transactions::table
        .filter(transactions::customer_id.eq(customer_id))
        .order(transactions::updated_at.desc())
        .limit(25)
        .select(Transaction::as_select())
        .load::<Transaction>(connection)
        .expect("Error loading transactions");
    results
}

pub fn add_transaction(transaction: NewTransaction) -> i32 {
    use learn_rust::schema::transactions;

    let connection = &mut establish_connection();

    let result = diesel::insert_into(transactions::table)
        .values(&transaction)
        .returning(transactions::id)
        .get_result(connection);

    match result {
        Ok(id) => id,
        Err(e) => {
            println!("Error adding transaction: {}", e);
            0
        }
    }
}
