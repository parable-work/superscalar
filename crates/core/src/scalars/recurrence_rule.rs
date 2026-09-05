use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use chrono::{NaiveDate, NaiveDateTime};

// RFC 5545 section 3.3.10 RECUR, WITHOUT the DTSTART that a calendar object
// would carry beside it.
//
// The omission is the whole reason this scalar exists rather than a plain
// string. A rule says "every second Tuesday at 09:00"; it does not say which
// 09:00. In an iCalendar file DTSTART answers that, carrying both the anchor
// instant and the zone. The schema this scalar was written for stores the three
// separately -- a recurrence column, an anchor instant and a timezone -- because
// the anchor and the zone are edited by different controls and a zone change
// must not rewrite the rule.
// So a rule containing DTSTART is rejected: it would be a second answer to a
// question two other columns already answer.
//
// THIS SCALAR DOES NOT COMPUTE OCCURRENCES, deliberately. Expansion needs a
// calendar and the IANA zone database, and it is not a scalar operation: the C
// ABI here is ten generic functions keyed on ScalarId, and a
// (rule, anchor, zone, n) -> [instant] call fits none of them. Exactly one
// engine expands, in Go, server-side. That is what stops a browser preview and a
// scheduler from disagreeing about a DST boundary -- not two implementations
// agreeing, but only one existing.

const MAX_LENGTH: usize = 512;

// RFC 5545 order. Normalization emits parts in this sequence so that two rules
// meaning the same thing are the same string -- which is what makes a Plan whose
// recurrence was retyped identically read as unchanged by the save path.
const PART_ORDER: &[&str] = &[
    "FREQ",
    "UNTIL",
    "COUNT",
    "INTERVAL",
    "BYSECOND",
    "BYMINUTE",
    "BYHOUR",
    "BYDAY",
    "BYMONTHDAY",
    "BYYEARDAY",
    "BYWEEKNO",
    "BYMONTH",
    "BYSETPOS",
    "WKST",
];

const FREQUENCIES: &[&str] = &[
    "SECONDLY", "MINUTELY", "HOURLY", "DAILY", "WEEKLY", "MONTHLY", "YEARLY",
];

// Index order is RFC 5545's, and is also the order BYDAY sorts into, so a
// weekday picker's chips normalize to calendar order however they were clicked.
const WEEKDAYS: &[&str] = &["SU", "MO", "TU", "WE", "TH", "FR", "SA"];

fn weekday_index(code: &str) -> Option<usize> {
    WEEKDAYS.iter().position(|day| *day == code)
}

// A signed list entry, e.g. BYMONTHDAY=-1 or BYSETPOS=2. `zero_allowed` is false
// everywhere RFC 5545 uses a signed list: 0 is never a valid ordinal because the
// positions are 1-based from either end.
fn parse_signed_list(value: &str, part: &str, magnitude: i32) -> Result<Vec<i32>, ScalarError> {
    let mut parsed = Vec::new();
    for item in value.split(',') {
        let number: i32 = item.parse().map_err(|_| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("{part} takes whole numbers, got {item}"),
            )
        })?;
        if number == 0 || number.abs() > magnitude {
            return Err(ScalarError::new(
                ErrorKind::Range,
                format!("{part} takes 1..={magnitude} or -1..=-{magnitude}, got {number}"),
            ));
        }
        parsed.push(number);
    }
    Ok(parsed)
}

// An unsigned list bounded at both ends, e.g. BYHOUR=0..23.
fn parse_unsigned_list(
    value: &str,
    part: &str,
    low: u32,
    high: u32,
) -> Result<Vec<u32>, ScalarError> {
    let mut parsed = Vec::new();
    for item in value.split(',') {
        let number: u32 = item.parse().map_err(|_| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("{part} takes whole numbers, got {item}"),
            )
        })?;
        if number < low || number > high {
            return Err(ScalarError::new(
                ErrorKind::Range,
                format!("{part} takes {low}..={high}, got {number}"),
            ));
        }
        parsed.push(number);
    }
    Ok(parsed)
}

