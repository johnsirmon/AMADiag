use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Duration;

fn main() {
    // Simulate the scenario:
    // File 1 (recent) processed first: adds event at index 0
    // File 2 (stale) processed second: adds event at index 1, then rollback
    
    let recent_ts = chrono::Utc::now() - Duration::days(1);
    let old_ts = chrono::Utc::now() - Duration::days(200);
    
    // Both map to same minute bucket
    let recent_minute = recent_ts.timestamp() / 60;
    let old_minute = old_ts.timestamp() / 60;
    
    println!("Recent minute: {}", recent_minute);
    println!("Old minute: {}", old_minute);
    println!("Same bucket: {}", recent_minute == old_minute);
    
    // Simulate time_index state after file1
    let mut time_index = std::collections::BTreeMap::new();
    time_index.entry(recent_minute).or_insert_with(Vec::new).push(0);
    
    println!("\nAfter file1 (index 0):");
    println!("  Minute {}: {:?}", recent_minute, time_index.get(&recent_minute));
    
    // Simulate adding file2 event (index 1) with old timestamp
    time_index.entry(old_minute).or_insert_with(Vec::new).push(1);
    
    println!("\nAfter file2 added (index 1):");
    println!("  Minute {}: {:?}", recent_minute, time_index.get(&recent_minute));
    println!("  Minute {}: {:?}", old_minute, time_index.get(&old_minute));
    
    // Now rollback file2 events (event_start=1, touched_minutes=[old_minute])
    let event_start = 1;
    let touched_minutes = vec![old_minute];
    
    let mut unique_minutes = touched_minutes.clone();
    unique_minutes.sort_unstable();
    unique_minutes.dedup();
    
    for minute in &unique_minutes {
        if let Some(indices) = time_index.get_mut(minute) {
            println!("\nBefore rollback minute {}: {:?}", minute, indices);
            indices.retain(|index| *index < event_start);
            println!("After rollback minute {}: {:?}", minute, indices);
            if indices.is_empty() {
                println!("  -> Will remove empty bucket");
            }
        }
    }
}
