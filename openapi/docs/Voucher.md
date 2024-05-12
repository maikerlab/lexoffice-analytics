# Voucher

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**organization_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | **String** | Voucher Type | 
**voucher_status** | **String** | Status | 
**voucher_number** | Option<**String**> |  | [optional]
**voucher_date** | **String** |  | 
**shipping_date** | Option<**String**> |  | [optional]
**due_date** | Option<**String**> |  | [optional]
**total_gross_amount** | Option<**f64**> |  | [optional]
**total_tax_amount** | Option<**f64**> |  | [optional]
**tax_type** | Option<**String**> |  | [optional]
**use_collective_contact** | Option<**bool**> |  | [optional]
**contact_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**remark** | Option<**String**> |  | [optional]
**voucher_items** | Option<[**Vec<models::VoucherVoucherItemsInner>**](Voucher_voucherItems_inner.md)> |  | [optional]
**files** | Option<[**Vec<models::File>**](File.md)> |  | [optional]
**created_date** | Option<**String**> |  | [optional]
**updated_date** | Option<**String**> |  | [optional]
**version** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