// BYDAY is the one list carrying two values per entry: an optional signed
// ordinal and a weekday, e.g. "-1FR" for the last Friday. Sorted by weekday then
// ordinal so the canonical form is stable.
fn normalize_byday(value: &str) -> Result<String, ScalarError> {
    let mut parsed: Vec<(usize, i32, String)> = Vec::new();
    for item in value.split(',') {
        if item.len() < 2 {
            return Err(ScalarError::new(
                ErrorKind::Parse,
                format!("BYDAY entry is too short: {item}"),
            ));
        }
        let split = item.len() - 2;
        let (ordinal_text, weekday) = item.split_at(split);
        let index = weekday_index(weekday).ok_or_else(|| {
            ScalarError::new(
                ErrorKind::Enum,
                format!(
                    "BYDAY weekday must be one of {}, got {weekday}",
                    WEEKDAYS.join(", ")
                ),
            )
        })?;
        let ordinal = if ordinal_text.is_empty() {
            0
        } else {
            let number: i32 = ordinal_text.parse().map_err(|_| {
                ScalarError::new(
                    ErrorKind::Parse,
                    format!("BYDAY ordinal must be a whole number, got {ordinal_text}"),
                )
            })?;
            if number == 0 || number.abs() > 53 {
                return Err(ScalarError::new(
                    ErrorKind::Range,
                    format!("BYDAY ordinal takes 1..=53 or -1..=-53, got {number}"),
                ));
            }
            number
        };
        parsed.push((index, ordinal, weekday.to_string()));
    }
    parsed.sort_unstable_by_key(|(index, ordinal, _)| (*index, *ordinal));
    parsed.dedup();
    let rendered: Vec<String> = parsed
        .into_iter()
        .map(|(_, ordinal, weekday)| {
            if ordinal == 0 {
                weekday
            } else {
                format!("{ordinal}{weekday}")
            }
        })
        .collect();
    Ok(rendered.join(","))
}

// UNTIL is a DATE or a DATE-TIME. RFC 5545 requires the UTC form whenever
// DTSTART is zoned, and a Plan's anchor always carries a zone, so the bare local
// forms are accepted on the way in and stamped with Z on the way out rather than
// being silently treated as some other zone.
fn normalize_until(value: &str) -> Result<String, ScalarError> {
    let trimmed = value.trim_end_matches('Z');
    if NaiveDateTime::parse_from_str(trimmed, "%Y%m%dT%H%M%S").is_ok() {
        return Ok(format!("{trimmed}Z"));
    }
    if NaiveDate::parse_from_str(trimmed, "%Y%m%d").is_ok() {
        return Ok(trimmed.to_string());
    }
    Err(ScalarError::new(
        ErrorKind::Parse,
        format!("UNTIL takes YYYYMMDD or YYYYMMDDTHHMMSSZ, got {value}"),
    ))
}

