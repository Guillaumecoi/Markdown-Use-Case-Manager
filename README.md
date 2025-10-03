
<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=soft&color=0:667eea,100:764ba2&height=160&section=header&text=Markdown%20Use%20Case%20Manager&fontSize=42&fontColor=ffffff&fontAlignY=40&desc=Documentation%20that%20travels%20with%20your%20code&descSize=26&descAlignY=70" style="border-radius: 25px;">
</div>

## Why This Tool?

Most use case management happens in external tools like Jira or Confluence, which creates a disconnect between your documentation and code. This tool keeps everything together in your repository as plain markdown files.

Your use cases live alongside your code, version-controlled and readable by anyone. No external dependencies, no cloud services, no vendor lock-in. Just markdown files that work with any static site generator or documentation platform.

Works great for solo developers, small teams, or any project where you want documentation that travels with your code.

## Features

- **Interactive CLI Mode** - Beautiful terminal interface with arrow key navigation and guided workflows
- **Script-Friendly** - Perfect for automation with all commands available in both modes
- **Automatic ID Generation** - Organize use cases by categories with automatic ID generation
- **Progress Tracking** - Track progress from planning to deployment
- **Documentation Generation** - Generate consistent documentation and test scaffolding  
- **Intelligent Naming** - Prevent conflicts with intelligent naming
- **Flexible Export** - Export to any markdown-compatible format
- **Test Generation** - Support automatic test generation
- **Custom Templates** - Customizable generation templates
- **Flexible Configuration** - Flexible configuration
- **Extended Metadata** - Comprehensive metadata including personas, prerequisites, business value, acceptance criteria, and more
- **Interactive CLI** - User-friendly interactive mode with guided workflows
- **Use Case Dependencies** - Reference and link related use cases for traceability

## Getting Started

### System Installation

```bash
git clone https://github.com/GuillaumeCoi/markdown-use-case-manager
cd markdown-use-case-manager
cargo install --path .            # Don't forget the dot at the end
```

Now you can run the tool with `mucm` from anywhere.

### Interactive Mode

For the best user experience, use interactive mode:

```bash
mucm -i                          # Start interactive mode
```

The interactive mode provides:
- **🔧 Guided use case creation** with optional extended metadata
- **📋 Extended metadata management** for existing use cases  
- **⚙️ Settings configuration** with auto-initialization
- **📊 Project status overview** and management
- **✨ Auto-initialization** for new projects

### Basic Usage

#### Interactive Mode (Recommended)

```bash
# Launch interactive mode
mucm interactive               # or mucm -i
```

![interactive terminal screenshot](images/interactive.png)

The interactive mode provides:
- **Smart category suggestions** from existing use cases
- **Step-by-step workflows** for creating use cases and scenarios
- **Auto-completion** for use case and scenario selection
- **Visual feedback** with colors and clear prompts

#### Script Mode (Perfect for Automation)

```bash
# Initialize your project
mucm init

# Create your first use case  
mucm create "User Login" --category "Security"

# Add scenarios to your use case
mucm add-scenario "UC-SEC-001" "Login with email and password"

# Update scenario status
mucm update-status "UC-SEC-001-S01" --status "implemented"

# View your documentation
mucm list
mucm status
```

### What You Get

Creating use cases generates a clean file structure:

```
docs/use-cases/
├── README.md                    # Auto-generated overview
├── security/
│   └── UC-SEC-001.md           # Individual use case 
└── ...

tests/use-cases/
├── security/
│   └── uc_sec_001.rs           # Test scaffolding 
└── ...
```

Everything is standard markdown with YAML frontmatter, so it works with any static site generator.

## Status Tracking

Six development statuses that automatically roll up from scenarios to use cases:

```
PLANNED 📋      → Basic idea documented
IN_PROGRESS 🔄  → Development started
IMPLEMENTED ⚡  → Code complete, not tested
TESTED ✅       → Tested and verified
DEPLOYED 🚀     → Live in production
DEPRECATED ⚠️   → No longer maintained
```

The use case status automatically reflects the minimum status of all its scenarios.

## Extended Metadata

Rich metadata support for professional documentation:

### **Available Fields**
- **👥 Personas** - Target users and stakeholders
- **📋 Prerequisites** - System requirements and dependencies  
- **✍️ Author/Reviewer** - Ownership and review information
- **💰 Business Value** - Why this use case matters
- **🔧 Complexity** - Implementation difficulty assessment
- **📦 Epic** - Project/epic association
- **✅ Acceptance Criteria** - Definition of "done"
- **💭 Assumptions & ⚠️ Constraints** - Context and limitations

### **Use Case Dependencies**
Reference related use cases in prerequisites:
```markdown
## Prerequisites
- User must be logged in (UC-AUTH-001)
- Payment method configured (UC-PAY-003)
- Shopping cart not empty (UC-CART-002)
```

### **Professional Output**
```markdown
# UC-AUTH-001: User Authentication

**Author:** John Doe | **Reviewer:** Jane Smith
**Target Users:** Customer, Admin User

## Business Value
Secure authentication improves user trust and reduces support tickets

## Prerequisites
- System is online
- User registration completed (UC-REG-001)

## Acceptance Criteria
- Login completes within 5 seconds
- Multi-factor authentication supported
```

See [Extended Metadata Guide](docs/EXTENDED_METADATA_GUIDE.md) for detailed usage.

## Configuration

Configure the tool via `.config/.mucm/mucm.toml`:

```toml
[project]
name = "My Project"
description = "Project managed with Markdown Use Case Manager"

[directories]
use_case_dir = "docs/use-cases"
test_dir = "tests/use-cases"

[generation]
test_language = "rust"        # rust, python, or none
auto_generate_tests = true

...
```

## Customization 🎨

The tool is designed to be flexible and adapt to your workflow:

**Custom Templates**: All documentation and test templates are stored in `.config/.mucm/templates/` and can be modified:
- `use_case_simple.hbs` - Basic use case format
- `use_case_detailed.hbs` - Detailed use case with scenarios
- `overview.hbs` - Auto-generated overview page
- `{language}/test.hbs` - Test scaffolding for your chosen language

## Deployment

Since everything is just markdown, your documentation works everywhere:

- **GitHub/GitLab Pages** - Automatic deployment from your repo
- **MkDocs** - `mkdocs serve` for instant documentation sites  
- **Docusaurus** - Modern documentation platform
- **Jekyll** - GitHub's default static site generator
- **Hugo** - Fast static site generator
- **Any markdown processor** - Pandoc, GitBook, etc.

## Contributing

Issues and pull requests welcome!

## License

MIT License - see [LICENSE](LICENSE) for details.