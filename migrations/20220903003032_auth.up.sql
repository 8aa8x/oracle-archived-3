-- This schema is defined by NextAuth.js, adapted to Postgres
-- See: https://github.com/hexcowboy/nextauth-slonik
create schema if not exists auth;

create table if not exists auth.verification_token (
  identifier text not null,
  expires timestamptz not null,
  token text not null,
  primary key (identifier, token)
);

create table if not exists auth.accounts (
  id int primary key generated always as identity,
  user_id int not null,
  "type" varchar(255) not null,
  provider varchar(255) not null,
  provider_account_id varchar(255) not null,
  refresh_token text,
  access_token text,
  expires_at bigint,
  id_token text,
  scope text,
  session_state text,
  token_type text
);

create table if not exists auth.sessions (
  id int primary key generated always as identity,
  user_id int not null,
  expires timestamptz not null,
  session_token varchar(255) not null
);

create table if not exists auth.users (
  id int primary key generated always as identity,
  name varchar(255),
  email varchar(255),
  email_verified timestamptz,
  image text,
  eth_address char(42)
);
