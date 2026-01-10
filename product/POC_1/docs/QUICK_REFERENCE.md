# Dev-Glow Quick Reference

## Setup

```bash
glow project init --name "MyProject"   # Initialize project
glow init ROOT                          # Initialize root process
glow start ROOT                         # Start working
```

## Step Workflow

```bash
glow init <FQID>      # Initialize step (Wait/Todo)
glow start <FQID>     # Start step (→ InProgress)
glow finish <FQID>    # Complete step (→ Done)
```

## Status Lifecycle

```
Wait → Todo → InProgress → Done
 │      │         │          │
 │      │         │          └── Completed
 │      │         └── Being worked on
 │      └── Ready to start
 └── Blocked by dependencies
```

## Information Commands

```bash
glow status           # Show status tree
glow status --list    # Show as flat list
glow show <FQID>      # Show step details
glow next             # Show what to do next
glow progress         # Show completion metrics
glow validate         # Check quality
```

## With Parameters

```bash
# Initialize with scope parameters
glow init FEAT-001 -- NAME="Auth" PRIORITY="high"

# Finish with outputs and summary
glow finish TASK-001 --summary "Done" -- OUTPUT="value"
```

## Common FQIDs

```
ROOT                    # Root process
FEAT-001                # Feature step
FEAT-001.REQ-001        # Nested requirement
FEAT-001.REQ-001.TASK   # Deeply nested task
```

## File Locations

```
.glow/config.yaml           # Project config
.glow/process_config.yaml   # Process definition
glow/                       # Step data files
glow/ROOT.md                # Root step file
glow/FEAT-001/              # Feature folder
```

## MCP Server

```bash
glow-mcp --project-dir .    # Start MCP server
```

Resources: `glow://project/status`, `glow://step/{fqid}`, `glow://next`
Tools: `glow_status`, `glow_next`, `glow_show_step`, `glow_start_step`, `glow_finish_step`

## Help

```bash
glow --help             # General help
glow <command> --help   # Command help
glow --version          # Version info
```
