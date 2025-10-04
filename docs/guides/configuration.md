# Configuration Guide

Let's set up MUCM exactly how you want it! Your settings live in a file called `mucm.toml`.

## Where's My Config File?

After you run `mucm init`, you'll find your settings here:
`.config/.mucm/mucm.toml`

## Basic Setup

Here's what a simple config looks like:

```toml
[project]
name = "My Awesome Project"
description = "What this project is about"
author = "Your Name"

[templates]
methodology = "simple"              # Which style to use

[directories]
use_case_dir = "docs/use-cases"     # Where your use cases go
```

## The Main Settings

### Project Info
```toml
[project]
name = "My Project"                 # Shows up in your docs
description = "A cool project"      # What it's about
version = "1.0.0"                  # Version number
author = "You!"                    # Who made it
```

### Pick Your Style
```toml
[templates]
methodology = "simple"              # Options: simple, cockburn, unified_process, bdd_gherkin
```

**Which one should you pick?**
- `simple` - You want something quick and easy
- `cockburn` - You need detailed business analysis
- `unified_process` - Your company has formal requirements
- `bdd_gherkin` - You write lots of automated tests

### Folder Setup
```toml
[directories]
use_case_dir = "docs/use-cases"     # Your use cases go here
test_dir = "tests/use-cases"        # Test files go here (if you want them)
```

### File Naming
```toml
[formatting]
id_format = "UC-{category}-{number:03}"     # How IDs look: UC-AUTH-001
filename_format = "{id}.md"                 # How files are named
```

**Want different IDs?**
- `"UC-{category}-{number:03}"` → `UC-AUTH-001`
- `"UC{number:04}"` → `UC0001` 
- `"{category}-{number}"` → `AUTH-1`

### Test Generation
```toml
[generation]
test_language = "rust"                      # Or "python" or "none"
auto_generate_tests = true                  # Create test files automatically
```

## Common Setups

### I Just Want Simple
```toml
[templates]
methodology = "simple"

[generation]
test_language = "none"                      # No test files
```

### I'm Building Business Software
```toml
[templates]
methodology = "cockburn"
use_extended_metadata = true               # More detailed fields

[generation]
test_language = "python"
```

### I Work at a Big Company
```toml
[templates]
methodology = "unified_process"
use_extended_metadata = true

[validation]
enforce_required_fields = true             # Make sure all fields are filled
```

### I Love Testing Everything
```toml
[templates]
methodology = "bdd_gherkin"

[generation]
test_language = "python"
auto_generate_tests = true
test_framework = "pytest"
```

## Need Help?

**Broke something?** Run `mucm status` to check if your config is valid.

**Want to start over?** Delete the `.config` folder and run `mucm init` again.

**Can't decide?** Use `mucm -i` and it'll help you pick the right settings.