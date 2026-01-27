//! AISP CLI - Command Line Interface for AI Symbolic Protocol Validation
//!
//! This tool provides comprehensive validation and analysis capabilities for AISP documents
//! including syntax checking, semantic analysis, relational logic, temporal logic, 
//! and formal verification using Z3.

use aisp_core::*;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "aisp")]
#[command(version = "0.1.0")]
#[command(about = "AISP Validator - AI Symbolic Protocol analysis and validation")]
#[command(long_about = "
A comprehensive validation tool for AI Symbolic Protocol (AISP) 5.1 documents.
Provides multi-level analysis from basic syntax checking to advanced formal verification.

Features:
  • Pure Rust implementation for maximum performance
  • Multi-level validation (Syntax, Semantic, Relational, Temporal)
  • Z3-based formal verification (optional)
  • Rich output formats (human, JSON, detailed)
  • Interactive mode for rapid development
")]
struct Cli {
    /// Input file(s) or directory to validate
    #[arg(value_name = "PATH")]
    input: Vec<PathBuf>,

    /// Output format
    #[arg(short = 'f', long = "format", default_value = "human")]
    format: OutputFormat,

    /// Output file (stdout if not specified)
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Validation level
    #[arg(short = 'l', long = "level", default_value = "semantic")]
    level: ValidationLevel,

    /// Enable strict mode validation
    #[arg(short = 's', long = "strict")]
    strict: bool,

    /// Include timing information
    #[arg(short = 't', long = "timing")]
    timing: bool,

    /// Include detailed AST in output
    #[arg(long = "include-ast")]
    include_ast: bool,

    /// Enable formal verification with Z3
    #[cfg(feature = "z3-verification")]
    #[arg(long = "formal-verification")]
    formal_verification: bool,

    /// Z3 timeout in seconds
    #[cfg(feature = "z3-verification")]
    #[arg(long = "z3-timeout", default_value = "30")]
    z3_timeout: u64,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Quiet mode (errors only)
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Number of parallel workers for batch processing
    #[arg(short = 'j', long = "jobs", default_value = "1")]
    jobs: usize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate AISP document(s)
    Validate {
        /// Files to validate
        files: Vec<PathBuf>,
        
        /// Stop on first error
        #[arg(long = "fail-fast")]
        fail_fast: bool,
    },
    /// Analyze document structure and metrics
    Analyze {
        /// File to analyze
        file: PathBuf,
        
        /// Include symbol statistics
        #[arg(long = "symbols")]
        symbols: bool,
        
        /// Include complexity metrics
        #[arg(long = "complexity")]
        complexity: bool,
    },
    /// Check syntax only (fast)
    Check {
        /// Files to check
        files: Vec<PathBuf>,
    },
    /// Format/prettify AISP document
    Format {
        /// File to format
        file: PathBuf,
        
        /// Format in place
        #[arg(short = 'i', long = "in-place")]
        in_place: bool,
    },
    /// Interactive validation mode
    Interactive,
    /// Show detailed information about validation levels
    Levels,
}

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    /// Human-readable format with colors
    Human,
    /// JSON output for programmatic use
    Json,
    /// Detailed human format with full analysis
    Detailed,
    /// Minimal output (validation result only)
    Minimal,
}

#[derive(ValueEnum, Clone, Debug)]
enum ValidationLevel {
    /// Syntax checking only
    Syntax,
    /// Syntax + semantic analysis
    Semantic,
    /// Semantic + relational logic (Level 4)
    Relational,
    /// Relational + temporal logic (Level 5)
    Temporal,
    /// Full formal verification
    Formal,
}

/// Validation result for output
#[derive(Serialize, Deserialize)]
struct CliValidationResult {
    file: PathBuf,
    valid: bool,
    tier: String,
    tier_symbol: String,
    tier_value: u8,
    delta: f64,
    ambiguity: f64,
    document_size: usize,
    parse_time_ms: Option<u64>,
    semantic_time_ms: Option<u64>,
    total_time_ms: Option<u64>,
    warnings: Vec<String>,
    errors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relational_analysis: Option<RelationalSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temporal_analysis: Option<TemporalSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formal_verification: Option<FormalSummary>,
}

#[derive(Serialize, Deserialize)]
struct RelationalSummary {
    consistency_score: f64,
    constraints_total: usize,
    constraints_satisfied: usize,
    conflicts: usize,
}

#[derive(Serialize, Deserialize)]
struct TemporalSummary {
    consistency_score: f64,
    ltl_formulas: usize,
    ctl_formulas: usize,
    patterns_detected: usize,
}

#[derive(Serialize, Deserialize)]
struct FormalSummary {
    status: String,
    properties_checked: usize,
    properties_proven: usize,
    verification_time_ms: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging based on verbosity
    setup_logging(&cli)?;

