## Project Overview

**Project Name**: papa_moo_3_bills (kbm3-plumbling)

**Repository**: https://github.com/mojisejr/kbm3-plumbling
**Author**: mojisejr

**Description**: A small Rust command-line program that reads plumbing/billing data from CSV files and generates PDF receipts. The project uses the `printpdf` crate and custom drawing/layout code to produce Thai-language formatted bills. The codebase is modular, with clear responsibilities split across `csv_util`, `pdf_util`, `drawing`, `font_util`, `model`, and `log`.

---

## ⚠️ CRITICAL SAFETY RULES

### 🚨 FORBIDDEN ACTIONS (NEVER ALLOWED)

- ❌ **NEVER merge PRs yourself** - Provide PR link and wait for user instructions
- ❌ **NEVER work on main/staging branches** - Always use feature branches
- ❌ **NEVER delete critical files** (.env, .git/, node_modules/, package.json, lib/database/)
- ❌ **NEVER commit sensitive data** (API keys, passwords, secrets) - Use environment variables
- ❌ **NEVER skip 100% validation** (build, lint, test) - Must pass completely
- ❌ **NEVER use git push --force** - Only use --force-with-lease when absolutely necessary
- ❌ **NEVER implement without task issue** - Must use =plan command first

### 📋 MANDATORY WORKFLOW RULES

- ✅ **ALWAYS** sync main branch before any implementation: `git checkout main && git pull origin main`
- ✅ **ALWAYS** verify task issue exists: `#[issue-number]` before `=impl`
- ✅ **ALWAYS** use feature branch naming: `feature/task-[issue-number]-[description]`
- ✅ **ALWAYS** ensure 100% build success before commit: `npm run build`
- ✅ **ALWAYS** ensure 100% lint pass before commit: `npm run lint`
- ✅ **ALWAYS** use template-guided workflow with proper context validation
- ✅ **ALWAYS** test desktop app functionality before deployment

---

## 📋 Workflow System

### Template Integration

**Context Issue Template** - `/docs/ISSUE-TEMP.md`:

- Used for: `=fcs > [topic-name]` or `=fcs > [CONTEXT]`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates living document for iterative discussion
- Contains: DISCUSSION LOG, ACCUMULATED CONTEXT, PLANNING READINESS CHECKLIST

**Task Issue Template** - `/docs/TASK-ISSUE-TEMP.md`:

- Used for: `=plan > [task description]`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates atomic tasks based on current mode (MANUAL/COPILOT)
- Contains: EXECUTION MODE field, 100% validation requirements

**Knowledge Issue Template** - `/docs/KNOWLEDGE-TEMP.md`:

- Used for: `=kupdate [category] "[topic]"`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates structured knowledge entries with AI honest feedback
- Contains: Problem → Solution → Lessons Learned → Links

### Mode-Based Execution System

**Default Mode**: MANUAL (human implementation)

**Mode Commands**:

```bash
=mode manual     # Tasks assigned to human developer
=mode copilot     # Tasks assigned to @copilot
=mode status      # Show current execution mode
```

**Mode-Specific Behavior**:

- **MANUAL Mode**: `=plan` creates tasks assigned to human, `=impl` waits for human implementation
- **COPILOT Mode**: `=plan` creates tasks assigned to @copilot, `=impl` triggers copilot implementation

### Core Commands

```bash
# Context Management
=fcs > [topic-name]           # Create new Context GitHub Issue (NEVER .md file)
=fcs > [CONTEXT]            # Update existing Context GitHub Issue (NEVER .md file)
=fcs list                     # Show all active Context Issues

# Task Management
=plan > [task description]      # Create Task GitHub Issue using /docs/TASK-ISSUE-TEMP.md (assigned by current mode) - NEVER .md file
=impl > [issue-number]         # Implementation workflow for specific GitHub issue (triggers based on current mode)
=impl > [issue-number] [msg]   # Implementation with additional context/clarification
=pr > [feedback]               # Create Pull Request from pushed feature branch (ALWAYS to staging, NEVER to main)

# Knowledge Management
=khub                          # 🔍 ALWAYS read Knowledge Hub #102 FIRST before creating knowledge entries
=kupdate [category] "[topic]"   # Create Knowledge GitHub Issue using /docs/KNOWLEDGE-TEMP.md (NEVER .md file) - CHECK existing numbers first!
=klink [knowledge-issue-number] # Link knowledge entry to Knowledge Hub #102 (automatic section placement)
=ksync                          # Synchronize Knowledge Hub #102 with all knowledge entries
=ksearch "[query]"              # Search across all knowledge entries
=krecent                       # Show last 5 knowledge updates
=kcategory [category]           # Show knowledge for specific category

# Other Commands
=rrr > [message]              # Create daily retrospective GitHub Issue (NEVER .md file)
```

