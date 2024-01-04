// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (contactid) {
        contactid -> Varchar,
        name -> Nullable<Varchar>,
        supplement -> Nullable<Varchar>,
        street -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        zip -> Nullable<Varchar>,
        countrycode -> Nullable<Varchar>,
    }
}

diesel::table! {
    invoices (id) {
        id -> Varchar,
        organizationid -> Nullable<Varchar>,
        createddate -> Nullable<Timestamp>,
        updateddate -> Nullable<Timestamp>,
        version -> Nullable<Int4>,
        language -> Nullable<Varchar>,
        archived -> Nullable<Bool>,
        voucherstatus -> Nullable<Varchar>,
        vouchernumber -> Nullable<Varchar>,
        voucherdate -> Nullable<Timestamp>,
        duedate -> Nullable<Timestamp>,
        address_id -> Nullable<Varchar>,
        address_name -> Nullable<Varchar>,
        address_supplement -> Nullable<Varchar>,
        address_street -> Nullable<Varchar>,
        address_city -> Nullable<Varchar>,
        address_zip -> Nullable<Varchar>,
        address_countrycode -> Nullable<Varchar>,
    }
}

diesel::table! {
    line_items (id) {
        id -> Int4,
        product_id -> Nullable<Varchar>,
        voucher_id -> Nullable<Varchar>,
        quantity -> Nullable<Int4>,
        unit_name -> Nullable<Varchar>,
        currency -> Nullable<Varchar>,
        net_amount -> Nullable<Float8>,
        gross_amount -> Nullable<Float8>,
        tax_rate_percentage -> Nullable<Int4>,
    }
}

diesel::table! {
    products (id) {
        id -> Varchar,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    vouchers (id) {
        id -> Varchar,
        vouchertype -> Nullable<Varchar>,
        voucherstatus -> Nullable<Varchar>,
        vouchernumber -> Nullable<Varchar>,
        voucherdate -> Nullable<Timestamp>,
        createddate -> Nullable<Timestamp>,
        updateddate -> Nullable<Timestamp>,
        duedate -> Nullable<Timestamp>,
        contactid -> Nullable<Varchar>,
        contactname -> Nullable<Varchar>,
        totalamount -> Nullable<Float8>,
        openamount -> Nullable<Float8>,
        currency -> Nullable<Varchar>,
        archived -> Nullable<Bool>,
    }
}

diesel::joinable!(line_items -> products (product_id));
diesel::joinable!(line_items -> vouchers (voucher_id));

diesel::allow_tables_to_appear_in_same_query!(
    addresses,
    invoices,
    line_items,
    products,
    vouchers,
);
