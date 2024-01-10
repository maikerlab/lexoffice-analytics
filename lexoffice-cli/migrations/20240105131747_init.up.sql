-- Add migration script here
CREATE TABLE voucherlist (
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

CREATE TABLE addresses (
  contact_id VARCHAR NOT NULL PRIMARY KEY,
  type VARCHAR NOT NULL DEFAULT 'collective',
  name VARCHAR NOT NULL,
  supplement VARCHAR,
  street VARCHAR,
  city VARCHAR,
  zip VARCHAR,
  country_code VARCHAR
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
  currency VARCHAR NOT NULL DEFAULT 'EUR',
  total_net_amount FLOAT NOT NULL DEFAULT 0.0,
  total_gross_amount FLOAT NOT NULL DEFAULT 0.0,
  total_tax_amount FLOAT NOT NULL DEFAULT 0.0,
  total_discount_absolute FLOAT NOT NULL DEFAULT 0.0,
  total_discount_percentage FLOAT NOT NULL DEFAULT 0.0,
  CONSTRAINT fk_address FOREIGN KEY(address_id) REFERENCES addresses(contact_id)
);

CREATE TABLE products (
  id VARCHAR NOT NULL PRIMARY KEY,
  type VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE line_items (
  id SERIAL NOT NULL PRIMARY KEY,
  product_id VARCHAR NOT NULL,
  voucher_id VARCHAR NOT NULL,
  quantity FLOAT NOT NULL DEFAULT 1.0,
  unit_name VARCHAR NOT NULL,
  currency VARCHAR NOT NULL DEFAULT 'EUR',
  net_amount FLOAT NOT NULL,
  gross_amount FLOAT NOT NULL,
  tax_rate_percentage FLOAT,
  discount_percentage FLOAT,
  line_item_amount FLOAT,
  CONSTRAINT fk_product FOREIGN KEY(product_id) REFERENCES products(id),
  CONSTRAINT fk_voucher FOREIGN KEY(voucher_id) REFERENCES voucherlist(id)
);
