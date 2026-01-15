# StockSymbol

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Symbol description | [optional]
**display_symbol** | Option<**String**> | Display symbol name. | [optional]
**symbol** | Option<**String**> | Unique symbol used to identify this symbol used in <code>/stock/candle</code> endpoint. | [optional]
**r#type** | Option<**String**> | Security type. | [optional]
**mic** | Option<**String**> | Primary exchange's MIC. | [optional]
**figi** | Option<**String**> | FIGI identifier. | [optional]
**share_class_figi** | Option<**String**> | Global Share Class FIGI. | [optional]
**currency** | Option<**String**> | Price's currency. This might be different from the reporting currency of fundamental data. | [optional]
**symbol2** | Option<**String**> | Alternative ticker for exchanges with multiple tickers for 1 stock such as BSE. | [optional]
**isin** | Option<**String**> | ISIN. This field is only available for EU stocks and selected Asian markets. Entitlement from Finnhub is required to access this field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


