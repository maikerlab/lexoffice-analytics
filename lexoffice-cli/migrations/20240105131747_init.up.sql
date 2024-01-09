-- Add migration script here
CREATE TABLE vouchers (
  id VARCHAR NOT NULL PRIMARY KEY,
  voucher_type VARCHAR NOT NULL,
  voucher_status VARCHAR NOT NULL,
  voucher_number VARCHAR NOT NULL,
  voucher_date TIMESTAMP NOT NULL,
  created_date TIMESTAMP NOT NULL,
  updated_date TIMESTAMP NOT NULL,
  due_date TIMESTAMP,
  contact_id VARCHAR,
  contact_name VARCHAR,
  total_amount FLOAT,
  open_amount FLOAT,
  currency VARCHAR NOT NULL DEFAULT 'EUR',
  archived BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE invoices (
  id VARCHAR NOT NULL PRIMARY KEY,
  organization_id VARCHAR,
  created_date TIMESTAMP NOT NULL,
  updated_date TIMESTAMP NOT NULL,
  version INTEGER NOT NULL DEFAULT 0,
  language VARCHAR NOT NULL DEFAULT 'DE',
  archived BOOLEAN NOT NULL DEFAULT FALSE,
  voucher_status VARCHAR NOT NULL,
  voucher_number VARCHAR NOT NULL,
  voucher_date TIMESTAMP NOT NULL,
  due_date TIMESTAMP,
  address_id VARCHAR,
  address_name VARCHAR NOT NULL,
  address_supplement VARCHAR,
  address_street VARCHAR,
  address_city VARCHAR,
  address_zip VARCHAR,
  address_countryCode VARCHAR
);

CREATE TABLE addresses (
  id INT NOT NULL PRIMARY KEY,
  contact_id VARCHAR,
  name VARCHAR NOT NULL,
  supplement VARCHAR,
  street VARCHAR,
  city VARCHAR,
  zip VARCHAR,
  countryCode VARCHAR
);

CREATE TABLE products (
  id VARCHAR NOT NULL PRIMARY KEY,
  type VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR
);

CREATE TABLE line_items (
  id SERIAL NOT NULL PRIMARY KEY,
  product_id VARCHAR NOT NULL,
  voucher_id VARCHAR NOT NULL,
  quantity INTEGER NOT NULL DEFAULT 1,
  unit_name VARCHAR,
  currency VARCHAR NOT NULL DEFAULT 'EUR',
  net_amount FLOAT,
  gross_amount FLOAT,
  tax_rate_percentage INTEGER,
  CONSTRAINT fk_product FOREIGN KEY(product_id) REFERENCES products(id),
  CONSTRAINT fk_voucher FOREIGN KEY(voucher_id) REFERENCES vouchers(id)
);
