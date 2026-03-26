/// Parser for Fluentbit td-agent.conf configuration files.
///
/// Fluentbit uses an INI-like format with `[SECTION]` headers and
/// space-delimited key-value pairs (no `=` sign). Example:
///
/// ```text
/// [SERVICE]
///     Log_Level    info
///     Flush        5
///
/// [INPUT]
///     Name         tail
///     Path         /var/log/myapp/*.log
///     Tag          custom.myapp
///
/// [OUTPUT]
///     Name         tcp
///     Match        *
///     Host         127.0.0.1
///     Port         28330
/// ```

/// Parsed Fluentbit configuration.
#[derive(Debug, Clone, Default)]
pub struct FluentbitConfig {
    pub service: FluentbitService,
    pub inputs: Vec<FluentbitInput>,
    pub outputs: Vec<FluentbitOutput>,
}

/// The [SERVICE] section of Fluentbit config.
#[derive(Debug, Clone, Default)]
pub struct FluentbitService {
    pub log_level: Option<String>,
    pub flush: Option<String>,
}

/// An [INPUT] section of Fluentbit config.
#[derive(Debug, Clone, Default)]
pub struct FluentbitInput {
    pub name: Option<String>,
    pub path: Option<String>,
    pub tag: Option<String>,
}

/// An [OUTPUT] section of Fluentbit config.
#[derive(Debug, Clone, Default)]
pub struct FluentbitOutput {
    pub name: Option<String>,
    pub match_pattern: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub path: Option<String>,
}

impl FluentbitConfig {
    /// Returns true if any output targets the mdsd TCP socket (port 28330).
    pub fn has_mdsd_output(&self) -> bool {
        self.outputs
            .iter()
            .any(|o| o.port == Some(28330) || o.name.as_deref() == Some("tcp"))
    }

    /// Returns true if debug logging is enabled.
    pub fn is_debug_logging(&self) -> bool {
        self.service
            .log_level
            .as_deref()
            .map(|l| l.eq_ignore_ascii_case("debug"))
            .unwrap_or(false)
    }

    /// Returns the list of file paths being tailed by INPUT sections.
    pub fn tailed_paths(&self) -> Vec<&str> {
        self.inputs
            .iter()
            .filter(|i| i.name.as_deref() == Some("tail"))
            .filter_map(|i| i.path.as_deref())
            .collect()
    }
}

#[derive(Debug, PartialEq)]
enum SectionType {
    Service,
    Input,
    Output,
    Other,
}

/// Parse a Fluentbit td-agent.conf file into a structured config.
pub fn parse_fluentbit_config(content: &str) -> FluentbitConfig {
    let mut config = FluentbitConfig::default();
    let mut current_section = SectionType::Other;
    let mut current_input = FluentbitInput::default();
    let mut current_output = FluentbitOutput::default();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Section header
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            // Flush previous section
            flush_section(
                &current_section,
                &mut config,
                &mut current_input,
                &mut current_output,
            );

            let section_name = &trimmed[1..trimmed.len() - 1];
            current_section = match section_name.to_uppercase().as_str() {
                "SERVICE" => SectionType::Service,
                "INPUT" => SectionType::Input,
                "OUTPUT" => SectionType::Output,
                _ => SectionType::Other,
            };
            continue;
        }

        // Key-value pair (space-delimited)
        let (key, value) = match parse_kv(trimmed) {
            Some(kv) => kv,
            None => continue,
        };

        match current_section {
            SectionType::Service => match key.to_lowercase().as_str() {
                "log_level" => config.service.log_level = Some(value.to_string()),
                "flush" => config.service.flush = Some(value.to_string()),
                _ => {}
            },
            SectionType::Input => match key.to_lowercase().as_str() {
                "name" => current_input.name = Some(value.to_string()),
                "path" => current_input.path = Some(value.to_string()),
                "tag" => current_input.tag = Some(value.to_string()),
                _ => {}
            },
            SectionType::Output => match key.to_lowercase().as_str() {
                "name" => current_output.name = Some(value.to_string()),
                "match" => current_output.match_pattern = Some(value.to_string()),
                "host" => current_output.host = Some(value.to_string()),
                "port" => current_output.port = value.parse().ok(),
                "path" => current_output.path = Some(value.to_string()),
                _ => {}
            },
            SectionType::Other => {}
        }
    }

    // Flush the last section
    flush_section(
        &current_section,
        &mut config,
        &mut current_input,
        &mut current_output,
    );

    config
}

