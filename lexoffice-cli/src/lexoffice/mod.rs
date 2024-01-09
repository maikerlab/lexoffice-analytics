use std::{thread::sleep, time::Duration};
pub mod utils;

use openapi::{
    apis::{
        configuration::Configuration,
        invoices_api::{invoices_id_get, InvoicesIdGetError},
        vouchers_api::{voucherlist_get, VoucherlistGetError},
        Error,
    },
    models::{invoice, voucherlist_voucher, Invoice, VoucherList},
};

pub trait EnumToString {
    fn enum_to_string(&self) -> String;
}

impl EnumToString for voucherlist_voucher::VoucherStatus {
    fn enum_to_string(&self) -> String {
        match self {
            voucherlist_voucher::VoucherStatus::Open => "open".to_string(),
            voucherlist_voucher::VoucherStatus::Paid => "paid".to_string(),
            voucherlist_voucher::VoucherStatus::Paidoff => "paidoff".to_string(),
            voucherlist_voucher::VoucherStatus::Voided => "voided".to_string(),
            voucherlist_voucher::VoucherStatus::Transferred => "transferred".to_string(),
            voucherlist_voucher::VoucherStatus::Sepadebit => "sepadebit".to_string(),
            voucherlist_voucher::VoucherStatus::Draft => "draft".to_string(),
            voucherlist_voucher::VoucherStatus::Overdue => "overdue".to_string(),
            voucherlist_voucher::VoucherStatus::Accepted => "accepted".to_string(),
            voucherlist_voucher::VoucherStatus::Rejected => "rejected".to_string(),
        }
    }
}

impl EnumToString for voucherlist_voucher::VoucherType {
    fn enum_to_string(&self) -> String {
        match self {
            voucherlist_voucher::VoucherType::Salesinvoice => "salesinvoice".to_string(),
            voucherlist_voucher::VoucherType::Salescreditnote => "salescreditnote".to_string(),
            voucherlist_voucher::VoucherType::Purchaseinvoice => "purchaseinvoice".to_string(),
            voucherlist_voucher::VoucherType::Purchasecreditnote => {
                "purchasecreditnote".to_string()
            }
            voucherlist_voucher::VoucherType::Invoice => "invoice".to_string(),
            voucherlist_voucher::VoucherType::Downpaymentinvoice => {
                "downpaymentinvoice".to_string()
            }
            voucherlist_voucher::VoucherType::Creditnote => "creditnote".to_string(),
            voucherlist_voucher::VoucherType::Orderconfirmation => "orderconfirmation".to_string(),
            voucherlist_voucher::VoucherType::Quotation => "quotation".to_string(),
            voucherlist_voucher::VoucherType::Deliverynote => "deliverynote".to_string(),
        }
    }
}

impl EnumToString for voucherlist_voucher::Currency {
    fn enum_to_string(&self) -> String {
        match self {
            voucherlist_voucher::Currency::Eur => "EUR".to_string(),
        }
    }
}

impl EnumToString for invoice::VoucherStatus {
    fn enum_to_string(&self) -> String {
        match self {
            invoice::VoucherStatus::Draft => "draft".to_string(),
            invoice::VoucherStatus::Open => "open".to_string(),
            invoice::VoucherStatus::Paid => "paid".to_string(),
            invoice::VoucherStatus::Voided => "voided".to_string(),
        }
    }
}

impl EnumToString for invoice::Language {
    fn enum_to_string(&self) -> String {
        match self {
            invoice::Language::De => "DE".to_string(),
            invoice::Language::En => "EN".to_string(),
        }
    }
}

pub const MAX_REQUESTS_PER_SECOND: f32 = 2.0;

fn request_delay() {
    sleep(Duration::from_millis(utils::get_api_rate_ms(
        MAX_REQUESTS_PER_SECOND,
    )));
}

pub struct LexofficeApi {
    conf: Configuration,
}

impl LexofficeApi {
    pub fn new(api_key: String) -> Self {
        let mut api_config = Configuration::default();
        api_config.bearer_access_token = Some(api_key);
        Self { conf: api_config }
    }

    pub async fn get_voucherlist(
        &self,
        voucher_type: String,
        page: i32,
        size: i32,
    ) -> Result<VoucherList, Error<VoucherlistGetError>> {
        request_delay();
        println!("syncing voucherlist (page {})", page);

        voucherlist_get(
            &self.conf,
            voucher_type.as_str(),
            "any",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(page),
            Some(size),
            Some("voucherDate,DESC"),
        )
        .await
    }

    pub async fn get_invoice(&self, id: String) -> Result<Invoice, Error<InvoicesIdGetError>> {
        request_delay();
        let response = invoices_id_get(&self.conf, id.as_str()).await;
        response
    }
}
