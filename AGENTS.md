# AGENTS.md - Development Guidelines

## Project Overview

**Papa Moo 3 Bills** is a Rust-based plumbing billing system that converts CSV meter reading data into professional PDF receipts in Thai. The system serves as a specialized billing solution for water utility management in residential communities.

### Core Functionality
- CSV data parsing and validation
- Thai font management and rendering
- PDF generation with structured bill layouts
- Multi-bill per page optimization (2 bills per A5 page)

## Coding Philosophy

### 1. Clean Code First
- **Readability**: Code should be self-documenting where possible
- **Simplicity**: Prefer simple solutions that are easy to understand
- **Consistency**: Maintain consistent naming conventions and patterns
- **Maintainability**: Future changes should be straightforward and predictable

### 2. Modular Architecture
Each module has a **single, well-defined responsibility**:

#### Core Modules
- **`main.rs`** - Application entry point and orchestration only
- **`model.rs`** - Data structures and domain models
- **`csv_util.rs`** - CSV parsing and data extraction logic
- **`pdf_util.rs`** - PDF generation and layout orchestration
- **`font_util.rs`** - Font discovery, downloading, and management
- **`drawing.rs`** - Drawing primitives and visual elements
- **`log.rs`** - Centralized logging configuration and utilities

### 3. Separation of Concerns
```
Data Layer:     model.rs, csv_util.rs
Business Layer: pdf_util.rs (main logic)
Presentation:   drawing.rs, font_util.rs
Infrastructure: log.rs, main.rs
```

- **Data concerns** stay in data modules
- **Business logic** stays in core processing modules  
- **Presentation/visual** logic stays in drawing/font modules
- **Infrastructure** concerns (logging, orchestration) stay separate

### 4. DRY (Don't Repeat Yourself) Principles
- **Extract common patterns**: Repeated code should become functions
- **Share constants**: Magic numbers and strings become named constants
- **Template patterns**: Similar operations should use shared helper functions
- **Configuration-driven**: Hard-coded values should be configurable where practical

## Module Responsibilities

### `model.rs` - Data Domain
**Single Responsibility**: Define all data structures used across the application
```rust
// Domain models only - no business logic
#[derive(Debug, Deserialize)]
pub struct BillRecord { ... }
```

**Rules**:
- Only data structure definitions
- No processing logic
- Deserialization attributes only

### `csv_util.rs` - Data Input
**Single Responsibility**: Parse and validate CSV input data
```rust
pub fn read_csv_file(file_path: &str) -> Result<Vec<BillRecord>, Box<dyn Error>>
```

**Rules**:
- Only CSV-specific logic
- Validation and error handling for input data
- Returns clean data structures for other modules

### `pdf_util.rs` - Business Logic Core
**Single Responsibility**: Orchestrate PDF generation from bill data
```rust
pub fn create_pdf(records: &[BillRecord], output_path: &str, for_month: &str) -> Result<(), Box<dyn Error>>
```

**Rules**:
- Main business logic coordinator
- Calls other modules for specific operations
- Layout and positioning calculations
- Document structure management

### `drawing.rs` - Visual Primitives  
**Single Responsibility**: Create reusable drawing elements
```rust
pub fn draw_line(y: Mm, start: Mm, end: Mm) -> Line
pub fn draw_vetical_line(x: Mm, start: Mm, end: Mm) -> Line  
pub fn draw_bill_split_line() -> Line
```

**Rules**:
- Pure drawing functions only
- No business logic
- Reusable visual components
- Position and style calculations only

### `font_util.rs` - Font Management
**Single Responsibility**: Handle font discovery and loading
```rust
pub fn find_thai_font(doc: &PdfDocumentReference) -> (IndirectFontRef, IndirectFontRef, IndirectFontRef)
pub fn download_font(font_name: &str, style: &str) -> Result<PathBuf, Box<dyn Error>>
```

**Rules**:
- Font discovery and downloading logic only
- No positioning or drawing logic
- Caching and system integration

### `log.rs` - Infrastructure
**Single Responsibility**: Centralized logging configuration
```rust
pub fn init_logger()
pub fn log_info(message: &str)  // etc.
```

**Rules**:
- Logger setup and configuration only
- Convenience wrapper functions
- No business logic

## Code Style Guidelines

### 1. Naming Conventions
```rust
// Functions: snake_case
pub fn read_csv_file() -> Result<...>

// Types: PascalCase  
pub struct BillRecord { ... }

// Constants: SCREAMING_SNAKE_CASE
const DEFAULT_FONT_SIZE: f64 = 12.0;

// Variables: descriptive snake_case
let table_header_y = y_offset - Mm(35.0);
```