fn flush_section(
    section: &SectionType,
    config: &mut FluentbitConfig,
    input: &mut FluentbitInput,
    output: &mut FluentbitOutput,
) {
    match section {
        SectionType::Input => {
            if input.name.is_some() || input.path.is_some() {
                config.inputs.push(std::mem::take(input));
            }
        }
        SectionType::Output => {
            if output.name.is_some() {
                config.outputs.push(std::mem::take(output));
            }
        }
        _ => {}
    }
}

/// Parse a Fluentbit key-value pair (space-delimited, no `=`).
fn parse_kv(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.splitn(2, char::is_whitespace);
    let key = parts.next()?.trim();
    let value = parts.next()?.trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic_config() {
        let content = "\
[SERVICE]
    Log_Level    info
    Flush        5

[INPUT]
    Name         tail
    Path         /var/log/myapp/*.log
    Tag          custom.myapp

[OUTPUT]
    Name         tcp
    Match        *
    Host         127.0.0.1
    Port         28330
";
        let config = parse_fluentbit_config(content);
        assert_eq!(config.service.log_level.as_deref(), Some("info"));
        assert_eq!(config.service.flush.as_deref(), Some("5"));
        assert_eq!(config.inputs.len(), 1);
        assert_eq!(config.inputs[0].name.as_deref(), Some("tail"));
        assert_eq!(
            config.inputs[0].path.as_deref(),
            Some("/var/log/myapp/*.log")
        );
        assert_eq!(config.inputs[0].tag.as_deref(), Some("custom.myapp"));
        assert_eq!(config.outputs.len(), 1);
        assert_eq!(config.outputs[0].name.as_deref(), Some("tcp"));
        assert_eq!(config.outputs[0].port, Some(28330));
        assert!(config.has_mdsd_output());
        assert!(!config.is_debug_logging());
    }

    #[test]
    fn detect_debug_logging() {
        let content = "\
[SERVICE]
    Log_Level    debug
";
        let config = parse_fluentbit_config(content);
        assert!(config.is_debug_logging());
    }

    #[test]
    fn multiple_inputs_and_outputs() {
        let content = "\
[INPUT]
    Name    tail
    Path    /var/log/syslog

[INPUT]
    Name    tail
    Path    /var/log/custom.log

[OUTPUT]
    Name    tcp
    Port    28330

[OUTPUT]
    Name    file
    Path    /tmp/debug-out.log
";
        let config = parse_fluentbit_config(content);
        assert_eq!(config.inputs.len(), 2);
        assert_eq!(config.outputs.len(), 2);
        let paths = config.tailed_paths();
        assert_eq!(paths, vec!["/var/log/syslog", "/var/log/custom.log"]);
    }

    #[test]
    fn no_mdsd_output() {
        let content = "\
[OUTPUT]
    Name    file
    Path    /tmp/out.log
";
        let config = parse_fluentbit_config(content);
        assert!(!config.has_mdsd_output());
    }

    #[test]
    fn empty_config() {
        let config = parse_fluentbit_config("");
        assert!(config.inputs.is_empty());
        assert!(config.outputs.is_empty());
        assert!(config.service.log_level.is_none());
    }

    #[test]
    fn comments_and_blank_lines() {
        let content = "\
# This is a comment
[SERVICE]
    # Another comment
    Log_Level    warn

";
        let config = parse_fluentbit_config(content);
        assert_eq!(config.service.log_level.as_deref(), Some("warn"));
    }
}
