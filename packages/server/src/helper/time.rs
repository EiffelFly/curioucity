use chrono::{DateTime, TimeZone, Utc};
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

pub fn get_edgedb_timestamp_from_string(timestamp: &String) -> Result<Datetime, String> {
    let chrono_utc: DateTime<Utc> = match timestamp.parse() {
        Ok(timestamp) => timestamp,
        Err(err) => {
            return Err(format!(
                "Something went wrong when parse timestamp to chrono DateTime: {}",
                err
            ))
        }
    };

    match Datetime::try_from(chrono_utc) {
        Ok(datetime) => return Ok(datetime),
        Err(err) => Err(format!(
            "Something went wrong when convert timestamp to edgedb Datetime: {}",
            err
        )),
    }
}

#[cfg(test)]
mod test {
    use super::get_edgedb_timestamp_from_2000;
    use super::get_edgedb_timestamp_from_string;

    #[test]
    fn get_edgedb_timestamp_from_int_test() {
        let timestamp = get_edgedb_timestamp_from_2000(1675219231000000).unwrap();
        assert_eq!(timestamp.to_string(), "2023-02-01 02:40:31 UTC".to_string())
    }

    #[test]
    fn get_edgedb_timestamp_from_string_test() {
        let timestamp =
            get_edgedb_timestamp_from_string(&"2023-02-01T02:44:12+00:00".to_string()).unwrap();
        assert_eq!(timestamp.to_string(), "2023-02-01 02:44:12 UTC".to_string())
    }
}
