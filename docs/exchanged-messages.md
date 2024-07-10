# Exchanged messages format

Exchanged message format follows this spec:

- Message

## Message

|field|type|required|description|
|-----|----|--------|-----------|
| name | string | y | name of the command |
| timestamp | string | y | ISODate of cmd gen |
| payload | bytes buffer | y | message payload |
