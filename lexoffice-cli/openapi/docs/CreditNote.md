# CreditNote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**created_date** | Option<**String**> |  | [optional]
**updated_date** | Option<**String**> |  | [optional]
**version** | Option<**i32**> |  | [optional]
**language** | Option<**String**> |  | [optional]
**archived** | Option<**bool**> |  | [optional]
**voucher_status** | Option<**String**> |  | [optional]
**voucher_number** | Option<**String**> |  | [optional]
**voucher_date** | Option<**String**> |  | [optional]
**address** | Option<[**crate::models::VoucherAddress**](VoucherAddress.md)> |  | [optional]
**line_items** | Option<[**Vec<crate::models::LineItem>**](LineItem.md)> |  | [optional]
**total_price** | Option<[**crate::models::TotalPrice**](TotalPrice.md)> |  | [optional]
**tax_amounts** | Option<[**Vec<crate::models::TaxAmount>**](TaxAmount.md)> |  | [optional]
**tax_conditions** | Option<[**crate::models::TaxConditions**](TaxConditions.md)> |  | [optional]
**related_vouchers** | Option<[**Vec<crate::models::RelatedVoucher>**](RelatedVoucher.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**files** | Option<[**crate::models::File**](File.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


