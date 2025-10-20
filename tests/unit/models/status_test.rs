// Unit tests for Status enum and related functionality
use markdown_use_case_manager::core::models::Status;

/// Test Status enum variants exist and have correct ordering
#[test]
fn test_status_enum_variants() {
    let status = Status::Planned;
    assert_eq!(format!("{:?}", status), "Planned");

    let status = Status::InProgress;
    assert_eq!(format!("{:?}", status), "InProgress");

    let status = Status::Implemented;
    assert_eq!(format!("{:?}", status), "Implemented");

    let status = Status::Tested;
    assert_eq!(format!("{:?}", status), "Tested");

    let status = Status::Deployed;
    assert_eq!(format!("{:?}", status), "Deployed");

    let status = Status::Deprecated;
    assert_eq!(format!("{:?}", status), "Deprecated");
}

/// Test Status priority method returns correct priority values
#[test]
fn test_status_priority() {
    assert_eq!(Status::Deprecated.priority(), 0);
    assert_eq!(Status::Planned.priority(), 1);
    assert_eq!(Status::InProgress.priority(), 2);
    assert_eq!(Status::Implemented.priority(), 3);
    assert_eq!(Status::Tested.priority(), 4);
    assert_eq!(Status::Deployed.priority(), 5);
}

/// Test Status emoji method returns correct emojis
#[test]
fn test_status_emoji() {
    assert_eq!(Status::Planned.emoji(), "📋");
    assert_eq!(Status::InProgress.emoji(), "🔄");
    assert_eq!(Status::Implemented.emoji(), "⚡");
    assert_eq!(Status::Tested.emoji(), "✅");
    assert_eq!(Status::Deployed.emoji(), "🚀");
    assert_eq!(Status::Deprecated.emoji(), "⚠️");
}

/// Test Status display_name method returns correct names
#[test]
fn test_status_display_name() {
    assert_eq!(Status::Planned.display_name(), "PLANNED");
    assert_eq!(Status::InProgress.display_name(), "IN_PROGRESS");
    assert_eq!(Status::Implemented.display_name(), "IMPLEMENTED");
    assert_eq!(Status::Tested.display_name(), "TESTED");
    assert_eq!(Status::Deployed.display_name(), "DEPLOYED");
    assert_eq!(Status::Deprecated.display_name(), "DEPRECATED");
}

/// Test Status Display trait implementation
#[test]
fn test_status_display() {
    assert_eq!(Status::Planned.to_string(), "📋 PLANNED");
    assert_eq!(Status::InProgress.to_string(), "🔄 IN_PROGRESS");
    assert_eq!(Status::Implemented.to_string(), "⚡ IMPLEMENTED");
    assert_eq!(Status::Tested.to_string(), "✅ TESTED");
    assert_eq!(Status::Deployed.to_string(), "🚀 DEPLOYED");
    assert_eq!(Status::Deprecated.to_string(), "⚠️ DEPRECATED");
}

/// Test Status aggregate with empty vector defaults to Planned
#[test]
fn test_status_aggregate_empty() {
    let statuses = vec![];
    assert_eq!(Status::aggregate(&statuses), Status::Planned);
}

/// Test Status aggregate with single status returns that status
#[test]
fn test_status_aggregate_single() {
    let statuses = vec![Status::InProgress];
    assert_eq!(Status::aggregate(&statuses), Status::InProgress);

    let statuses = vec![Status::Tested];
    assert_eq!(Status::aggregate(&statuses), Status::Tested);

    let statuses = vec![Status::Deprecated];
    assert_eq!(Status::aggregate(&statuses), Status::Deprecated);
}

/// Test Status aggregate with all planned returns planned
#[test]
fn test_status_aggregate_all_planned() {
    let statuses = vec![Status::Planned, Status::Planned, Status::Planned];
    assert_eq!(Status::aggregate(&statuses), Status::Planned);
}

/// Test Status aggregate with deprecated always returns deprecated
#[test]
fn test_status_aggregate_deprecated_wins() {
    let statuses = vec![Status::Deployed, Status::Deprecated, Status::Tested];
    assert_eq!(Status::aggregate(&statuses), Status::Deprecated);

    let statuses = vec![Status::Planned, Status::Deprecated];
    assert_eq!(Status::aggregate(&statuses), Status::Deprecated);
}

/// Test Status aggregate returns lowest non-planned status
#[test]
fn test_status_aggregate_lowest_non_planned() {
    let statuses = vec![Status::Planned, Status::InProgress, Status::Tested];
    assert_eq!(Status::aggregate(&statuses), Status::InProgress);

    let statuses = vec![Status::Deployed, Status::Implemented, Status::Tested];
    assert_eq!(Status::aggregate(&statuses), Status::Implemented);

    let statuses = vec![Status::Tested, Status::Deployed, Status::InProgress];
    assert_eq!(Status::aggregate(&statuses), Status::InProgress);
}

/// Test Status comparison and ordering (based on enum variant order)
#[test]
fn test_status_ordering() {
    assert!(Status::Planned < Status::InProgress);
    assert!(Status::InProgress < Status::Implemented);
    assert!(Status::Implemented < Status::Tested);
    assert!(Status::Tested < Status::Deployed);
    assert!(Status::Deployed < Status::Deprecated);
}

/// Test Status equality
#[test]
fn test_status_equality() {
    assert_eq!(Status::Planned, Status::Planned);
    assert_ne!(Status::Planned, Status::InProgress);
    assert_eq!(Status::Tested, Status::Tested);
}

/// Test Status serialization works (for YAML storage)
#[test]
fn test_status_serialization() {
    let status = Status::InProgress;

    let serialized = serde_json::to_string(&status).expect("Failed to serialize");
    let deserialized: Status = serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(status, deserialized);
}
