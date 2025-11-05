use crate::server::tools::meta::time::*;
use crate::server::tools::meta::EmptyParams;
use crate::server::Server;
use chrono::{DateTime, Local, Timelike};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::{tool, tool_router, Json};

#[tool_router(router = tool_touter_time, vis = "pub")]
impl Server {
    #[tool(
        name = "time.datetime_now",
        description = "Get the current datetime with timestamp"
    )]
    fn datetime_now(&self, _: Parameters<EmptyParams>) -> Json<DatetimeNowResult> {
        let local = Local::now().with_nanosecond(0).unwrap();
        let datetime = local.to_rfc3339();
        let timestamp = local.timestamp_millis().to_string();
        let result = DatetimeNowResult::new(datetime, timestamp);
        Json(result)
    }

    #[tool(
        name = "time.timestamp_to_datetime",
        description = "Convert a timestamp to a datetime"
    )]
    fn timestamp_to_datetime(
        &self,
        Parameters(params): Parameters<TimestampToDatetimeParams>,
    ) -> Result<String, String> {
        if let Some(timestamp) = params.timestamp_millis()
            && let Some(datetime) = DateTime::from_timestamp_millis(timestamp)
        {
            Ok(datetime.with_timezone(&Local).to_rfc3339())
        } else {
            Err("Invalid timestamp".to_string())
        }
    }

    #[tool(
        name = "time.datetime_to_timestamp",
        description = "Convert a datetime to a timestamp"
    )]
    fn datetime_to_timestamp(
        &self,
        Parameters(params): Parameters<DatetimeToTimestampParams>,
    ) -> Result<String, String> {
        if params.is_valid() {
            match DateTime::parse_from_rfc3339(&params.rfc3339()) {
                Ok(datetime) => Ok(datetime.timestamp_millis().to_string()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Invalid datetime".to_string())
        }
    }
}
