-- Add up migration script here
ALTER TABLE invoices
ALTER COLUMN total_net_amount TYPE NUMERIC,
ALTER COLUMN total_gross_amount TYPE NUMERIC,
ALTER COLUMN total_tax_amount TYPE NUMERIC,
ALTER COLUMN total_discount_absolute TYPE NUMERIC,
ALTER COLUMN total_discount_percentage TYPE NUMERIC;

UPDATE invoices
SET total_net_amount = ROUND(total_net_amount, 2),
    total_gross_amount = ROUND(total_gross_amount, 2),
    total_tax_amount = ROUND(total_tax_amount, 2),
    total_discount_absolute = ROUND(total_discount_absolute, 2),
    total_discount_percentage = ROUND(total_discount_percentage, 2);

ALTER TABLE line_items
ALTER COLUMN quantity TYPE NUMERIC,
ALTER COLUMN net_amount TYPE NUMERIC,
ALTER COLUMN gross_amount TYPE NUMERIC,
ALTER COLUMN tax_rate_percentage TYPE NUMERIC,
ALTER COLUMN discount_percentage TYPE NUMERIC,
ALTER COLUMN line_item_amount TYPE NUMERIC;

UPDATE line_items
SET quantity = ROUND(quantity, 2),
    net_amount = ROUND(net_amount, 2),
    gross_amount = ROUND(gross_amount, 2),
    tax_rate_percentage = ROUND(tax_rate_percentage, 2),
    discount_percentage = ROUND(discount_percentage, 2),
    line_item_amount = ROUND(line_item_amount, 2);

ALTER TABLE voucherlist
ALTER COLUMN total_amount TYPE NUMERIC,
ALTER COLUMN open_amount TYPE NUMERIC;

UPDATE voucherlist
SET total_amount = ROUND(total_amount, 2),
    open_amount = ROUND(open_amount, 2);