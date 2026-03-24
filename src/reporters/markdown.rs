use crate::analyzers::finding::{DiagnosticReport, Severity};

/// Render a diagnostic report as Markdown.
pub fn render(report: &DiagnosticReport) -> String {
    let mut out = String::with_capacity(4096);

    // Title
    out.push_str("# AMADiag Diagnostic Report\n\n");

    // Summary
    out.push_str("## Summary\n\n");
    out.push_str(&format!("- **Bundle**: `{}`\n", report.bundle_path));
    out.push_str(&format!("- **Files analyzed**: {}\n", report.files_analyzed));
    out.push_str(&format!(
        "- **Total findings**: {}\n",
        report.findings.len()
    ));
    out.push_str(&format!(
        "- **Critical**: {}\n",
        report.finding_count_by_severity(Severity::Critical)
    ));
    out.push_str(&format!(
        "- **Warning**: {}\n",
        report.finding_count_by_severity(Severity::Warning)
    ));
    out.push_str(&format!(
        "- **Info**: {}\n",
        report.finding_count_by_severity(Severity::Info)
    ));
    out.push('\n');

    // Environment
    out.push_str("## Environment\n\n");
    let env = &report.environment;
    if let Some(os) = &env.os {
        out.push_str(&format!("- **OS**: {os}\n"));
    }
    if let Some(platform) = &env.platform {
        out.push_str(&format!("- **Platform**: {platform}\n"));
    }
    if let Some(version) = &env.ama_version {
        out.push_str(&format!("- **AMA Version**: {version}\n"));
    }
    if let Some(sku) = &env.vm_sku {
        out.push_str(&format!("- **VM SKU**: {sku}\n"));
    }
    if let Some(is_arc) = env.is_arc {
        out.push_str(&format!(
            "- **Arc-enabled**: {}\n",
            if is_arc { "Yes" } else { "No" }
        ));
    }
    if env.dcr_count > 0 {
        out.push_str(&format!("- **DCR Count**: {}\n", env.dcr_count));
    }
    if let Some(hostname) = &env.hostname {
        out.push_str(&format!("- **Hostname**: {hostname}\n"));
    }
    out.push('\n');

    // Findings table
    if report.findings.is_empty() {
        out.push_str("## Findings\n\n");
        out.push_str("No issues detected. The AMA configuration appears healthy.\n\n");
        return out;
    }

    out.push_str("## Findings Overview\n\n");
    out.push_str("| # | Severity | Category | Rule | Name |\n");
    out.push_str("|---|----------|----------|------|------|\n");

    for (i, f) in report.findings.iter().enumerate() {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            i + 1,
            severity_emoji(f.severity),
            f.category,
            f.rule_id,
            f.name
        ));
    }
    out.push('\n');

    // Detailed findings
    out.push_str("## Detailed Findings\n\n");

    for (i, f) in report.findings.iter().enumerate() {
        out.push_str(&format!(
            "### {}. {} {} — {}\n\n",
            i + 1,
            severity_emoji(f.severity),
            f.rule_id,
            f.name
        ));
        out.push_str(&format!("**Severity**: {}  \n", f.severity));
        out.push_str(&format!("**Category**: {}  \n\n", f.category));
        out.push_str(&format!("{}\n\n", f.description));

        if !f.evidence.is_empty() {
            out.push_str("**Evidence:**\n\n```\n");
            for line in &f.evidence {
                out.push_str(line);
                out.push('\n');
            }
            out.push_str("```\n\n");
        }

        out.push_str("**Remediation:**\n\n");
        out.push_str(&f.remediation);
        out.push_str("\n\n");

        if let Some(link) = &f.doc_link {
            out.push_str(&format!("**Documentation**: [{link}]({link})\n\n"));
        }

        out.push_str("---\n\n");
    }

    out
}

fn severity_emoji(s: Severity) -> &'static str {
    match s {
        Severity::Critical => "🔴 Critical",
        Severity::Warning => "🟡 Warning",
        Severity::Info => "🔵 Info",
    }
}