### 2. Error Handling
- **Consistent Result types**: `Result<T, Box<dyn Error>>` for most functions
- **Proper error propagation**: Use `?` operator consistently
- **Descriptive error messages**: Include context about what failed
```rust
pub fn create_pdf(...) -> Result<(), Box<dyn Error>> {
    log::log_info("เริ่มสร้าง PDF...");
    // ... logic
    doc.save(...)?;  // Proper error propagation
    log::log_info("บันทึกไฟล์ PDF สำเร็จ!");
    Ok(())
}
```

### 3. Function Organization
```rust
// Public functions first
pub fn main_function() -> Result<...> {
    // High-level logic
}

// Private helper functions after
fn calculate_position(...) -> Mm {
    // Detail implementation
}

// Constants at module level
const DEFAULT_HEIGHT: Mm = Mm(210.0);
```

### 4. Comment Guidelines
- **No unnecessary comments**: Code should be self-documenting
- **Complex logic only**: Comment non-obvious algorithms
- **Thai comments**: Where relevant for domain-specific content
- **Function documentation**: For public APIs

## Development Workflow

### 1. Starting New Features
1. **Identify the responsible module** - Should this go in existing code or new module?
2. **Review module boundaries** - Are we violating separation of concerns?
3. **Write the public interface first** - What's the expected input/output?
4. **Implementation details** - Write the actual logic
5. **Test the interface** - Ensure it works with calling code

### 2. Code Review Checklist
- [ ] Does this change respect module boundaries?
- [ ] Is there duplicate code that could be extracted?
- [ ] Are error messages descriptive and helpful?
- [ ] Are naming conventions followed?
- [ ] Is the code readable without extensive comments?
- [ ] Are hard-coded values extracted to constants?

### 3. Refactoring Guidelines
- **When to refactor**: Code duplication, complex functions, unclear module boundaries
- **Small steps**: Make incremental changes that maintain functionality
- **Test after each change**: Ensure the system still works
- **Update documentation**: Keep this file in sync with actual practices

## Quality Assurance

### 1. Code Quality Standards
- **Function length**: Functions should fit on one screen when possible
- **Complexity**: Avoid deeply nested logic (>3 levels)
- **Dependencies**: Minimize cross-module dependencies
- **Memory**: Prefer references and borrowing over cloning

### 2. Error Handling Quality
- **Never use unwrap()** in production code except when impossible to fail
- **Always provide context** in error messages
- **Handle recoverable errors** gracefully
- **Log errors appropriately** before returning them

### 3. Performance Considerations
- **No premature optimization** - clarity first
- **Profile before optimizing** - measure actual bottlenecks
- **Consider memory usage** with large CSV files
- **Efficient PDF generation** - batch operations when possible

## Testing Philosophy

### 1. Manual Testing Process
Since this is a solo project, focus on systematic manual testing:
```bash
# Test different scenarios
cargo run    # Normal operation
cargo run    # Test with different CSV files
cargo run    # Test font fallback scenarios
```

### 2. Test Data Management
- **Use representative data**: Test with actual CSV formats
- **Edge cases**: Empty files, malformed data, special characters
- **Thai text**: Ensure proper rendering of Thai fonts
- **Large datasets**: Test performance with many bills

### 3. Quality Checkpoints
- **CSV parsing**: Does it handle all required fields?
- **PDF generation**: Does output match expected format?
- **Font rendering**: Are Thai characters displayed correctly?
- **Error messages**: Are they clear and actionable?

## Contributing Guidelines

### For Future Collaborators
1. **Read this document first** - Understand the architectural principles
2. **Start with small changes** - Get familiar with the codebase structure
3. **Respect module boundaries** - Don't mix concerns across modules
4. **Follow existing patterns** - Look at similar functions for guidance
5. **Ask questions** - When in doubt, prioritize clarity

### Code Submission Process
1. **Test locally** - Ensure your changes work with real data
2. **Review module boundaries** - Verify separation of concerns
3. **Check for duplication** - Look for DRY principle violations
4. **Verify error handling** - Ensure proper Result usage
5. **Update relevant documentation** - Keep this file current

## Project-Specific Considerations

### Thai Language Support
- **Font handling is critical** - Users expect proper Thai rendering
- **Testing with actual Thai text** - Don't assume fonts work without testing
- **Fallback behavior** - System should degrade gracefully when fonts fail

### PDF Layout Requirements
- **Precise positioning** - Bill layout requires exact positioning
- **A5 page format** - Two bills per page is a hard requirement
- **Professional appearance** - Bills are customer-facing documents

### Data Processing
- **CSV format consistency** - Input format is relatively stable
- **Data validation** - Ensure all required fields are present
- **Numeric calculations** - Bill calculations must be accurate

---

*This document should evolve with the project. Update it when patterns change or new conventions are established.*
