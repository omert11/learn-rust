-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_customer_balance_trigger ON transactions;
DROP TABLE IF EXISTS "transactions";
DROP FUNCTION IF EXISTS update_customer_balance();
