#[test]
fn rollback_preserves_index_integrity_with_overlapping_timestamps() {
    use std::time::{SystemTime, UNIX_EPOCH};
    use chrono::Duration;
    
    let mut path = std::env::temp_dir();
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    path.push(format!("amadiag-rollback-{unique}"));
    std::fs::create_dir_all(&path).unwrap();

    // Create two files with same minute timestamp
    let recent_ts = Utc::now() - Duration::days(1);
    let old_ts = Utc::now() - Duration::days(DEFAULT_STALE_LOG_AGE_DAYS + 30);
    
    // Both files have events in same minute
    let ts1 = recent_ts.format("%Y-%m-%dT%H:%M:00Z");
    let ts2 = old_ts.format("%Y-%m-%dT%H:%M:00Z");
    
    std::fs::write(
        path.join("file1.log"),
        format!("{ts1} ERROR file1 error\n"),
    ).unwrap();
    
    std::fs::write(
        path.join("file2.log"),
        format!("{ts2} ERROR file2 error\n"),
    ).unwrap();

    let store = EventStore::from_bundle_dir(&path, OsKind::Linux).unwrap();
    
    // Only file1 should remain
    assert_eq!(store.events().len(), 1);
    assert!(store.events()[0].evidence.file_id.contains("file1"));
    
    // Verify time_index only has valid indices
    for (_minute, indices) in &store.time_index {
        for &idx in indices {
            assert!(idx < store.events().len(), "Index {idx} out of bounds");
        }
    }
    
    let _ = std::fs::remove_dir_all(path);
}
