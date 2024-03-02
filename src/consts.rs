pub const DEFAULT_REPORT_FILE: &str = "report.pdf";
pub const TMP_FILE: &str = "tmp.typ";
pub const REPORT_TEMPLATE: &str = include_str!("../templates/main_report.typ");

pub const EXAMPLE_METADATA: &str = "title:Example Pentest Report
prepared_for:Example prepared for
prepared_by:Example prepared by
";

pub const EXAMPLE_SECTION: &str = "= Example section
Look at this gorgeus sections content
#lorem(200)
";

pub const EXAMPLE_FINDING: &str = "= Example finding
Look at this amazing finding
#lorem(200)
";

pub const EXAMPLE_SUMMARY: &str = "= Summary
Example summary content
#lorem(200)
";

pub const EXAMPLE_METHODOLOGY: &str = "= Methodology
Example methodology
#lorem(200)
";

pub const EXAMPLE_SCOPE: &str = "= Scope
Example scope
#lorem(200)
";
