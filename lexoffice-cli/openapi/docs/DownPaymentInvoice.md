# DownPaymentInvoice

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
**due_date** | Option<**String**> |  | [optional]
**address** | Option<[**crate::models::VoucherAddress**](VoucherAddress.md)> |  | [optional]
**line_items** | [**Vec<crate::models::LineItem>**](LineItem.md) |  | 
**total_price** | Option<[**crate::models::TotalPrice**](TotalPrice.md)> |  | [optional]
**tax_amounts** | Option<[**Vec<crate::models::TaxAmount>**](TaxAmount.md)> |  | [optional]
**tax_conditions** | Option<[**crate::models::TaxConditions**](TaxConditions.md)> |  | [optional]
**payment_conditions** | Option<[**crate::models::PaymentConditions**](PaymentConditions.md)> |  | [optional]
**shipping_conditions** | Option<[**crate::models::ShippingConditions**](ShippingConditions.md)> |  | [optional]
**closing_invoice_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**related_vouchers** | Option<[**Vec<crate::models::RelatedVoucher>**](RelatedVoucher.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**files** | Option<[**crate::models::File**](File.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


