-- Your SQL goes here
CREATE TABLE "customers"(
	"id" SMALLSERIAL PRIMARY KEY,
	"name" VARCHAR(50) NOT NULL,
	"balance" NUMERIC(10, 3) NOT NULL DEFAULT 0,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('customers');

