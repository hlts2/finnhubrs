# InFilingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filing_id** | Option<**String**> | Filing Id in Alpharesearch platform | [optional]
**title** | Option<**String**> | Filing title | [optional]
**filer_id** | Option<**String**> | Id of the entity submitted the filing | [optional]
**symbol** | Option<[**serde_json::Value**](.md)> | List of symbol associate with this filing | [optional]
**name** | Option<**String**> | Filer name | [optional]
**acceptance_date** | Option<**String**> | Date the filing is submitted. | [optional]
**filed_date** | Option<**String**> | Date the filing is make available to the public | [optional]
**report_date** | Option<**String**> | Date as which the filing is reported | [optional]
**form** | Option<**String**> | Filing Form | [optional]
**amend** | Option<**bool**> | Amendment | [optional]
**source** | Option<**String**> | Filing Source | [optional]
**page_count** | Option<**i32**> | Estimate number of page when printing | [optional]
**document_count** | Option<**i32**> | Number of document in this filing | [optional]
**documents** | Option<[**Vec<models::DocumentResponse>**](DocumentResponse.md)> | Document for this filing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


