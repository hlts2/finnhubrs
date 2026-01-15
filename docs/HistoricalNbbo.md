# HistoricalNbbo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**s** | Option<**String**> | Symbol. | [optional]
**skip** | Option<**i64**> | Number of ticks skipped. | [optional]
**count** | Option<**i64**> | Number of ticks returned. If <code>count</code> < <code>limit</code>, all data for that date has been returned. | [optional]
**total** | Option<**i64**> | Total number of ticks for that date. | [optional]
**av** | Option<**Vec<f32>**> | List of Ask volume data. | [optional]
**a** | Option<**Vec<f32>**> | List of Ask price data. | [optional]
**ax** | Option<**Vec<String>**> | List of venues/exchanges - Ask price. A list of exchange codes can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1Tj53M1svmr-hfEtbk6_NpVR1yAyGLMaH6ByYU6CG0ZY/edit?usp=sharing\",>here</a> | [optional]
**bv** | Option<**Vec<f32>**> | List of Bid volume data. | [optional]
**b** | Option<**Vec<f32>**> | List of Bid price data. | [optional]
**bx** | Option<**Vec<String>**> | List of venues/exchanges - Bid price. A list of exchange codes can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1Tj53M1svmr-hfEtbk6_NpVR1yAyGLMaH6ByYU6CG0ZY/edit?usp=sharing\",>here</a> | [optional]
**t** | Option<**Vec<i64>**> | List of timestamp in UNIX ms. | [optional]
**c** | Option<[**Vec<Vec<String>>**](Vec.md)> | List of quote conditions. A comprehensive list of quote conditions code can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1iiA6e7Osdtai0oPMOUzgAIKXCsay89dFDmsegz6OpEg/edit?usp=sharing\">here</a> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


