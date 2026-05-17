use std::fmt;

pub trait ReportFormatter {
    fn format(&self, data: &str) -> String;
}

pub struct JsonFormatter;

impl ReportFormatter for JsonFormatter {
    fn format(&self, data: &str) -> String {
        format!("{{ \"data\": \"{data}\" }}")
    }
}

pub struct CsvWriter;

impl CsvWriter {
    pub fn write(&self, data: &str) -> Result<String, String> {
        std::fs::write("/tmp/report.csv", data).map_err(|e| e.to_string())?;
        Ok(data.to_string())
    }

    pub fn header(&self) -> String {
        "col1,col2".to_string()
    }
}

impl fmt::Display for CsvWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CsvWriter")
    }
}
