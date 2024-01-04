-- Your SQL goes here
CREATE TABLE vouchers (
  id VARCHAR NOT NULL PRIMARY KEY,
  voucherType VARCHAR,
  voucherStatus VARCHAR,
  voucherNumber VARCHAR,
  voucherDate TIMESTAMP,
  createdDate TIMESTAMP,
  updatedDate TIMESTAMP,
  dueDate TIMESTAMP,
  contactId VARCHAR,
  contactName VARCHAR,
  totalAmount FLOAT,
  openAmount FLOAT,
  currency VARCHAR,
  archived BOOLEAN
);

CREATE TABLE invoices (
  id VARCHAR NOT NULL PRIMARY KEY,
  organizationId VARCHAR,
  createdDate TIMESTAMP,
  updatedDate TIMESTAMP,
  version INTEGER,
  language VARCHAR,
  archived BOOLEAN,
  voucherStatus VARCHAR,
  voucherNumber VARCHAR,
  voucherDate TIMESTAMP,
  dueDate TIMESTAMP,
  address_id VARCHAR,
  address_name VARCHAR,
  address_supplement VARCHAR,
  address_street VARCHAR,
  address_city VARCHAR,
  address_zip VARCHAR,
  address_countryCode VARCHAR
);

CREATE TABLE addresses (
  contactId VARCHAR PRIMARY KEY,
  name VARCHAR,
  supplement VARCHAR,
  street VARCHAR,
  city VARCHAR,
  zip VARCHAR,
  countryCode VARCHAR
);

CREATE TABLE products (
  id VARCHAR PRIMARY KEY,
  type VARCHAR,
  name VARCHAR,
  description VARCHAR
);

CREATE TABLE line_items (
  id SERIAL PRIMARY KEY,
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
