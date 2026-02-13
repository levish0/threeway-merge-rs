use std::fs;
use std::path::Path;
use std::process::Command;
use threeway_merge::*;

// Test scenario from file system
struct TestScenario {
    name: String,
    base: String,
    ours: String,
    theirs: String,
}

struct GitMergeOutput {
    content: String,
    conflicts: usize,
}

fn load_test_scenarios() -> Result<Vec<TestScenario>, Box<dyn std::error::Error>> {
    let scenarios_dir = Path::new("tests/scenarios");
    let mut scenarios = Vec::new();
    
    for entry in fs::read_dir(scenarios_dir)? {
        let entry = entry?;
        let scenario_path = entry.path();
        
        if scenario_path.is_dir() {
            let scenario_name = scenario_path.file_name()
                .and_then(|name| name.to_str())
                .ok_or("Invalid scenario directory name")?;
            
            let base_path = scenario_path.join("base.txt");
            let ours_path = scenario_path.join("ours.txt");
            let theirs_path = scenario_path.join("theirs.txt");
            
            if base_path.exists() && ours_path.exists() && theirs_path.exists() {
                let base = fs::read_to_string(&base_path)?;
                let ours = fs::read_to_string(&ours_path)?;
                let theirs = fs::read_to_string(&theirs_path)?;
                
                scenarios.push(TestScenario {
                    name: scenario_name.to_string(),
                    base,
                    ours,
                    theirs,
                });
            }
        }
    }
    
    scenarios.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(scenarios)
}

fn ensure_git_available() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git").arg("--version").output()?;
    if !output.status.success() {
        return Err(std::io::Error::other("git is installed but not runnable").into());
    }
    Ok(())
}

fn git_merge_file(
    base: &str,
    ours: &str,
    theirs: &str,
    algorithm: DiffAlgorithm,
    _level: MergeLevel,
    favor: Option<MergeFavor>,
    style: MergeStyle,
) -> Result<GitMergeOutput, Box<dyn std::error::Error>> {
    let temp_dir = tempfile::tempdir()?;
    let base_path = temp_dir.path().join("base.txt");
    let ours_path = temp_dir.path().join("ours.txt");
    let theirs_path = temp_dir.path().join("theirs.txt");
    
    fs::write(&base_path, base)?;
    fs::write(&ours_path, ours)?;
    fs::write(&theirs_path, theirs)?;
    
    let mut cmd = Command::new("git");
    cmd.arg("merge-file");
    
    // Algorithm
    match algorithm {
        DiffAlgorithm::Myers => {},
        DiffAlgorithm::Minimal => { cmd.arg("--diff-algorithm").arg("minimal"); },
        DiffAlgorithm::Patience => { cmd.arg("--diff-algorithm").arg("patience"); },
        DiffAlgorithm::Histogram => { cmd.arg("--diff-algorithm").arg("histogram"); },
    }
    
    // Style
    match style {
        MergeStyle::Normal => {},
        MergeStyle::Diff3 => { cmd.arg("--diff3"); },
        MergeStyle::ZealousDiff3 => { cmd.arg("--zdiff3"); },
    }
    
    // Favor
    if let Some(favor_option) = favor {
        match favor_option {
            MergeFavor::Ours => { cmd.arg("--ours"); },
            MergeFavor::Theirs => { cmd.arg("--theirs"); },
            MergeFavor::Union => { cmd.arg("--union"); },
        }
    }
    
    // Note: Git merge-file doesn't have direct level control, so we'll compare
    // what we can and note differences for levels
    
    cmd.arg("-L").arg("ours")
       .arg(&ours_path)
       .arg("-L").arg("base") 
       .arg(&base_path)
       .arg("-L").arg("theirs")
       .arg(&theirs_path)
       .arg("--stdout");
    
    let output = cmd.output()?;
    let status_code = output.status.code().unwrap_or(-1);
    let conflicts = match status_code {
        0..=127 => status_code as usize,
        _ => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "git merge-file failed with status {}: {}",
                status_code,
                stderr.trim()
            )
            .into());
        }
    };

    Ok(GitMergeOutput {
        content: String::from_utf8_lossy(&output.stdout).to_string(),
        conflicts,
    })
}

fn normalize_output(output: &str) -> String {
    // Keep comparison strict and only normalize platform line-endings.
    output.replace("\r\n", "\n")
}

