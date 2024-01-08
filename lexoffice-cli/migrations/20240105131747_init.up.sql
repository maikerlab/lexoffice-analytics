-- Add migration script here
CREATE TABLE vouchers (
  id VARCHAR NOT NULL PRIMARY KEY,
  voucher_type VARCHAR,
  voucher_status VARCHAR,
  voucher_number VARCHAR,
  voucher_date TIMESTAMP,
  created_date TIMESTAMP,
  updated_date TIMESTAMP,
  due_date TIMESTAMP,
  contact_id VARCHAR,
  contact_name VARCHAR,
  total_amount FLOAT,
  open_amount FLOAT,
  currency VARCHAR,
  archived BOOLEAN
);

CREATE TABLE invoices (
  id VARCHAR NOT NULL PRIMARY KEY,
  organization_id VARCHAR,
  created_date TIMESTAMP,
  updated_date TIMESTAMP,
  version INTEGER,
  language VARCHAR,
  archived BOOLEAN,
  voucher_status VARCHAR,
  voucher_number VARCHAR,
  voucher_date TIMESTAMP,
  due_date TIMESTAMP,
  address_id VARCHAR,
  address_name VARCHAR,
  address_supplement VARCHAR,
  address_street VARCHAR,
  address_city VARCHAR,
  address_zip VARCHAR,
  address_countryCode VARCHAR
);

CREATE TABLE addresses (
  id INT NOT NULL PRIMARY KEY,
  contact_id VARCHAR,
  name VARCHAR,
  supplement VARCHAR,
  street VARCHAR,
  city VARCHAR,
  zip VARCHAR,
  countryCode VARCHAR
);

CREATE TABLE products (
  id VARCHAR NOT NULL PRIMARY KEY,
  type VARCHAR,
  name VARCHAR,
  description VARCHAR
);

CREATE TABLE line_items (
  id SERIAL NOT NULL PRIMARY KEY,
  product_id VARCHAR,
  voucher_id VARCHAR,
  quantity INTEGER,
  unit_name VARCHAR,
  currency VARCHAR,
  net_amount FLOAT,
  gross_amount FLOAT,
  tax_rate_percentage INTEGER,
  CONSTRAINT fk_product FOREIGN KEY(product_id) REFERENCES products(id),
  CONSTRAINT fk_voucher FOREIGN KEY(voucher_id) REFERENCES vouchers(id)
);
