CREATE TYPE "data_source" AS ENUM (
  'place_order',
  'trade_india',
  'rapidor'
);

CREATE TABLE  IF NOT EXISTS short_url(
    id SERIAL PRIMARY KEY,
    short_url TEXT NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    user_id uuid NOT NULL
);

CREATE TABLE IF NOT EXISTS user_account(
    id uuid PRIMARY KEY,
    company_name TEXT NOT NULL,
    username TEXT NOT NULL
);