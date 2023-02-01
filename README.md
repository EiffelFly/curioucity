# Curioucity

Knowledge management tool built for community and its builder.

## Philosophy

- [On a way to solve clustered context, my thinking process](https://www.summerbud.org/thoughts/my-thinking-process-of-solving-clustered-context#user-content-fnref-3)
- [Why is the documentation of tech products so hard to use? (In the user’s point of view)](https://www.summerbud.org/thoughts/why-is-the-documentation-of-tech-products-so-hard-to-use)


## Note

### Parse String back to Datetime

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