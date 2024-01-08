# \PaymentsApi

All URIs are relative to *https://api.lexoffice.io/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_conditions_get**](PaymentsApi.md#payment_conditions_get) | **GET** /payment-conditions | Retrieve payment conditions configured in lexoffice
[**payments_voucher_id_get**](PaymentsApi.md#payments_voucher_id_get) | **GET** /payments/{voucherId} | Find a payment by Voucher ID



## payment_conditions_get

> Vec<crate::models::PostingCategory> payment_conditions_get()
Retrieve payment conditions configured in lexoffice

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PostingCategory>**](PostingCategory.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_voucher_id_get

> crate::models::Payment payments_voucher_id_get(voucher_id)
Find a payment by Voucher ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**voucher_id** | **String** |  | [required] |

### Return type

[**crate::models::Payment**](Payment.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

