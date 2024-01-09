# LineItemQuotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | **String** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**quantity** | **f32** |  | 
**unit_name** | Option<**String**> |  | [optional]
**unit_price** | Option<[**crate::models::UnitPrice**](UnitPrice.md)> |  | [optional]
**discount_percentage** | Option<**f32**> |  | [optional]
**line_item_amount** | Option<**f32**> |  | [optional]
**sub_items** | Option<[**Vec<crate::models::LineItemQuotation>**](LineItemQuotation.md)> |  | [optional]
**optional** | Option<**bool**> |  | [optional]
**alternative** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


