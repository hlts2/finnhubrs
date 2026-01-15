# Transactions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Symbol. | [optional]
**name** | Option<**String**> | Insider's name. | [optional]
**share** | Option<**i64**> | Number of shares held after the transaction. | [optional]
**change** | Option<**i64**> | Number of share changed from the last period. A positive value suggests a <code>BUY</code> transaction. A negative value suggests a <code>SELL</code> transaction. | [optional]
**filing_date** | Option<[**String**](string.md)> | Filing date. | [optional]
**transaction_date** | Option<[**String**](string.md)> | Transaction date. | [optional]
**transaction_price** | Option<**f32**> | Average transaction price. | [optional]
**transaction_code** | Option<**String**> | Transaction code. A list of codes and their meanings can be found <a href=\"https://www.sec.gov/about/forms/form4data.pdf\" target=\"_blank\" rel=\"noopener\">here</a>. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


