# Getting Started with MUCM

Hey there! Let's get you creating awesome use cases in just a few minutes.

## Step 1: Install MUCM

First, let's get MUCM on your computer:

```bash
git clone https://github.com/GuillaumeCoi/markdown-use-case-manager
cd markdown-use-case-manager
cargo install --path .            # Don't forget the dot!
```

## Step 2: Pick Your Methodology

MUCM has 3 different methodologies. Pick one that fits your project:

| What's your situation? | Pick this methodology | Command to use |
|------------------------|----------------------|----------------|
| I want something simple and quick | **Simple** - Lightweight, flexible approach for rapid development | `mucm init --methodology simple` |
| I need detailed business analysis | **Business** - Business-focused approach emphasizing stakeholder value | `mucm init --methodology business` |
| I'm focused on testing and quality | **Testing** - Test-driven approach with comprehensive coverage | `mucm init --methodology testing` |

**Can't decide?** Start with `simple` - it's perfect for most projects and you can always regenerate with a different methodology later.

### Storage Backend Options

MUCM supports two storage backends:

- **TOML (default)**: Human-readable files, great for version control, perfect for < 100 use cases
- **SQLite**: Database storage, better performance for 100+ use cases, supports complex queries

Start with TOML unless you know you'll have a large project. You can always migrate later if needed.

## Step 3: Set Up Your Project

```bash
# Set up with your chosen methodology (uses TOML storage by default)
mucm init --methodology simple

# Or use SQLite for better performance with large projects
mucm init --methodology simple --backend sqlite

# Or let MUCM guide you through it
mucm -i
```

## Step 4: Create Your First Use Case

```bash
# Quick way
mucm create "User Login" --category "Security" --methodology simple

# Let MUCM ask you questions (easier!)
mucm -i
```

## What You'll Get

MUCM creates a nice, organized folder structure for you:

```
your-project/
├── .config/.mucm/
│   ├── mucm.toml                    # Your settings
│   └── templates/                   # Templates you can customize
├── docs/use-cases/
│   ├── README.md                    # Overview page (auto-generated)
│   └── security/
│       └── UC-SEC-001.md           # Your use cases go here
└── tests/use-cases/
    └── security/
        └── uc_sec_001.rs           # Test files (if you want them)
```

## Working with Actors and Personas

MUCM supports two ways to represent users in your scenarios:

### Quick Start: Actors

**Actors** are simple role names used in scenario steps. Use these for straightforward use cases:

```bash
# Actors are defined inline in scenarios using the Actor enum
# Examples: User, Admin, System, Guest
```

When writing scenarios, you can use predefined actors like `User`, `Admin`, `System`, or create custom ones.

### Advanced: Personas

**Personas** are detailed user archetypes with goals, technical levels, and context. Create these when you need:
- Detailed user profiles for documentation
- Understanding different user skill levels
- Tailoring scenarios to specific user types

```bash
# Create a persona
mucm persona create "Power User Sarah" \
  --description "Experienced user who needs advanced features" \
  --tech-level 8 \
  --goals "Increase efficiency,Automate workflows"

# List personas
mucm persona list

# Reference personas in your scenarios by ID
```

**Personas vs Actors:**
- **Actors** are simple roles (`User`, `Admin`) used directly in scenario steps
- **Personas** are rich user profiles stored separately and referenced in scenarios
- Use actors for quick scenarios, personas when you need detailed user context

### Interactive Mode

The easiest way to create personas:

```bash
mucm -i
# Choose "Persona Management" → "Create New Persona"
```

## What's Next?

Now that you're set up, here are some good next steps:

1. **Learn the methodologies** - Use `mucm methodology-info <name>` to understand each approach
2. **Try different methodologies** - Use `mucm regenerate UC-XXX-001 --methodology business` to see the differences
3. **Create personas** - Define your user archetypes with `mucm persona create` or `mucm -i`
4. **Customize your setup** - Read the [configuration guide](configuration.md) to make it yours
5. **Get better at writing** - See our [best practices](best-practices.md) for tips
6. **Connect with other tools** - Look at [integration](integration.md) for CI/CD and websites

## Need Help?

- **Stuck?** Try `mucm -i` - it'll ask you questions and guide you through
- **Want to see methodologies?** Use `mucm methodologies` to list them and `mucm methodology-info <name>` for details
- **Found a bug?** Let us know on [GitHub](https://github.com/GuillaumeCoi/markdown-use-case-manager/issues)
- **Questions?** All our guides are in the [docs folder](../README.md)