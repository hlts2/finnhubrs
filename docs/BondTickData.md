# BondTickData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**skip** | Option<**i64**> | Number of ticks skipped. | [optional]
**count** | Option<**i64**> | Number of ticks returned. If <code>count</code> < <code>limit</code>, all data for that date has been returned. | [optional]
**total** | Option<**i64**> | Total number of ticks for that date. | [optional]
**v** | Option<**Vec<f32>**> | List of volume data. | [optional]
**p** | Option<**Vec<f32>**> | List of price data. | [optional]
**y** | Option<**Vec<f32>**> | List of yield data. | [optional]
**t** | Option<**Vec<i64>**> | List of timestamp in UNIX ms. | [optional]
**si** | Option<**Vec<String>**> | List of values showing the side (Buy/sell) of each trade. List of supported values: <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1O3aueXSPOqo7Iuyz4PqDG6yZunHsX8BTefZ2kFk5pz4/edit?usp=sharing\",>here</a> | [optional]
**cp** | Option<**Vec<String>**> | List of values showing the counterparty of each trade. List of supported values: <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1O3aueXSPOqo7Iuyz4PqDG6yZunHsX8BTefZ2kFk5pz4/edit?usp=sharing\",>here</a> | [optional]
**rp** | Option<**Vec<String>**> | List of values showing the reporting party of each trade. List of supported values: <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1O3aueXSPOqo7Iuyz4PqDG6yZunHsX8BTefZ2kFk5pz4/edit?usp=sharing\",>here</a> | [optional]
**ats** | Option<**Vec<String>**> | ATS flag. Y or empty | [optional]
**c** | Option<[**Vec<Vec<String>>**](Vec.md)> | List of trade conditions. A comprehensive list of trade conditions code can be found <a target=\"_blank\" href=\"https://docs.google.com/spreadsheets/d/1O3aueXSPOqo7Iuyz4PqDG6yZunHsX8BTefZ2kFk5pz4/edit?usp=sharing\">here</a> | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


