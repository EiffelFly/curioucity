

## About the protobuf

- We not only use protobuf to generate the type for gRPC API but also use it to unify the type or rest API. This act enforces our API to follow the single source of truth. 

## About the data field: kind

- Every third_party resource should have this field with one_of constraint.

## Parse String back to Datetime

```rust
use chrono::{DateTime, Utc};

let utc_timestamp = match full_tag_with_string_timestamp
  .created_timestamp_at_curioucity
  .parse::<DateTime<Utc>>()
{
  Ok(time) => time.timestamp(),
  Err(error) => {
      println!("Parse Timestamp Error: {}", error);
      bail!("Parse Timestamp Error: {}", error)
  }
};
```

## About the release 

We are currently using the changeset to control our versioning but not publishing. Every PR will be followed by a set of changests which only target the curioucity folder. And once the versioning PR is merged the changeset bot will change the version number of `package.json`. 

But the version of `Cargo.toml` will remain the same. Currently, we don't want to bother with the version of this cargo because we don't publish cargo on the Rust registry but only plan to publish the docker image on the docker hub.