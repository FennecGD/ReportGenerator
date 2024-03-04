pub const DEFAULT_REPORT_FILE: &str = "report.pdf";
pub const TMP_FILE: &str = "tmp.typ";

pub const MAIN_TEMPLATE: &str = include_str!("../templates/main_report.typ");
pub const T_METADATA: &str = include_str!("../templates/metadata.typ");

pub const T_SECTION: &str = include_str!("../templates/sections/default.typ");
pub const T_SCOPE: &str = include_str!("../templates/sections/scope.typ");
pub const T_SUMMARY: &str = include_str!("../templates/sections/summary.typ");
pub const T_METHODOLOGY: &str = include_str!("../templates/sections/methodology.typ");

pub const T_FINDING: &str = include_str!("../templates/findings/default.typ");
pub const T_XSS: &str = include_str!("../templates/findings/xss.typ");
pub const T_SQL_INJECTION: &str = include_str!("../templates/findings/sql-injection.typ");
