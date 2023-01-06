

## About the protobuf

- We not only use protobuf to generate the type for gRPC API but also use it to unify the type or rest API. This act enforce our API to follow the single source of truth. 

## About the data field: kind

- Every third_party resource should have this field with one_of constraint.

## About how we handle timestamp

- We use chrono, accuracy to micros
- Internally every timestamp will be converted into i64, and converted to edgedb_protocol::model::Datetime when we need to insert it into database.
- external source 
  - source -> chrono to i64 -> edgedb_protocol::model::Datetime
- internal source
  - chrono generated i64 -> edgedb_protocol::model::Datetime
- All our exposed API will return i64 timestamp