use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Serialize;

/// Parsed mcsconfig XML data.
#[derive(Debug, Clone, Default, Serialize)]
pub struct McsConfig {
    pub file_name: String,
    pub counter_sets: Vec<CounterSet>,
    pub subscriptions: Vec<Subscription>,
    pub management_endpoints: Vec<String>,
    pub config_chunks: Vec<String>,
    pub raw_element_count: usize,
}

/// Performance counter set from mcsconfig.
#[derive(Debug, Clone, Serialize)]
pub struct CounterSet {
    pub namespace: Option<String>,
    pub counters: Vec<String>,
    pub sample_rate: Option<u32>,
}

/// Windows Event Log subscription from mcsconfig.
#[derive(Debug, Clone, Serialize)]
pub struct Subscription {
    pub name: Option<String>,
    pub query: Option<String>,
}

/// Parse an mcsconfig XML file content into structured data.
pub fn parse_mcsconfig(xml_content: &str) -> anyhow::Result<McsConfig> {
    let mut config = McsConfig::default();
    let mut reader = Reader::from_str(xml_content);
    reader.config_mut().trim_text(true);

    let mut current_element = String::new();
    let mut in_counter_set = false;
    let mut current_counter_set = CounterSet {
        namespace: None,
        counters: Vec::new(),
        sample_rate: None,
    };
    let mut in_subscription = false;
    let mut current_subscription = Subscription {
        name: None,
        query: None,
    };

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                config.raw_element_count += 1;
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_element = tag.clone();

                match tag.as_str() {
                    "CounterSet" => {
                        in_counter_set = true;
                        current_counter_set = CounterSet {
                            namespace: None,
                            counters: Vec::new(),
                            sample_rate: None,
                        };
                        for attr in e.attributes().flatten() {
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            match key.as_str() {
                                "namespace" | "eventName" => {
                                    current_counter_set.namespace = Some(val);
                                }
                                "sampleRateInSeconds" | "sampleRate" => {
                                    current_counter_set.sample_rate = val.parse().ok();
                                }
                                _ => {}
                            }
                        }
                    }
                    "Subscription" => {
                        in_subscription = true;
                        current_subscription = Subscription {
                            name: None,
                            query: None,
                        };
                        for attr in e.attributes().flatten() {
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let val = String::from_utf8_lossy(&attr.value).to_string();
                            if key == "name" || key == "eventName" {
                                current_subscription.name = Some(val);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if in_counter_set && current_element == "Counter" {
                    current_counter_set.counters.push(text.clone());
                }
                if in_subscription && current_element == "Query" {
                    current_subscription.query = Some(text.clone());
                }
                if current_element == "Endpoint" || current_element == "ManagementEndpoint" {
                    config.management_endpoints.push(text);
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                match tag.as_str() {
                    "CounterSet" => {
                        in_counter_set = false;
                        config.counter_sets.push(current_counter_set.clone());
                    }
                    "Subscription" => {
                        in_subscription = false;
                        config.subscriptions.push(current_subscription.clone());
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                tracing::warn!(
                    "XML parse error at position {}: {e}",
                    reader.error_position()
                );
                break;
            }
            _ => {}
        }
    }

    Ok(config)
}
