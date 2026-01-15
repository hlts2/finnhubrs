# SearchBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | **String** | Search query | 
**isins** | Option<**String**> | List of isin to search, comma separated (Max: 50). | [optional]
**cusips** | Option<**String**> | List of cusip to search, comma separated (Max: 50). | [optional]
**ciks** | Option<**String**> | List of SEC Center Index Key to search, comma separated (Max: 50). | [optional]
**sedar_ids** | Option<**String**> | List of SEDAR issuer number to search, comma separated (Max: 50). | [optional]
**ch_ids** | Option<**String**> | List of Companies House number to search, comma separated (Max: 50). | [optional]
**symbols** | Option<**String**> | List of symbols to search, comma separated (Max: 50). | [optional]
**sedols** | Option<**String**> | List of sedols to search, comma separated (Max: 50). | [optional]
**sources** | Option<**String**> | List of sources to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**forms** | Option<**String**> | List of forms to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**gics** | Option<**String**> | List of gics to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**naics** | Option<**String**> | List of sources to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**exhibits** | Option<**String**> | List of exhibits to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**exchanges** | Option<**String**> | List of exchanges to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**countries** | Option<**String**> | List of sources to search, comma separated (Max: 50). Look at <code>/filter</code> endpoint to see all available values. | [optional]
**acts** | Option<**String**> | List of SEC's exchanges act to search, comma separated. Look at <code>/filter</code> endpoint to see all available values. | [optional]
**caps** | Option<**String**> | List of market capitalization to search, comma separated. Look at <code>/filter</code> endpoint to see all available values. | [optional]
**from_date** | Option<**String**> | Search from date in format: YYYY-MM-DD, default from the last 2 years | [optional]
**to_date** | Option<**String**> | Search to date in format: YYYY-MM-DD, default to today | [optional]
**page** | Option<**String**> | Use for pagination, default to page 1 | [optional]
**sort** | Option<**String**> | Sort result by, default: sortMostRecent. Look at <code>/filter</code> endpoint to see all available values. | [optional]
**highlighted** | Option<**bool**> | Enable highlight in returned filings. If enabled, only return 10 results each time | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


