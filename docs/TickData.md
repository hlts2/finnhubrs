# TickData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**s** | Option<**String**> | Symbol. | [optional]
**skip** | Option<**i64**> | Number of ticks skipped. | [optional]
**count** | Option<**i64**> | Number of ticks returned. If <code>count</code> < <code>limit</code>, all data for that date has been returned. | [optional]
**total** | Option<**i64**> | Total number of ticks for that date. | [optional]
**v** | Option<**Vec<f32>**> | List of volume data. | [optional]
**p** | Option<**Vec<f32>**> | List of price data. | [optional]
**t** | Option<**Vec<i64>**> | List of timestamp in UNIX ms. | [optional]
**x** | Option<**Vec<String>**> | List of venues/exchanges. A list of exchange codes can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1Tj53M1svmr-hfEtbk6_NpVR1yAyGLMaH6ByYU6CG0ZY/edit?usp=sharing\",>here</a> | [optional]
**c** | Option<[**Vec<Vec<String>>**](Vec.md)> | List of trade conditions. A comprehensive list of trade conditions code can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1PUxiSWPHSODbaTaoL2Vef6DgU-yFtlRGZf19oBb9Hp0/edit?usp=sharing\">here</a> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


