# UpgradeDowngrade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**symbol** | Option<**String**> | Company symbol. | [optional]
**grade_time** | Option<**i64**> | Upgrade/downgrade time in UNIX timestamp. | [optional]
**from_grade** | Option<**String**> | From grade. | [optional]
**to_grade** | Option<**String**> | To grade. | [optional]
**company** | Option<**String**> | Company/analyst who did the upgrade/downgrade. | [optional]
**action** | Option<**String**> | Action can take any of the following values: <code>up(upgrade), down(downgrade), main(maintains), init(initiate), reit(reiterate)</code>. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


