# \VouchersApi

All URIs are relative to *https://api.lexoffice.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**voucherlist_get**](VouchersApi.md#voucherlist_get) | **GET** /voucherlist | Retrieve Voucherlist
[**vouchers_get**](VouchersApi.md#vouchers_get) | **GET** /vouchers | Filter voucher by voucherNumber
[**vouchers_id_get**](VouchersApi.md#vouchers_id_get) | **GET** /vouchers/{id} | Find a voucher by ID



## voucherlist_get

> crate::models::VoucherList voucherlist_get(voucher_type, voucher_status, archived, contact_id, voucher_date_from, voucher_date_to, created_date_from, created_date_to, updated_date_from, updated_date_to, voucher_number, page, size, sort)
Retrieve Voucherlist

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_type** | **String** |  | [required] |
**voucher_status** | **String** |  | [required] |
**archived** | Option<**bool**> |  |  |
**contact_id** | Option<**uuid::Uuid**> |  |  |
**voucher_date_from** | Option<**String**> |  |  |
**voucher_date_to** | Option<**String**> |  |  |
**created_date_from** | Option<**String**> |  |  |
**created_date_to** | Option<**String**> |  |  |
**updated_date_from** | Option<**String**> |  |  |
**updated_date_to** | Option<**String**> |  |  |
**voucher_number** | Option<**String**> |  |  |
**page** | Option<**i32**> |  |  |
**size** | Option<**i32**> |  |  |[default to 25]
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::VoucherList**](VoucherList.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vouchers_get

> crate::models::Voucher vouchers_get(voucher_number)
Filter voucher by voucherNumber

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_number** | **String** |  | [required] |

### Return type

[**crate::models::Voucher**](Voucher.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vouchers_id_get

> crate::models::Voucher vouchers_id_get(id)
Find a voucher by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::Voucher**](Voucher.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

