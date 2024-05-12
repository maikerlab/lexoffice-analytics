# DeliveryNote

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
**address** | Option<[**models::VoucherAddress**](VoucherAddress.md)> |  | [optional]
**line_items** | [**Vec<models::LineItem>**](LineItem.md) |  | 
**tax_conditions** | Option<[**models::TaxConditions**](TaxConditions.md)> |  | [optional]
**related_vouchers** | Option<[**Vec<models::RelatedVoucher>**](RelatedVoucher.md)> |  | [optional]
**title** | Option<**String**> |  | [optional]
**introduction** | Option<**String**> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**delivery_terms** | Option<**String**> |  | [optional]
**files** | Option<[**models::File**](File.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


