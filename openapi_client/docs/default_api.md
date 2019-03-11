# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**api**](default_api.md#api) | **GET** /mbus/api | 
**get**](default_api.md#get) | **POST** /mbus/get/{device}/{baudrate}/{address} | 
**hat**](default_api.md#hat) | **GET** /mbus/hat | 
**hatOff**](default_api.md#hatOff) | **POST** /mbus/hat/off | 
**hatOn**](default_api.md#hatOn) | **POST** /mbus/hat/on | 
**scan**](default_api.md#scan) | **POST** /mbus/scan/{device}/{baudrate} | 


# **api**
> String api()


Returns this API specification

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, text/x-yaml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get**
> String get(device, baudrate, address)


Gets data from the slave identified by {address}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **device** | **String**| The serial device to scan - /dev/ is pre-pended to {device} by M-Bus HTTPD before scanning | 
  **baudrate** | [****](.md)| Baudrate to communicate with M-Bus devices | 
  **address** | **i32**| The slave device to get data from | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, text/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **hat**
> models::Hat hat()


Gets Raspberry Pi Hat information

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::Hat**](hat.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/plain, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **hatOff**
> hatOff()


Turns off power to the M-Bus

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **hatOn**
> hatOn()


Turns on power to the M-Bus

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **scan**
> String scan(device, baudrate)


Scan the specified device for slaves

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **device** | **String**| The serial device to scan - /dev/ is pre-pended to {device} by M-Bus HTTPD before scanning | 
  **baudrate** | [****](.md)| Baudrate to communicate with M-Bus devices | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plaintext/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

