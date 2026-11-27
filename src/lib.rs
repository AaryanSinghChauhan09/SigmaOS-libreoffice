#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// Errors returned by the LibreOffice conversion planner.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LibreOfficeError {
    #[error("unsupported file format '{0}'")]
    UnsupportedFormat(String),

    #[error("incompatible conversion from {from:?} to {to:?}")]
    IncompatibleConversion {
        from: DocumentFormat,
        to: DocumentFormat,
    },

    #[error("input path cannot be empty")]
    EmptyInputPath,

    #[error("output directory cannot be empty")]
    EmptyOutputDir,
}

/// Supported LibreOffice document formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentFormat {
    Odt,
    Docx,
    Html,
    Ods,
    Xlsx,
    Csv,
    Odp,
    Pptx,
    Pdf,
}

/// Logical document family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFamily {
    Writer,
    Calc,
    Impress,
    Portable,
}

impl DocumentFormat {
    pub fn from_extension(ext: &str) -> Result<Self, LibreOfficeError> {
        match ext.trim().trim_start_matches('.').to_lowercase().as_str() {
            "odt" => Ok(Self::Odt),
            "docx" => Ok(Self::Docx),
            "html" | "htm" => Ok(Self::Html),
            "ods" => Ok(Self::Ods),
            "xlsx" => Ok(Self::Xlsx),
            "csv" => Ok(Self::Csv),
            "odp" => Ok(Self::Odp),
            "pptx" => Ok(Self::Pptx),
            "pdf" => Ok(Self::Pdf),
            other => Err(LibreOfficeError::UnsupportedFormat(other.to_string())),
        }
    }

    pub fn as_soffice_arg(self) -> &'static str {
        match self {
            Self::Odt => "odt",
            Self::Docx => "docx",
            Self::Html => "html",
            Self::Ods => "ods",
            Self::Xlsx => "xlsx",
            Self::Csv => "csv",
            Self::Odp => "odp",
            Self::Pptx => "pptx",
            Self::Pdf => "pdf",
        }
    }

    pub fn family(self) -> DocumentFamily {
        match self {
            Self::Odt | Self::Docx | Self::Html => DocumentFamily::Writer,
            Self::Ods | Self::Xlsx | Self::Csv => DocumentFamily::Calc,
            Self::Odp | Self::Pptx => DocumentFamily::Impress,
            Self::Pdf => DocumentFamily::Portable,
        }
    }
}

/// A validated LibreOffice headless conversion job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversionJob {
    pub input_path: String,
    pub output_dir: String,
    pub source_format: DocumentFormat,
    pub target_format: DocumentFormat,
    pub headless: bool,
}

impl ConversionJob {
    /// Creates and validates a conversion job from an input file path and target format.
    pub fn new(
        input_path: impl Into<String>,
        output_dir: impl Into<String>,
        target_format: DocumentFormat,
    ) -> Result<Self, LibreOfficeError> {
        let input_path = input_path.into();
        let output_dir = output_dir.into();
        if input_path.trim().is_empty() {
            return Err(LibreOfficeError::EmptyInputPath);
        }
        if output_dir.trim().is_empty() {
            return Err(LibreOfficeError::EmptyOutputDir);
        }

        let ext = Path::new(&input_path)
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| LibreOfficeError::UnsupportedFormat(input_path.clone()))?;
        let source_format = DocumentFormat::from_extension(ext)?;

        let compatible = target_format == DocumentFormat::Pdf
            || source_format.family() == target_format.family();
        if !compatible {
            return Err(LibreOfficeError::IncompatibleConversion {
                from: source_format,
                to: target_format,
            });
        }

        Ok(Self {
            input_path,
            output_dir,
            source_format,
            target_format,
            headless: true,
        })
    }

    /// Builds CLI arguments for `soffice`.
    pub fn to_soffice_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.headless {
            args.push("--headless".to_string());
        }
        args.push("--convert-to".to_string());
        args.push(self.target_format.as_soffice_arg().to_string());
        args.push("--outdir".to_string());
        args.push(self.output_dir.clone());
        args.push(self.input_path.clone());
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_writer_to_pdf_conversion_args_aaa() {
        // Arrange
        let job = ConversionJob::new("/docs/report.odt", "/tmp/out", DocumentFormat::Pdf)
            .expect("valid job");

        // Act
        let args = job.to_soffice_args();

        // Assert
        assert_eq!(
            args,
            vec![
                "--headless",
                "--convert-to",
                "pdf",
                "--outdir",
                "/tmp/out",
                "/docs/report.odt"
            ]
        );
    }

    #[test]
    fn test_incompatible_family_conversion_rejected_aaa() {
        // Arrange
        let input = "/docs/budget.xlsx";

        // Act
        let result = ConversionJob::new(input, "/tmp/out", DocumentFormat::Odp);

        // Assert
        assert_eq!(
            result,
            Err(LibreOfficeError::IncompatibleConversion {
                from: DocumentFormat::Xlsx,
                to: DocumentFormat::Odp,
            })
        );
    }

    #[test]
    fn test_unsupported_extension_rejected_aaa() {
        // Arrange
        let input = "/docs/archive.tar.gz";

        // Act
        let result = ConversionJob::new(input, "/tmp/out", DocumentFormat::Pdf);

        // Assert
        assert_eq!(
            result,
            Err(LibreOfficeError::UnsupportedFormat("gz".to_string()))
        );
    }
}