#[test]
fn test_comprehensive_git_comparison() {
    ensure_git_available().expect("git is required for comprehensive compatibility tests");
    let scenarios = load_test_scenarios().expect("Failed to load test scenarios");
    println!("Loaded {} test scenarios", scenarios.len());
    
    let algorithms = [
        DiffAlgorithm::Myers,
        DiffAlgorithm::Minimal, 
        DiffAlgorithm::Patience,
        DiffAlgorithm::Histogram,
    ];
    
    // Git doesn't have merge level control, so we only test with Zealous 
    // which matches Git's default behavior (as discovered in our tests)
    let levels = [
        MergeLevel::Zealous,
    ];
    
    let favors = [
        None,
        Some(MergeFavor::Ours),
        Some(MergeFavor::Theirs),
        Some(MergeFavor::Union),
    ];
    
    let styles = [
        MergeStyle::Normal,
        MergeStyle::Diff3,
        MergeStyle::ZealousDiff3,
    ];
    
    let mut total_tests = 0;
    let mut passing_tests = 0usize;
    let mut failing_tests = Vec::new();
    
    for scenario in &scenarios {
        for &algorithm in &algorithms {
            for &level in &levels {
                for &favor in &favors {
                    for &style in &styles {
                        total_tests += 1;
                        
                        let mut options = MergeOptions::default();
                        options.algorithm = algorithm;
                        options.level = level;
                        options.favor = favor;
                        options.style = style;
                        options.ours_label = Some("ours".to_string());
                        options.base_label = Some("base".to_string());
                        options.theirs_label = Some("theirs".to_string());
                        
                        // Our result
                        let our_result = merge_strings(
                            &scenario.base,
                            &scenario.ours, 
                            &scenario.theirs,
                            &options
                        );
                        
                        // Git result (when available - note some combinations aren't supported)
                        let git_result = git_merge_file(
                            &scenario.base,
                            &scenario.ours,
                            &scenario.theirs,
                            algorithm,
                            level,
                            favor,
                            style
                        );
                        
                        match (our_result, git_result) {
                            (Ok(our), Ok(git)) => {
                                let our_normalized = normalize_output(&our.content);
                                let git_normalized = normalize_output(&git.content);
                                let conflict_match = our.conflicts == git.conflicts;
                                
                                if our_normalized == git_normalized && conflict_match {
                                    passing_tests += 1;
                                } else {
                                    let test_name = format!(
                                        "{}_{:?}_{:?}_{:?}_{:?}_mismatch",
                                        &scenario.name, algorithm, level, favor, style
                                    );
                                    failing_tests.push(test_name);
                                    
                                    // Print detailed comparison for debugging (limit output)
                                    if failing_tests.len() <= 3 {
                                        println!("\n=== MISMATCH: {} ===", &scenario.name);
                                        println!("Algorithm: {:?}, Level: {:?}, Favor: {:?}, Style: {:?}", 
                                               algorithm, level, favor, style);
                                        println!(
                                            "Conflict count - ours: {}, git: {}",
                                            our.conflicts,
                                            git.conflicts
                                        );
                                        let our_preview = our_normalized.lines().take(3).collect::<Vec<_>>().join("\\n");
                                        let git_preview = git_normalized.lines().take(3).collect::<Vec<_>>().join("\\n");
                                        println!("Our output: {}...", our_preview);
                                        println!("Git output: {}...", git_preview);
                                        println!("================================");
                                    }
                                }
                            },
                            (Ok(_our), Err(git_err)) => {
                                let test_name = format!(
                                    "{}_{:?}_{:?}_{:?}_{:?}_git_error",
                                    &scenario.name, algorithm, level, favor, style
                                );
                                failing_tests.push(test_name);
                                if failing_tests.len() <= 3 {
                                    println!("Git invocation failed: {}", git_err);
                                }
                            },
                            (Err(our_err), Ok(_git)) => {
                                let test_name = format!(
                                    "{}_{:?}_{:?}_{:?}_{:?}_our_error",
                                    &scenario.name, algorithm, level, favor, style
                                );
                                failing_tests.push(test_name);
                                if failing_tests.len() <= 3 {
                                    println!("Our implementation failed: {:?}", our_err);
                                }
                            },
                            (Err(our_err), Err(git_err)) => {
                                let test_name = format!(
                                    "{}_{:?}_{:?}_{:?}_{:?}_both_error",
                                    &scenario.name, algorithm, level, favor, style
                                );
                                failing_tests.push(test_name);
                                if failing_tests.len() <= 3 {
                                    println!("Both failed. ours: {:?}, git: {}", our_err, git_err);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
    
    eprintln!("\n=== COMPREHENSIVE TEST RESULTS ===");
    eprintln!("Scenarios tested: {}", scenarios.len());
    eprintln!("Total test combinations: {}", total_tests);
    eprintln!("Passing tests: {}", passing_tests);
    eprintln!("Failing tests: {}", failing_tests.len());
    eprintln!("Success rate: {:.1}%", (passing_tests as f64 / total_tests as f64) * 100.0);
    
    if !failing_tests.is_empty() {
        println!("\nFirst few failing test cases:");
        for (i, test) in failing_tests.iter().enumerate() {
            if i < 10 { // Limit output
                println!("  - {}", test);
            } else {
                println!("  ... and {} more", failing_tests.len() - i);
                break;
            }
        }
    }
    
    assert!(
        failing_tests.is_empty(),
        "Found {} incompatible cases out of {} combinations",
        failing_tests.len(),
        total_tests
    );
}
