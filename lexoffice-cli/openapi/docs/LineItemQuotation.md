# LineItemQuotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**quantity** | **f32** |  | 
**unit_name** | **String** |  | 
**unit_price** | [**crate::models::UnitPrice**](UnitPrice.md) |  | 
**discount_percentage** | **f32** |  | 
**line_item_amount** | **f32** |  | 
**sub_items** | Option<[**Vec<crate::models::LineItemQuotation>**](LineItemQuotation.md)> |  | [optional]
**optional** | Option<**bool**> |  | [optional]
**alternative** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


