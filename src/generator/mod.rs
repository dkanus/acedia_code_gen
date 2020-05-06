use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub mod config;
pub use config::Config;

const UPPERCASE_MAPPING_INDEX: usize = 12;
const LOWERCASE_MAPPING_INDEX: usize = 13;
const TITLECASE_MAPPING_INDEX: usize = 14;
const UC_OUTPUT_NAME: &str = "UnicodeData.uc";

struct CodePointMapping {
    from: i32,
    to: i32,
}

struct CaseMappings {
    to_lower: Vec<CodePointMapping>,
    to_upper: Vec<CodePointMapping>,
    to_title: Vec<CodePointMapping>,
}

impl CaseMappings {
    fn new() -> CaseMappings {
        CaseMappings {
            to_lower: Vec::new(),
            to_upper: Vec::new(),
            to_title: Vec::new(),
        }
    }

    fn read_mappings(&mut self, record: csv::StringRecord) -> Result<(), Box<dyn Error>> {
        let code_point = i32::from_str_radix(&record[0], 16)?;
        if !record[UPPERCASE_MAPPING_INDEX].is_empty() {
            let code_point_image = i32::from_str_radix(&record[UPPERCASE_MAPPING_INDEX], 16)?;
            self.to_upper.push(CodePointMapping {
                from: code_point,
                to: code_point_image,
            });
        }
        if !record[LOWERCASE_MAPPING_INDEX].is_empty() {
            let code_point_image = i32::from_str_radix(&record[LOWERCASE_MAPPING_INDEX], 16)?;
            self.to_lower.push(CodePointMapping {
                from: code_point,
                to: code_point_image,
            });
        }
        if !record[TITLECASE_MAPPING_INDEX].is_empty() {
            let code_point_image = i32::from_str_radix(&record[TITLECASE_MAPPING_INDEX], 16)?;
            self.to_title.push(CodePointMapping {
                from: code_point,
                to: code_point_image,
            });
        }
        Ok(())
    }
}

fn get_data_reader(config: &Config) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let data_file = File::open(&config.unicode_data_path)?;
    Ok(csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_reader(data_file))
}

fn generate_uc(config: &Config, mappings: &CaseMappings) -> Result<(), Box<dyn Error>> {
    let template_path = &config.template_path;
    let uc_path = env::current_dir()?;
    let uc_path = uc_path.join(&Path::new(UC_OUTPUT_NAME));
    fs::copy(template_path, &uc_path)?;

    let uc_file = OpenOptions::new().append(true).open(uc_path)?;
    writeln!(&uc_file, "defaultproperties")?;
    writeln!(&uc_file, "{{")?;
    for (i, mapping) in mappings.to_lower.iter().enumerate() {
        writeln!(
            &uc_file,
            "    to_lower({})=(from=0x{:x},to=0x{:x})",
            i, mapping.from, mapping.to
        )?;
    }
    for (i, mapping) in mappings.to_upper.iter().enumerate() {
        writeln!(
            &uc_file,
            "    to_upper({})=(from=0x{:x},to=0x{:x})",
            i, mapping.from, mapping.to
        )?;
    }
    for (i, mapping) in mappings.to_title.iter().enumerate() {
        writeln!(
            &uc_file,
            "    to_title({})=(from=0x{:x},to=0x{:x})",
            i, mapping.from, mapping.to
        )?;
    }
    writeln!(&uc_file, "}}")?;
    Ok(())
}

pub fn run(config: &Config) {
    let mut data_reader = match get_data_reader(config) {
        Ok(reader) => reader,
        Err(_) => panic!("Cannot open file with unicode data"),
    };
    let mut mappings = CaseMappings::new();
    for result in data_reader.records() {
        let record;
        match result {
            Ok(parsed_record) => record = parsed_record,
            Err(_) => panic!("Provided Unicode data file has incorrect csv format"),
        }
        if let Err(_) = mappings.read_mappings(record) {
            panic!("Provided Unicode data file has incorrect csv format");
        }
    }
    if let Err(_) = generate_uc(config, &mappings) {
        panic!("Issues with writing into file \"UnicodeData.uc\"");
    }
}
