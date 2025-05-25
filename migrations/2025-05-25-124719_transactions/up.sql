-- Your SQL goes here
CREATE TABLE "transactions" (
    "id" SERIAL PRIMARY KEY,
    "customer_id" INT2 NOT NULL REFERENCES "customers"("id"),
    "before_balance" DECIMAL(10, 2) NOT NULL,
    "after_balance" DECIMAL(10, 2) NOT NULL,
    "amount" DECIMAL(10, 2) NOT NULL,
    "type" VARCHAR(255) NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('transactions');


CREATE OR REPLACE FUNCTION update_customer_balance() RETURNS TRIGGER AS $$
BEGIN
    UPDATE customers SET balance = NEW.after_balance WHERE id = NEW.customer_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_customer_balance_trigger AFTER INSERT ON transactions FOR EACH ROW EXECUTE FUNCTION update_customer_balance();