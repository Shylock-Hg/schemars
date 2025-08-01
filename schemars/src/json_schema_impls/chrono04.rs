use crate::SchemaGenerator;
use crate::{json_schema, JsonSchema, Schema};
use alloc::borrow::Cow;
use chrono04::{prelude::*, TimeDelta};

impl JsonSchema for Weekday {
    inline_schema!();

    fn schema_name() -> Cow<'static, str> {
        "Weekday".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "chrono::Weekday".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "enum": [
                "Mon",
                "Tue",
                "Wed",
                "Thu",
                "Fri",
                "Sat",
                "Sun",
            ]
        })
    }
}

impl JsonSchema for TimeDelta {
    inline_schema!();

    fn schema_name() -> Cow<'static, str> {
        "TimeDelta".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "chrono::TimeDelta".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
          "type": "array",
          "prefixItems": [
            {
              "type": "integer",
              "format": "int64"
            },
            {
              "type": "integer",
              "minimum": 0,
              "exclusiveMaximum": 1_000_000_000
            }
          ],
          "minItems": 2,
          "maxItems": 2
        })
    }
}

macro_rules! formatted_string_impl {
    ($ty:ident, $format:literal) => {
        formatted_string_impl!($ty, $format, JsonSchema for $ty);
    };
    ($ty:ident, $format:literal, $($desc:tt)+) => {
        impl $($desc)+ {
            inline_schema!();

            fn schema_name() -> Cow<'static, str> {
                stringify!($ty).into()
            }

            fn schema_id() -> Cow<'static, str>  {
                stringify!(chrono::$ty).into()
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                json_schema!({
                    "type": "string",
                    "format": $format
                })
            }
        }
    };
}

formatted_string_impl!(NaiveDate, "date");
formatted_string_impl!(NaiveDateTime, "partial-date-time");
formatted_string_impl!(NaiveTime, "partial-time");
formatted_string_impl!(DateTime, "date-time", <Tz: TimeZone> JsonSchema for DateTime<Tz>);