    match cli.command {
        Some(Commands::Validate { ref files, fail_fast }) => {
            validate_files(&cli, files.clone(), fail_fast).await
        }
        Some(Commands::Analyze { ref file, symbols, complexity }) => {
            analyze_file(&cli, file.clone(), symbols, complexity).await
        }
        Some(Commands::Check { ref files }) => {
            check_files(&cli, files.clone()).await
        }
        Some(Commands::Format { ref file, in_place }) => {
            format_file(&cli, file.clone(), in_place).await
        }
        Some(Commands::Interactive) => {
            interactive_mode(&cli).await
        }
        Some(Commands::Levels) => {
            show_validation_levels()
        }
        None => {
            // Default behavior: validate input files
            if cli.input.is_empty() {
                eprintln!("{}", "Error: No input files specified".red());
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
            validate_files(&cli, cli.input.clone(), false).await
        }
    }
}

/// Setup logging based on CLI options
fn setup_logging(_cli: &Cli) -> Result<()> {
    // For now, just handle verbosity flags
    // In a full implementation, this would setup proper logging
    Ok(())
}

/// Validate multiple files
async fn validate_files(cli: &Cli, files: Vec<PathBuf>, fail_fast: bool) -> Result<()> {
    if !cli.quiet {
        println!("{} AISP Validator", "🔍".bold());
        println!();
    }

    let mut results = Vec::new();
    let mut total_errors = 0;

    // Setup progress bar for multiple files
    let progress = if files.len() > 1 && !cli.quiet {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    for file in files {
        if let Some(pb) = &progress {
            pb.set_message(format!("Validating {}", file.display()));
        }

        let result = validate_single_file(cli, &file).await?;
        
        if !result.valid {
            total_errors += 1;
            if fail_fast {
                if let Some(pb) = progress {
                    pb.finish_and_clear();
                }
                eprintln!("{} Validation failed for {}", "✗".red().bold(), file.display());
                std::process::exit(1);
            }
        }

        results.push(result);
        
        if let Some(pb) = &progress {
            pb.inc(1);
        }
    }

    if let Some(pb) = progress {
        pb.finish_and_clear();
    }

    // Output results
    output_results(cli, &results)?;

    if total_errors > 0 {
        if !cli.quiet {
            eprintln!();
            eprintln!("{} {} file(s) failed validation", 
                "✗".red().bold(), total_errors);
        }
        std::process::exit(1);
    } else if !cli.quiet {
        eprintln!();
        eprintln!("{} All files validated successfully", "✓".green().bold());
    }

    Ok(())
}

/// Validate a single file
async fn validate_single_file(cli: &Cli, file: &Path) -> Result<CliValidationResult> {
    let source = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file {}", file.display()))?;

    // Create validation config
    let mut config = ValidationConfig::default();
    config.strict_mode = cli.strict;
    config.include_timing = cli.timing;
    config.include_ast = cli.include_ast;
    config.include_symbol_stats = true;

    // Set validation level
    match cli.level {
        ValidationLevel::Formal => {
            config.enable_formal_verification = true;
            #[cfg(feature = "z3-verification")]
            {
                config.z3_timeout = Duration::from_secs(cli.z3_timeout);
            }
        }
        _ => {}
    }

    let validator = AispValidator::with_config(config);
    let validation_result = validator.validate(&source);

    // Convert to CLI result
    let mut cli_result = CliValidationResult {
        file: file.to_path_buf(),
        valid: validation_result.valid,
        tier: validation_result.tier_name.clone(),
        tier_symbol: validation_result.tier_symbol.clone(),
        tier_value: validation_result.tier_value,
        delta: validation_result.delta,
        ambiguity: validation_result.ambiguity,
        document_size: validation_result.document_size,
        parse_time_ms: validation_result.parse_time.map(|d| d.as_millis() as u64),
        semantic_time_ms: validation_result.semantic_time.map(|d| d.as_millis() as u64),
        total_time_ms: validation_result.total_time.map(|d| d.as_millis() as u64),
        warnings: validation_result.warnings.iter().map(|w| w.to_string()).collect(),
        errors: if let Some(error) = &validation_result.error {
            vec![error.to_string()]
        } else {
            Vec::new()
        },
        relational_analysis: None,
        temporal_analysis: None,
        formal_verification: None,
    };

    // Add analysis summaries based on validation level
    if let Some(semantic) = &validation_result.semantic_analysis {
        if let Some(rel_analysis) = &semantic.relational_analysis() {
            cli_result.relational_analysis = Some(RelationalSummary {
                consistency_score: rel_analysis.consistency_score,
                constraints_total: rel_analysis.constraint_analysis.constraints.len(),
                constraints_satisfied: rel_analysis.constraint_analysis.satisfied.len(),
                conflicts: rel_analysis.conflict_analysis.conflicts.len(),
            });
        }

        if let Some(temp_analysis) = &semantic.temporal_analysis() {
            cli_result.temporal_analysis = Some(TemporalSummary {
                consistency_score: temp_analysis.consistency_score,
                ltl_formulas: temp_analysis.formula_analysis.formulas.len(),
                ctl_formulas: temp_analysis.formula_analysis.formulas.len(), // Both use same formula set
                patterns_detected: temp_analysis.pattern_analysis.patterns.len(),
            });
        }
    }

    if let Some(formal) = &validation_result.formal_verification {
        cli_result.formal_verification = Some(FormalSummary {
            status: format!("{:?}", formal.status),
            properties_checked: formal.properties.len(),
            properties_proven: formal.properties.iter()
                .filter(|p| p.status == PropertyStatus::Proven).count(),
            verification_time_ms: formal.stats.total_time.as_millis() as u64,
        });
    }

    Ok(cli_result)
}

/// Output validation results
fn output_results(cli: &Cli, results: &[CliValidationResult]) -> Result<()> {
    match cli.format {
        OutputFormat::Json => output_json(cli, results),
        OutputFormat::Human => output_human(cli, results, false),
        OutputFormat::Detailed => output_human(cli, results, true),
        OutputFormat::Minimal => output_minimal(cli, results),
    }
}

/// Output results in JSON format
fn output_json(cli: &Cli, results: &[CliValidationResult]) -> Result<()> {
    let json = if results.len() == 1 {
        serde_json::to_string_pretty(&results[0])?
    } else {
        serde_json::to_string_pretty(results)?
    };

    write_output(cli, &json)?;
    Ok(())
}

/// Output results in human-readable format
fn output_human(cli: &Cli, results: &[CliValidationResult], detailed: bool) -> Result<()> {
    for result in results {
        if !cli.quiet {
            println!("{}", format!("File: {}", result.file.display()).bold());
            
            // Validation status
            if result.valid {
                println!("  Status: {} {}", "✓".green().bold(), "Valid".green().bold());
            } else {
                println!("  Status: {} {}", "✗".red().bold(), "Invalid".red().bold());
            }

            // Quality tier
            println!("  Quality: {} {} (δ={:.3}, ambiguity={:.3})", 
                result.tier_symbol.blue().bold(),
                result.tier.blue(),
                result.delta,
                result.ambiguity
            );

            // Document size
            println!("  Size: {} bytes", result.document_size);

            // Timing information
            if let Some(total_ms) = result.total_time_ms {
                println!("  Time: {}ms total", total_ms);
                if detailed {
                    if let Some(parse_ms) = result.parse_time_ms {
                        println!("    Parse: {}ms", parse_ms);
                    }
                    if let Some(semantic_ms) = result.semantic_time_ms {
                        println!("    Semantic: {}ms", semantic_ms);
                    }
                }
            }

            // Analysis results
            if detailed {
                if let Some(rel) = &result.relational_analysis {
                    println!("  Relational Analysis:");
                    println!("    Consistency: {:.3}", rel.consistency_score);
                    println!("    Constraints: {}/{} satisfied", 
                        rel.constraints_satisfied, rel.constraints_total);
                    if rel.conflicts > 0 {
                        println!("    Conflicts: {}", rel.conflicts.to_string().red());
                    }
                }

                if let Some(temp) = &result.temporal_analysis {
                    println!("  Temporal Analysis:");
                    println!("    Consistency: {:.3}", temp.consistency_score);
                    println!("    LTL Formulas: {}", temp.ltl_formulas);
                    println!("    CTL Formulas: {}", temp.ctl_formulas);
                    println!("    Patterns: {}", temp.patterns_detected);
                }

                if let Some(formal) = &result.formal_verification {
                    println!("  Formal Verification:");
                    println!("    Status: {}", formal.status);
                    println!("    Properties: {}/{} proven", 
                        formal.properties_proven, formal.properties_checked);
                    println!("    Time: {}ms", formal.verification_time_ms);
                }
            }

            // Warnings and errors
            if !result.warnings.is_empty() {
                println!("  Warnings:");
                for warning in &result.warnings {
                    println!("    {}", warning.yellow());
                }
            }

            if !result.errors.is_empty() {
                println!("  Errors:");
                for error in &result.errors {
                    println!("    {}", error.red());
                }
            }

            println!();
        }
    }

    Ok(())
}

/// Output results in minimal format
fn output_minimal(cli: &Cli, results: &[CliValidationResult]) -> Result<()> {
    for result in results {
        let status = if result.valid { "✓" } else { "✗" };
        let line = format!("{} {} {}", status, result.file.display(), result.tier_symbol);
        write_output(cli, &line)?;
    }
    Ok(())
}

/// Write output to file or stdout
fn write_output(cli: &Cli, content: &str) -> Result<()> {
    match &cli.output {
        Some(output_file) => {
            fs::write(output_file, content)
                .with_context(|| format!("Failed to write to {}", output_file.display()))?;
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

/// Analyze a single file in detail
async fn analyze_file(cli: &Cli, file: PathBuf, _symbols: bool, _complexity: bool) -> Result<()> {
    println!("{} Analyzing {}", "🔬".bold(), file.display().to_string().cyan());
    
    // For now, just run detailed validation
    let mut detailed_cli = cli.clone();
    detailed_cli.format = OutputFormat::Detailed;
    
    validate_single_file(&detailed_cli, &file).await?;
    
    Ok(())
}

/// Check syntax of files (fast mode)
async fn check_files(cli: &Cli, files: Vec<PathBuf>) -> Result<()> {
    let mut syntax_cli = cli.clone();
    syntax_cli.level = ValidationLevel::Syntax;
    syntax_cli.format = OutputFormat::Minimal;
    
    validate_files(&syntax_cli, files, false).await
}

/// Format AISP file
async fn format_file(_cli: &Cli, _file: PathBuf, _in_place: bool) -> Result<()> {
    // TODO: Implement AISP formatter
    eprintln!("{} Formatting not yet implemented", "⚠️".yellow());
    Ok(())
}

/// Interactive validation mode
async fn interactive_mode(_cli: &Cli) -> Result<()> {
    println!("{} Interactive AISP Validator", "🚀".bold());
    println!("Enter AISP text (Ctrl+D to validate, 'quit' to exit):");
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let input = input.trim();
                if input == "quit" {
                    break;
                }
                
                // Quick validation
                let validator = AispValidator::new();
                let result = validator.validate(input);
                
                if result.valid {
                    println!("  {} Valid {} (δ={:.3})", 
                        "✓".green().bold(), 
                        result.tier_symbol.blue(), 
                        result.delta
                    );
                } else {
                    println!("  {} Invalid", "✗".red().bold());
                    if let Some(error) = result.error {
                        println!("    {}", error.to_string().red());
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
    
    println!("Goodbye!");
    Ok(())
}

/// Show information about validation levels
fn show_validation_levels() -> Result<()> {
    println!("{} AISP Validation Levels", "📋".bold());
    println!();
    
    println!("{} {} - Basic syntax checking", "1".cyan().bold(), "Syntax".bold());
    println!("  • Parse AISP document structure");
    println!("  • Check Unicode symbol usage");
    println!("  • Validate block organization");
    println!();
    
    println!("{} {} - Semantic analysis", "2".cyan().bold(), "Semantic".bold());
    println!("  • Type checking and inference");
    println!("  • Symbol usage validation");
    println!("  • Quality tier calculation (δ)");
    println!("  • Ambiguity measurement");
    println!();
    
    println!("{} {} - Relational logic (Level 4)", "3".cyan().bold(), "Relational".bold());
    println!("  • Set theory validation");
    println!("  • Type relationship analysis");
    println!("  • Constraint satisfaction");
    println!("  • Dependency graph construction");
    println!();
    
    println!("{} {} - Temporal logic (Level 5)", "4".cyan().bold(), "Temporal".bold());
    println!("  • Linear Temporal Logic (LTL)");
    println!("  • Computation Tree Logic (CTL)");
    println!("  • Model checking");
    println!("  • Pattern recognition");
    println!();
    
    #[cfg(feature = "z3-verification")]
    {
        println!("{} {} - Formal verification", "5".cyan().bold(), "Formal".bold());
        println!("  • Z3 theorem proving");
        println!("  • Property verification");
        println!("  • Counterexample generation");
        println!("  • Proof certificates");
    }
    
    #[cfg(not(feature = "z3-verification"))]
    {
        println!("{} {} - Formal verification {}", 
            "5".cyan().bold(), 
            "Formal".bold(),
            "(disabled - compile with z3-verification feature)".yellow()
        );
    }
    
    Ok(())
}

/// Implement Clone for Cli struct
impl Clone for Cli {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
            format: self.format.clone(),
            output: self.output.clone(),
            level: self.level.clone(),
            strict: self.strict,
            timing: self.timing,
            include_ast: self.include_ast,
            #[cfg(feature = "z3-verification")]
            formal_verification: self.formal_verification,
            #[cfg(feature = "z3-verification")]
            z3_timeout: self.z3_timeout,
            verbose: self.verbose,
            quiet: self.quiet,
            jobs: self.jobs,
            command: None, // Don't clone the command
        }
    }
}