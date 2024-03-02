use std::{fs::read_to_string, path::PathBuf};

pub struct Template {
    template: String,
}

impl Template {
    #[allow(dead_code)]
    pub fn new(file: PathBuf) -> Self {
        Self {
            template: read_to_string(file).expect("Failed to read template file"),
        }
    }

    pub fn from_str(template: &str) -> Self {
        Self {
            template: template.to_string(),
        }
    }

    pub fn render(&self, context: &Vec<(&str, &str)>) -> String {
        let mut report = self.template.clone();
        for element in context {
            report = report.replace(&format!("{{{{ {} }}}}", element.0), element.1);
        }
        report.to_string()
    }
}
