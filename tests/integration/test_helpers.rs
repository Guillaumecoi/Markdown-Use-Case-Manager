// Test helper utilities for safe working directory management

use std::path::Path;
use std::sync::Mutex;
use tempfile::TempDir;

// Global mutex to ensure only one test changes working directory at a time
static WORKING_DIR_MUTEX: Mutex<()> = Mutex::new(());

/// Helper struct that safely manages working directory changes for tests
/// This ensures that working directory changes are synchronized and properly cleaned up
pub struct WorkingDirGuard {
    original_dir: std::path::PathBuf,
    _guard: std::sync::MutexGuard<'static, ()>,
}

impl WorkingDirGuard {
    /// Create a new working directory guard and change to the specified directory
    /// This will block if another test is currently using a different working directory
    pub fn new(new_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // Acquire the mutex to ensure exclusive access to working directory changes
        // Handle poisoned mutex by clearing it
        let guard = match WORKING_DIR_MUTEX.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Warning: Mutex was poisoned, recovering...");
                poisoned.into_inner()
            }
        };

        // Save the current directory
        let original_dir = std::env::current_dir()?;

        // Change to the new directory
        std::env::set_current_dir(new_dir)?;

        Ok(WorkingDirGuard {
            original_dir,
            _guard: guard,
        })
    }
}

impl Drop for WorkingDirGuard {
    fn drop(&mut self) {
        // Restore the original working directory when the guard is dropped
        std::env::set_current_dir(&self.original_dir).expect("Failed to restore working directory");
        // The mutex guard is automatically released when _guard is dropped
    }
}

/// Helper function to run a test in a temporary directory with proper isolation
pub fn with_temp_dir<F, R>(test_fn: F) -> R
where
    F: FnOnce(&TempDir) -> R,
{
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let _guard = WorkingDirGuard::new(temp_dir.path()).expect("Failed to change working directory");
    test_fn(&temp_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_working_dir_guard_isolation() {
        let original_dir = std::env::current_dir().unwrap();

        {
            let temp_dir = TempDir::new().unwrap();
            let _guard = WorkingDirGuard::new(temp_dir.path()).unwrap();

            // Verify we're in the temp directory
            let current_dir = std::env::current_dir().unwrap();
            assert_eq!(current_dir, temp_dir.path());

            // Create a test file
            fs::write("test_file.txt", "test content").unwrap();
            assert!(Path::new("test_file.txt").exists());
        }

        // Verify we're back to the original directory
        let current_dir = std::env::current_dir().unwrap();
        assert_eq!(current_dir, original_dir);

        // Verify the test file doesn't exist in the original directory
        assert!(!Path::new("test_file.txt").exists());
    }

    #[test]
    fn test_with_temp_dir_helper() {
        let original_dir = std::env::current_dir().unwrap();

        let result = with_temp_dir(|temp_dir| {
            // Verify we're in the temp directory
            let current_dir = std::env::current_dir().unwrap();
            assert_eq!(current_dir, temp_dir.path());

            // Create and test file operations
            fs::write("helper_test.txt", "helper content").unwrap();
            assert!(Path::new("helper_test.txt").exists());

            "test_result"
        });

        assert_eq!(result, "test_result");

        // Verify we're back to the original directory
        let current_dir = std::env::current_dir().unwrap();
        assert_eq!(current_dir, original_dir);
    }
}
