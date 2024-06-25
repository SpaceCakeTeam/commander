# Exchanged messages format

Exchanged message format follows this spec:

- Command
- DataRecord

## Command

|field|type|required|description|
|-----|----|--------|-----------|
| name | string | y | name of the command |
| timestamp | string | y | ISODate of cmd gen |
| payload | any | y | command payload |

## DataRecord

|field|type|required|description|
|-----|----|--------|-----------|
| name | string | y | name of the record datatype  |
| timestamp | string | y | ISODate of record gen |
| payload | buffer | y | command payload |
