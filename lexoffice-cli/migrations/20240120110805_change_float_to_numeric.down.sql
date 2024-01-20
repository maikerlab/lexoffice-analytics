-- Add down migration script here
ALTER TABLE invoices
ALTER COLUMN total_net_amount TYPE FLOAT,
ALTER COLUMN total_gross_amount TYPE FLOAT,
ALTER COLUMN total_tax_amount TYPE FLOAT,
ALTER COLUMN total_discount_absolute TYPE FLOAT,
ALTER COLUMN total_discount_percentage TYPE FLOAT;

ALTER TABLE line_items
ALTER COLUMN quantity TYPE FLOAT,
ALTER COLUMN net_amount TYPE FLOAT,
ALTER COLUMN gross_amount TYPE FLOAT,
ALTER COLUMN tax_rate_percentage TYPE FLOAT,
ALTER COLUMN discount_percentage TYPE FLOAT,
ALTER COLUMN line_item_amount TYPE FLOAT;

ALTER TABLE voucherlist
ALTER COLUMN total_amount TYPE FLOAT,
ALTER COLUMN open_amount TYPE FLOAT;
