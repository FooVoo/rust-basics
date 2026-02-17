//! Exercise 08: File Extension Validation - Check file extensions
//! Difficulty: Easy
//!
//! # Learning Objectives
//! - Work with string slices
//! - Validate file extensions
//! - Use pattern matching with strings

/// Check if a filename has a valid extension from the allowed list.
/// Return Ok(extension) if valid, Err otherwise.
pub fn validate_file_extension(filename: &str, allowed: &[&str]) -> Result<String, String> {
    let extension = filename
        .rsplit('.')
        .next()
        .filter(|ext| !ext.is_empty() && *ext != filename)
        .ok_or_else(|| "No file extension found".to_string())?;
    
    if allowed.contains(&extension) {
        Ok(extension.to_string())
    } else {
        Err(format!(
            "Invalid extension '{}'. Allowed: {}",
            extension,
            allowed.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_extension_valid() {
        let allowed = &["txt", "pdf", "doc"];
        assert_eq!(validate_file_extension("file.txt", allowed), Ok("txt".to_string()));
        assert_eq!(validate_file_extension("document.pdf", allowed), Ok("pdf".to_string()));
        assert_eq!(validate_file_extension("report.doc", allowed), Ok("doc".to_string()));
    }

    #[test]
    fn test_validate_extension_invalid() {
        let allowed = &["txt", "pdf"];
        let result = validate_file_extension("file.exe", allowed);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exe"));
    }

    #[test]
    fn test_validate_extension_no_extension() {
        let allowed = &["txt"];
        assert_eq!(
            validate_file_extension("file", allowed),
            Err("No file extension found".to_string())
        );
    }

    #[test]
    fn test_validate_extension_multiple_dots() {
        let allowed = &["tar", "gz"];
        assert_eq!(
            validate_file_extension("archive.tar.gz", allowed),
            Ok("gz".to_string())
        );
    }

    #[test]
    fn test_validate_extension_empty() {
        let allowed = &["txt"];
        assert!(validate_file_extension("file.", allowed).is_err());
    }
}
