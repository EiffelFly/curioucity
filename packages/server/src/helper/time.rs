use chrono::{TimeZone, Utc};
use edgedb_protocol::model::Datetime;

/**
 * Right now the edgedb-rust get the timestamp micros from 2000 (as Postgres)
 * But some of our timestamp is coming from the unix epoch timestamp (1970).
 * This function will get the current timestamp
 *
 */
pub fn get_edgedb_timestamp_from_2000(timestamp: i64) -> Result<Datetime, String> {
    let epoch_micros = Utc
        .with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
        .unwrap()
        .timestamp_micros();

    match Datetime::try_from_micros(timestamp - epoch_micros) {
        Ok(converted_time) => return Ok(converted_time),
        Err(err) => Err(format!(
            "Something went wrong when convert timestamp to edgedb DateTime: {}",
            err
        )),
    }
}
