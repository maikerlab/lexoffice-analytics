# Invoice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**created_date** | **String** |  | 
**updated_date** | **String** |  | 
**version** | **i32** |  | 
**language** | **String** |  | 
**archived** | **bool** |  | 
**voucher_status** | **String** |  | 
**voucher_number** | **String** |  | 
**voucher_date** | **String** |  | 
**due_date** | **String** |  | 
**address** | [**models::VoucherAddress**](VoucherAddress.md) |  | 
**x_rechnung** | Option<[**models::InvoiceXRechnung**](Invoice_xRechnung.md)> |  | [optional]
**line_items** | [**Vec<models::LineItem>**](LineItem.md) |  | 
**total_price** | [**models::TotalPrice**](TotalPrice.md) |  | 
**tax_amounts** | Option<[**Vec<models::TaxAmount>**](TaxAmount.md)> |  | [optional]
**tax_conditions** | Option<[**models::TaxConditions**](TaxConditions.md)> |  | [optional]
**payment_conditions** | Option<[**models::PaymentConditions**](PaymentConditions.md)> |  | [optional]
**shipping_conditions** | Option<[**models::ShippingConditions**](ShippingConditions.md)> |  | [optional]
**closing_invoice** | **bool** |  | 
**claimed_gross_amount** | Option<**f64**> |  | [optional]
**down_payment_deductions** | Option<[**serde_json::Value**](.md)> |  | [optional]
**recurring_template_id** | Option<**String**> |  | [optional]
**related_vouchers** | Option<[**Vec<models::RelatedVoucher>**](RelatedVoucher.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**files** | Option<[**models::File**](File.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


