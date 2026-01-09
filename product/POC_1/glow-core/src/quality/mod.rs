//! Quality assessment module
//!
//! Provides context quality metrics and assessment.

use crate::engine::operations::{ProcessEngine, ValidationReport};
use crate::error::Result;

/// Quality assessment levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityLevel {
    /// 100% - All required parameters filled
    Complete,
    /// >= 80% - Most parameters filled
    Sufficient,
    /// < 80% - Missing critical context
    Insufficient,
}

impl QualityLevel {
    /// Determine quality level from completeness percentage
    pub fn from_percentage(percentage: f64) -> Self {
        if percentage >= 100.0 {
            QualityLevel::Complete
        } else if percentage >= 80.0 {
            QualityLevel::Sufficient
        } else {
            QualityLevel::Insufficient
        }
    }
}

impl std::fmt::Display for QualityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QualityLevel::Complete => write!(f, "Complete"),
            QualityLevel::Sufficient => write!(f, "Sufficient"),
            QualityLevel::Insufficient => write!(f, "Insufficient"),
        }
    }
}

/// Quality report for a project
#[derive(Debug, Clone)]
pub struct QualityReport {
    /// Overall quality level
    pub level: QualityLevel,
    /// Completeness percentage
    pub completeness: f64,
    /// Consistency status
    pub is_consistent: bool,
    /// Semantic connection status
    pub has_semantic_connection: bool,
    /// Detailed validation report
    pub validation: ValidationReport,
}

/// Assess quality of a project
pub fn assess_quality(engine: &ProcessEngine, fqid: Option<&str>) -> Result<QualityReport> {
    let validation = engine.validate(fqid)?;

    let level = QualityLevel::from_percentage(validation.completeness);
    let is_consistent = validation.issues.is_empty();

    // Check semantic connection by looking at link coverage
    let has_semantic_connection = validation.issues.iter()
        .all(|i| !matches!(i.issue_type, crate::engine::operations::IssueType::BrokenLink));

    Ok(QualityReport {
        level,
        completeness: validation.completeness,
        is_consistent,
        has_semantic_connection,
        validation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_level_from_percentage() {
        assert_eq!(QualityLevel::from_percentage(100.0), QualityLevel::Complete);
        assert_eq!(QualityLevel::from_percentage(95.0), QualityLevel::Sufficient);
        assert_eq!(QualityLevel::from_percentage(80.0), QualityLevel::Sufficient);
        assert_eq!(QualityLevel::from_percentage(79.0), QualityLevel::Insufficient);
        assert_eq!(QualityLevel::from_percentage(50.0), QualityLevel::Insufficient);
    }
}
