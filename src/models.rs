use std::fmt::Display;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Customer {
    pub id: i16,
    pub name: String,
    pub balance: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::customers)]
pub struct NewCustomer {
    pub name: String,
    pub balance: BigDecimal,
}

impl Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} ({} TRY)",
            self.id,
            self.name,
            self.balance.to_string()
        )
    }
}

#[derive(Debug, Queryable, Selectable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub customer_id: i16,
    pub before_balance: BigDecimal,
    pub after_balance: BigDecimal,
    pub amount: BigDecimal,
    #[diesel(column_name = "type_")]
    pub transaction_type: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::transactions)]
pub struct NewTransaction {
    pub customer_id: i16,
    pub before_balance: BigDecimal,
    pub after_balance: BigDecimal,
    pub amount: BigDecimal,
    #[diesel(column_name = "type_")]
    pub transaction_type: String,
    pub description: String,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} TRY",
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.description,
            self.amount.to_string()
        )
    }
}