### Template-Driven Workflow Process

1. **Phase 1**: `=fcs > [topic]` → Create initial context **GitHub Issue** (NEVER .md file)
2. **Phase 2**: `=fcs > [CONTEXT]` → Update context **GitHub Issue** iteratively
3. **Phase 3**: Context reaches `[Ready for Planning]` status → Ready for planning
4. **Phase 4**: `=plan > [task]` → Create atomic **GitHub Issues** (NEVER .md files)
5. **Phase 5**: `=impl > [issue-number]` → Implement specific GitHub issue based on mode

### Implementation Workflow (MANDATORY)

**Pre-Implementation Checklist**:

1. **Staging Sync**: `git checkout staging && git pull origin staging`
2. **Task Verification**: Confirm Task **GitHub Issue** `#[issue-number]` exists and is [TASK] type
3. **Context Status**: Verify Context **GitHub Issue** is `[Ready for Planning]` or `[Implementation Ready]`
4. **Environment Check**: `git status` - working directory must be clean

**Implementation Steps**:

1. **Create Feature Branch**: `git checkout -b feature/task-[issue-number]-[description]`
2. **Execute Implementation**: Follow task requirements, use TodoWrite for complex tasks
3. **Debug with Enhanced VS Code Setup**:
   - Use "Debug Main Process (Launch)" for hardware communication debugging
   - Set breakpoints in SerialPort and IPC handler code
   - Monitor real-time console output in VS Code debug console
   - Test CU12/KU16 communication with live debugging
4. **Quality Validation**: `npm run build` (100% pass) + `npm run lint` (100% pass) + `npx tsc --noEmit`
5. **Commit Changes**:

   ```bash
   git add .
   git commit -m "feat: [feature description]

   - Address #[issue-number]: [task title]
   - Build validation: 100% PASS
   - Linter validation: 100% PASS

   🤖 Generated with Claude Code
   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

6. **Push Branch**: `git push -u origin feature/task-[issue-number]-[description]`

**Post-Implementation**:

- **MANUAL Mode**: User commits and pushes, then uses `=pr` to create PR
- **COPILOT Mode**: Agent handles complete implementation including PR creation via `=pr`

---

## 🧠 Knowledge Management System

### Knowledge Workflow Integration

**Knowledge Capture Points**:

- **After Implementation**: When `=impl` completes successfully, use `=kupdate` to document learnings **(auto-prompts for hub linking)**
- **After Context Discussion**: When `=fcs` reaches key decisions, use `=kupdate` to capture insights **(auto-prompts for hub linking)**
- **After Chat Discoveries**: When breakthrough solutions are found, use `=kupdate` to preserve knowledge **(auto-prompts for hub linking)**

**Enhanced Knowledge Workflow**:

1. **🔍 Pre-Creation Check**: `=khub` → Read Knowledge Hub #102 FIRST to check existing KNOW-[CATEGORY]-XXX numbers
2. **Verify**: Check category section for existing numbers to avoid duplicates (e.g., KNOW-DEVICE-001, KNOW-DEVICE-002)
3. **Create**: `=kupdate [category] "[topic]"` → Creates knowledge issue with next available number
4. **Prompt**: System asks "Link to Knowledge Hub #102? (y/n)"
5. **Link**: If "y" → Automatically runs `=klink`
6. **Sync**: Use `=ksync` to ensure hub is fully synchronized
7. **Discover**: All knowledge accessible through `=khub` navigation

### Knowledge Categories

**Standard Categories**:

- `device` - CU12, KU16, SerialPort, hardware integration
- `database` - SQLite, Sequelize, migrations, queries
- `architecture` - Design patterns, structural decisions
- `debug` - Error solutions, troubleshooting, workarounds
- `workflow` - Process improvements, automation
- `frontend` - React, Electron, UI components
- `backend` - Node.js, APIs, services

### Knowledge ID System

**Format**: `KNOW-[CATEGORY]-[NUMBER]`

- Example: `KNOW-DEVICE-001`, `KNOW-DATABASE-015`
- Auto-increment per category
- Easy reference and cross-linking

### 🔍 Knowledge ID Conflict Prevention (CRITICAL)

**MANDATORY Pre-Creation Checklist**:

1. **ALWAYS run `=khub` first** - Read Knowledge Hub #102 completely
2. **Check existing numbers** in your category section (e.g., "Device Knowledge")
3. **Identify next available number** (if 001, 002 exist, use 003)
4. **Never assume** - always verify existing entries before creating

**Common Mistakes to Avoid**:

- ❌ Creating KNOW-DEVICE-001 when it already exists
- ❌ Not checking Knowledge Hub #102 before creating entries
- ❌ Assuming numbers without verification
- ❌ Creating duplicate knowledge IDs

**Correct Workflow Example**:

```bash
# ❌ WRONG (creates duplicate)
= kupdate device "SHT30 sensor fix"  # Creates KNOW-DEVICE-001 (duplicate!)