fn sorted_signed(value: &str, part: &str, magnitude: i32) -> Result<String, ScalarError> {
    let mut numbers = parse_signed_list(value, part, magnitude)?;
    numbers.sort_unstable();
    numbers.dedup();
    Ok(numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

fn sorted_unsigned(value: &str, part: &str, low: u32, high: u32) -> Result<String, ScalarError> {
    let mut numbers = parse_unsigned_list(value, part, low, high)?;
    numbers.sort_unstable();
    numbers.dedup();
    Ok(numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

fn normalize_recurrence_rule(input: &str) -> Result<String, ScalarError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ScalarError::new(
            ErrorKind::Empty,
            "a recurrence rule cannot be empty".to_string(),
        ));
    }
    if trimmed.len() > MAX_LENGTH {
        return Err(ScalarError::new(
            ErrorKind::Length,
            format!("a recurrence rule is at most {MAX_LENGTH} characters"),
        ));
    }

    // An RRULE: prefix is what a calendar file writes around the value. Accepted
    // on the way in because that is what people paste, dropped on the way out
    // because the column stores the rule, not a line of iCalendar.
    let body = trimmed
        .strip_prefix("RRULE:")
        .or_else(|| trimmed.strip_prefix("rrule:"))
        .unwrap_or(trimmed);

    let mut seen: Vec<(String, String)> = Vec::new();
    for part in body.split(';') {
        if part.is_empty() {
            continue;
        }
        let (name, value) = part.split_once('=').ok_or_else(|| {
            ScalarError::new(
                ErrorKind::Parse,
                format!("every rule part is NAME=VALUE, got {part}"),
            )
        })?;
        let name = name.trim().to_ascii_uppercase();
        let value = value.trim().to_ascii_uppercase();
        if value.is_empty() {
            return Err(ScalarError::new(
                ErrorKind::Parse,
                format!("{name} has no value"),
            ));
        }
        // DTSTART is checked HERE, ahead of the unknown-part error, because it
        // is the one rejection a person needs an explanation for rather than a
        // name. It is a legitimate RFC 5545 property, just not one this column
        // owns: the anchor instant and the zone are their own fields on the
        // owning row, and a rule carrying them could disagree with them.
        if name == "DTSTART" {
            return Err(ScalarError::new(
                ErrorKind::Custom,
                "DTSTART belongs on the owning row's anchor and timezone, not in the rule"
                    .to_string(),
            ));
        }
        if !PART_ORDER.contains(&name.as_str()) {
            return Err(ScalarError::new(
                ErrorKind::Enum,
                format!("unknown rule part {name}"),
            ));
        }
        if seen.iter().any(|(existing, _)| *existing == name) {
            return Err(ScalarError::new(
                ErrorKind::Custom,
                format!("{name} appears more than once"),
            ));
        }

        let normalized = match name.as_str() {
            "FREQ" => {
                if !FREQUENCIES.contains(&value.as_str()) {
                    return Err(ScalarError::new(
                        ErrorKind::Enum,
                        format!(
                            "FREQ must be one of {}, got {value}",
                            FREQUENCIES.join(", ")
                        ),
                    ));
                }
                value
            }
            "UNTIL" => normalize_until(&value)?,
            "COUNT" | "INTERVAL" => {
                let number: u32 = value.parse().map_err(|_| {
                    ScalarError::new(
                        ErrorKind::Parse,
                        format!("{name} takes a whole number, got {value}"),
                    )
                })?;
                if number == 0 {
                    return Err(ScalarError::new(
                        ErrorKind::Range,
                        format!("{name} must be 1 or more"),
                    ));
                }
                number.to_string()
            }
            "BYSECOND" => sorted_unsigned(&value, &name, 0, 60)?,
            "BYMINUTE" => sorted_unsigned(&value, &name, 0, 59)?,
            "BYHOUR" => sorted_unsigned(&value, &name, 0, 23)?,
            "BYMONTH" => sorted_unsigned(&value, &name, 1, 12)?,
            "BYDAY" => normalize_byday(&value)?,
            "BYMONTHDAY" => sorted_signed(&value, &name, 31)?,
            "BYYEARDAY" => sorted_signed(&value, &name, 366)?,
            "BYWEEKNO" => sorted_signed(&value, &name, 53)?,
            "BYSETPOS" => sorted_signed(&value, &name, 366)?,
            "WKST" => {
                if weekday_index(&value).is_none() {
                    return Err(ScalarError::new(
                        ErrorKind::Enum,
                        format!("WKST must be one of {}, got {value}", WEEKDAYS.join(", ")),
                    ));
                }
                value
            }
            _ => unreachable!("PART_ORDER membership was checked above"),
        };
        seen.push((name, normalized));
    }

    let has = |part: &str| seen.iter().any(|(name, _)| name == part);

    if !has("FREQ") {
        return Err(ScalarError::new(
            ErrorKind::Custom,
            "a recurrence rule must specify FREQ".to_string(),
        ));
    }
    // RFC 5545: "UNTIL and COUNT MUST NOT occur in the same 'recur'." A rule
    // carrying both has two different end conditions and no way to choose.
    if has("UNTIL") && has("COUNT") {
        return Err(ScalarError::new(
            ErrorKind::Custom,
            "UNTIL and COUNT cannot both be set".to_string(),
        ));
    }
    // RFC 5545: BYSETPOS selects from the set another BYxxx rule produced, so
    // alone it has nothing to select from.
    if has("BYSETPOS")
        && !seen
            .iter()
            .any(|(name, _)| name.starts_with("BY") && name != "BYSETPOS")
    {
        return Err(ScalarError::new(
            ErrorKind::Custom,
            "BYSETPOS needs another BY rule to select from".to_string(),
        ));
    }
    let mut ordered = Vec::with_capacity(seen.len());
    for part in PART_ORDER {
        if let Some((name, value)) = seen.iter().find(|(name, _)| name == part) {
            ordered.push(format!("{name}={value}"));
        }
    }
    Ok(ordered.join(";"))
}

pub struct TemporalRecurrenceRule;

impl Scalar for TemporalRecurrenceRule {
    fn id(&self) -> ScalarId {
        ScalarId::TEMPORAL_RECURRENCE_RULE
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_recurrence_rule(input)
    }

    // Normalizing a rule requires understanding its parts, so there is nothing
    // this can do that parse does not already do. Same shape as Temporal.Date.
    fn normalize(&self, registry: &Registry, input: &str) -> Result<String, ScalarError> {
        self.parse(registry, input)
    }

    fn validate(&self, registry: &Registry, input: &str) -> Result<(), ScalarError> {
        self.parse(registry, input).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_recurrence_rule as parse;

    #[test]
    fn accepts_the_three_cadences_the_ui_ships() {
        assert_eq!(
            parse("FREQ=DAILY;BYHOUR=9;BYMINUTE=0").unwrap(),
            "FREQ=DAILY;BYMINUTE=0;BYHOUR=9"
        );
        assert_eq!(
            parse("FREQ=WEEKLY;BYDAY=MO,WE,FR;BYHOUR=9;BYMINUTE=0").unwrap(),
            "FREQ=WEEKLY;BYMINUTE=0;BYHOUR=9;BYDAY=MO,WE,FR"
        );
        assert_eq!(
            parse("FREQ=MONTHLY;BYMONTHDAY=1;BYHOUR=9;BYMINUTE=0").unwrap(),
            "FREQ=MONTHLY;BYMINUTE=0;BYHOUR=9;BYMONTHDAY=1"
        );
    }

    #[test]
    fn expresses_what_cron_could_not() {
        // The three cases that motivated choosing RRULE over Temporal.CronExpression.
        assert!(parse("FREQ=WEEKLY;INTERVAL=2;BYDAY=TU").is_ok());
        assert!(parse("FREQ=MONTHLY;BYMONTHDAY=-1").is_ok());
        assert!(parse("FREQ=MONTHLY;BYDAY=2TU").is_ok());
    }

    #[test]
    fn normalization_is_canonical_so_a_reorder_is_not_a_change() {
        // The save path treats an overlay row as unchanged only when its content
        // is identical, so clicking weekday chips in a different order must not
        // read as an edit.
        let clicked_one_way = parse("FREQ=WEEKLY;BYDAY=FR,MO,WE").unwrap();
        let clicked_another = parse("FREQ=WEEKLY;BYDAY=WE,FR,MO").unwrap();
        assert_eq!(clicked_one_way, clicked_another);
        assert_eq!(clicked_one_way, "FREQ=WEEKLY;BYDAY=MO,WE,FR");
    }

    #[test]
    fn normalization_is_idempotent() {
        let once = parse("freq=weekly;byday=fr,mo;byhour=9").unwrap();
        assert_eq!(parse(&once).unwrap(), once);
    }

    #[test]
    fn accepts_and_strips_the_calendar_prefix() {
        assert_eq!(parse("RRULE:FREQ=DAILY").unwrap(), "FREQ=DAILY");
    }

    #[test]
    fn rejects_dtstart_because_two_other_columns_own_it() {
        let err = parse("DTSTART=20260101T090000Z;FREQ=DAILY").unwrap_err();
        // Assert the GUIDANCE, not the word: "unknown rule part DTSTART" also
        // contains "DTSTART", so a substring check on the name alone passes
        // even when this rejection has fallen through to the generic
        // unknown-part arm and the explanation has been lost.
        assert!(
            err.to_string().contains("anchor and timezone"),
            "DTSTART must be refused with its own explanation, got: {err}"
        );
    }

    #[test]
    fn rejects_the_rfc_contradictions() {
        assert!(parse("FREQ=DAILY;UNTIL=20260101;COUNT=5").is_err());
        assert!(parse("BYDAY=MO").is_err(), "FREQ is required");
        assert!(
            parse("FREQ=MONTHLY;BYSETPOS=2").is_err(),
            "BYSETPOS needs a BY rule"
        );
        assert!(parse("FREQ=MONTHLY;BYSETPOS=2;BYDAY=TU").is_ok());
    }

    #[test]
    fn rejects_out_of_range_and_zero_ordinals() {
        assert!(parse("FREQ=DAILY;BYHOUR=24").is_err());
        assert!(parse("FREQ=MONTHLY;BYMONTHDAY=0").is_err());
        assert!(parse("FREQ=MONTHLY;BYMONTHDAY=32").is_err());
        assert!(parse("FREQ=YEARLY;BYMONTH=13").is_err());
        assert!(parse("FREQ=DAILY;INTERVAL=0").is_err());
        assert!(parse("FREQ=WEEKLY;BYDAY=0MO").is_err());
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
        assert!(parse("FREQ").is_err(), "not NAME=VALUE");
        assert!(parse("FREQ=").is_err(), "empty value");
        assert!(parse("FREQ=FORTNIGHTLY").is_err());
        assert!(parse("FREQ=DAILY;NOPE=1").is_err());
        assert!(parse("FREQ=DAILY;FREQ=WEEKLY").is_err(), "duplicate part");
        assert!(parse("FREQ=WEEKLY;BYDAY=XX").is_err());
        assert!(parse(&format!("FREQ=DAILY;BYSECOND={}", "1,".repeat(400))).is_err());
    }

    #[test]
    fn stamps_until_as_utc_and_keeps_date_only_bare() {
        assert_eq!(
            parse("FREQ=DAILY;UNTIL=20261231T235959").unwrap(),
            "FREQ=DAILY;UNTIL=20261231T235959Z"
        );
        assert_eq!(
            parse("FREQ=DAILY;UNTIL=20261231").unwrap(),
            "FREQ=DAILY;UNTIL=20261231"
        );
        assert!(parse("FREQ=DAILY;UNTIL=2026-12-31").is_err());
    }
}
