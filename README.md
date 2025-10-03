
<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=soft&color=0:667eea,100:764ba2&height=160&section=header&text=Markdown%20Use%20Case%20Manager&fontSize=42&fontColor=ffffff&fontAlignY=40&desc=Documentation%20that%20travels%20with%20your%20code&descSize=26&descAlignY=70" style="border-radius: 25px;">
</div>

## Why This Tool?

Most use case management happens in external tools like Jira or Confluence, which creates a disconnect between your documentation and code. This tool keeps everything together in your repository as plain markdown files.

Your use cases live alongside your code, version-controlled and readable by anyone. No external dependencies, no cloud services, no vendor lock-in. Just markdown files that work with any static site generator or documentation platform.

Works great for solo developers, small teams, or any project where you want documentation that travels with your code.

## Features

- Organize use cases by categories with automatic ID generation
- Track progress from planning to deployment
- Generate consistent documentation and test scaffolding  
- Prevent conflicts with intelligent naming
- Export to any markdown-compatible format
- Support automatic test generation
- Customizable generation templates
- Flexible configuration

## Getting Started

### System Installation

```bash
git clone https://github.com/GuillaumeCoi/markdown-use-case-manager
cd markdown-use-case-manager
cargo install --path .            # Don't forget the dot at the end
```

Now you can run the tool with `mucm` from anywhere.

### Basic Usage

```bash
# Initialize your project
mucm init

# Create your first use case  
mucm create "User Login" --category "Security"

# Add scenarios to your use case
mucm add-scenario "UC-SEC-001" "Login with email and password"

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