# ✅ RIGHT (prevents duplicates)
= khub                              # Read Knowledge Hub #102
# See: KNOW-DEVICE-001, KNOW-DEVICE-002 exist
= kupdate device "SHT30 sensor fix" # Creates KNOW-DEVICE-003 (correct!)
```

### Auto-Label Creation

**System Behavior**:

```bash
# When =kupdate device "CU12 lock-back solution" is used:
# 1. Check if 'knowledge-device' label exists
# 2. If not, create: gh label create knowledge-device --color "1d76db" --description "Device integration knowledge"
# 3. Apply label to knowledge issue
# 4. Auto-generate Knowledge ID: KNOW-DEVICE-001
```

**Knowledge Labels Created Automatically**:

- `knowledge-device` - Device integration knowledge
- `knowledge-database` - Database and persistence knowledge
- `knowledge-architecture` - System design and patterns
- `knowledge-debug` - Debugging and troubleshooting
- `knowledge-workflow` - Development workflow improvements

### Enhanced Knowledge Hub Integration

**New Automated Commands**:

**`=klink [knowledge-issue-number]`**:

- Automatically detects category from knowledge issue labels
- Places knowledge link in appropriate Knowledge Hub section
- Updates statistics counters
- Maintains proper markdown formatting

**`=ksync`**:

- Scans all issues with `knowledge-*` labels
- Synchronizes Knowledge Hub with all existing knowledge entries
- Updates statistics and distribution
- Fixes broken links and formatting
- Ensures hub reflects current knowledge base state

**Enhanced `=kupdate` Workflow**:

1. Creates knowledge GitHub issue ✅
2. **Automatically prompts**: "Link to Knowledge Hub #102? (y/n)"
3. If "y": Runs `=klink` automatically ✨
4. Maintains consistency across knowledge system

**Command Implementation Details**:

**`=klink [issue-number]` Implementation**:

1. **Issue Analysis**: Extract title, labels, and description
2. **Category Detection**: Parse `knowledge-[category]` label
3. **Format Entry**: `**KNOW-[CATEGORY]-[NUMBER]**: [Title](issue-link) - Brief description`
4. **Section Insert**: Add to appropriate "Recent Entries" section
5. **Statistics Update**: Increment total and category counts
6. **Timestamp Update**: Set "Last Updated" to current date

**`=ksync` Implementation**:

1. **Knowledge Discovery**: Scan all issues with `knowledge-*` labels
2. **Category Processing**: Group by label type (device, database, etc.)
3. **Entry Generation**: Create standardized format for each found issue
4. **Hub Reconstruction**: Replace all category sections with complete lists
5. **Statistics Calculation**: Recalculate all counts from scratch
6. **Format Validation**: Ensure proper markdown structure and valid links

**Hub Integration Benefits**:

- ✅ **No more manual linking required**
- ✅ **Automatic statistics updates**
- ✅ **Consistent formatting maintained**
- ✅ **Centralized knowledge discovery**
- ✅ **Real-time hub synchronization**

### Knowledge Search & Retrieval

**Search Capabilities**:

```bash
=ksearch "CU12 lock-back"    # Full-text search across all knowledge
=kcategory device           # Show all device-related knowledge
=krecent                    # Last 5 knowledge entries
=khub                       # Go to main Knowledge Hub issue
=ksync                      # Synchronize hub with all knowledge entries
=klink 116                  # Link knowledge issue #116 to hub
```

**Search Optimization**:

- Knowledge entries include searchable tags
- Problem statements use clear, technical language
- Solutions include specific keywords and technologies
- Cross-references link related knowledge
- Hub ensures all knowledge is discoverable from central location

### Knowledge Structure

**Each Knowledge Entry Contains**:

- **Problem Statement**: Clear description of what was solved
- **Solution Implementation**: Step-by-step working solution
- **AI Honest Feedback**: What worked, what didn't, lessons learned
- **Things to Avoid**: Common pitfalls and their consequences
- **Prerequisites**: What to check before starting
- **AI Self-Improvement**: Insights for future problem-solving
- **Links & References**: Connections to source issues/PRs/code
- **Verification Status**: Testing and validation state

---

## 🏗️ Technical Architecture

### Core Stack

- **Language & Runtime**: Rust (edition 2021)
- **Primary crates**: `printpdf` (PDF generation), `csv` (CSV parsing), `chrono` (date/time), `serde` (serialization), `font-kit` (font handling), `reqwest` (optional HTTP), `log` / `env_logger` (logging)
- **Project type**: Command-line utility that reads CSV files and writes PDF files (no web UI or Electron)

### Project Structure

```
./
├── Cargo.toml               # Cargo manifest (dependencies and metadata)
├── src/
│   ├── main.rs              # Program entry: reads CSV and calls PDF generator
│   ├── csv_util.rs          # CSV parsing helpers
│   ├── pdf_util.rs          # PDF creation and layout
│   ├── drawing.rs           # Low-level drawing utilities
│   ├── font_util.rs         # Font loading and metrics helpers
│   ├── model.rs             # Domain models for bill records
│   └── log.rs               # Logging initialization and helpers
├── input-csv/               # Example CSV inputs used by the program
├── bills/                   # Output PDFs generated by the tool
├── docs/                    # Documentation and issue templates
└── target/                  # Cargo build artifacts
```

### Data model and behavior

- The program reads CSV rows into strongly-typed Rust structs defined in `model.rs` (using `serde` derives), formats data for a target month, and renders a PDF via `printpdf`.
- Fonts are managed via `font_util` and `font-kit` so Thai script and custom fonts render correctly.
- Logging is initialized at startup (`log::init_logger`) and used for info/error telemetry.

### Key Features

- CSV input parsing and validation
- Multi-page PDF generation with custom layout and Thai-language text
- Reusable drawing primitives (lines, text blocks, table-like layouts)
- Simple CLI usage: choose input CSV and output path via constants or command-line arguments (currently hard-coded in `main.rs`)

### Development & Run Commands

```bash
# Build
cargo build --release

