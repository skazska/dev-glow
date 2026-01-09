//! Output formatting utilities

use colored::Colorize;
use glow_core::engine::operations::StatusTree;
use glow_core::model::StepStatus;

/// Print an info message
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

/// Print a success message
pub fn print_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red(), message.red());
}

/// Print a status tree
pub fn print_tree(tree: &StatusTree) {
    print_tree_node(tree, "", true);
}

fn print_tree_node(tree: &StatusTree, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let status_icon = format_status_icon(tree.status);
    let purpose = tree.purpose.as_deref().unwrap_or("");

    println!(
        "{}{}{} {} {}",
        prefix,
        connector.dimmed(),
        status_icon,
        tree.id.cyan(),
        format!("({})", purpose).dimmed()
    );

    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    for (i, child) in tree.children.iter().enumerate() {
        let is_last_child = i == tree.children.len() - 1;
        print_tree_node(child, &child_prefix, is_last_child);
    }
}

fn format_status_icon(status: StepStatus) -> colored::ColoredString {
    match status {
        StepStatus::Wait => "○".dimmed(),
        StepStatus::Todo => "◐".yellow(),
        StepStatus::InProgress => "◑".blue().bold(),
        StepStatus::Done => "●".green(),
    }
}
