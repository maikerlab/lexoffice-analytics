# LineItemQuotation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**r#type** | **String** |  | 
**name** | **String** |  | 
**description** | **String** |  | 
**quantity** | **f64** |  | 
**unit_name** | **String** |  | 
**unit_price** | [**models::UnitPrice**](UnitPrice.md) |  | 
**discount_percentage** | **f64** |  | 
**line_item_amount** | **f64** |  | 
**sub_items** | Option<[**Vec<models::LineItemQuotation>**](LineItemQuotation.md)> |  | [optional]
**optional** | Option<**bool**> |  | [optional]
**alternative** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