# Run (uses example CSV paths from repository)
cargo run --release

# Test (if tests exist)
cargo test
```

Notes: this project is a Rust CLI. There is no `npm`/TypeScript toolchain used. Update any workflow steps that reference `npm` commands when working with this repository.

### Debugging & Environment

- Use `RUST_LOG=info` (or `debug`) with `cargo run` to enable runtime logging via `env_logger`.
- For quick iteration, run `cargo run` which will recompile and execute the program. Use `cargo build` then run the produced binary from `target/debug/`.
- There are no external secrets in the repository. CSV input paths and output paths are local files under `input-csv/` and `bills/`.

---

## 🎯 Quality Standards

### Code Quality Requirements

- **TypeScript**: Strict mode enabled
- **ESLint**: Zero violations allowed
- **Prettier**: Consistent formatting
- **Build**: 100% success rate (zero errors/warnings)
- **Tests**: 100% pass rate when implemented

### UI/UX Requirements

- **Desktop-First**: Electron desktop application optimization
- **Accessibility**: WCAG 2.1 AA compliance (4.5:1 contrast)
- **Clear Visual Indicators**: Medication slot status and device states
- **Performance**: Fast startup and responsive UI for healthcare environment
- **Error Prevention**: Clear feedback and validation for medication safety

### Template-Guided Quality

- **Context Issues**: Complete PLANNING READINESS CHECKLIST ✅ (Always GitHub Issues)
- **Task Issues**: 100% build/lint/test requirements mandatory (Always GitHub Issues)
- **Mode Execution**: Follow mode-specific behavior exactly
- **Template Consistency**: All issues follow template structures
- **File Policy**: NEVER create local .md files for issues - ALWAYS use GitHub Issues

---

## 📚 Reference Materials

### Templates

- `/docs/ISSUE-TEMP.md` - Context Issue Template for iterative discussions
- `/docs/TASK-ISSUE-TEMP.md` - Atomic Task Template for implementation
- `/docs/KNOWLEDGE-TEMP.md` - Knowledge Issue Template for structured learning

### Performance Metrics

- **Target**: Desktop app startup within ≤5 seconds
- **Goal**: 99.9% uptime for medication dispensing operations
- **Reliability**: 99.99% accurate medication slot tracking
- **Database**: Support local SQLite operations with minimal latency

### Security Notes

- **Input Validation**: Comprehensive validation for all user inputs
- **Authentication**: Role-based access control with passkey system
- **Data Protection**: Local SQLite database with secure storage
- **Access Control**: Role-based permissions (Admin, Pharmacist, Nurse)
- **Device Security**: Secure serial communication with medical devices
- **Audit Trail**: Complete dispensing logs for compliance

---

_This document focuses on agent-critical information for efficient workflow execution and safe development practices._