# MD Use Case Manager Documentation

Welcome! This documentation will help you use MUCM to create better use case documents.

## Getting Started

**New to MUCM?** Start here:
- 📘 **[Getting Started Guide](guides/getting-started.md)** - Install and create your first use case
- ⚙️ **[Configuration Guide](guides/configuration.md)** - Set up your project
- � **[Best Practices](guides/best-practices.md)** - Tips for better use cases

## Documentation Sections

### 📘 User Guides
Learn how to use MUCM:
- **[Getting Started](guides/getting-started.md)** - Install and create your first use case
- **[Configuration](guides/configuration.md)** - Set up project settings
- **[Best Practices](guides/best-practices.md)** - Write better use cases
- **[Integration](guides/integration.md)** - Use with other tools

### 📚 Reference
Look up technical details:
- **[CLI Commands](reference/cli-reference.md)** - All command options
- **[Template System](reference/template-system.md)** - How templates work
- **[Troubleshooting](reference/troubleshooting.md)** - Fix common problems

### 🎯 Choose Your Methodology
Pick the right approach for your project:
- **[Simple](methodologies/simple.md)** - Lightweight, flexible approach for rapid development
- **[Business](methodologies/business.md)** - Business-focused approach emphasizing stakeholder value  
- **[Testing](methodologies/testing.md)** - Test-driven approach with comprehensive coverage

---

## Which Methodology Should I Use?

Not sure which methodology to pick? Here's a simple guide:

| What do you need? | Use this methodology | Why? |
|-------------------|---------------------|------|
| Something quick and simple | [Simple](methodologies/simple.md) | Minimal overhead, maximum clarity |
| Business requirements analysis | [Business](methodologies/business.md) | Focuses on stakeholder value and strategic alignment |
| Quality-focused development | [Testing](methodologies/testing.md) | Emphasizes automated testing and comprehensive coverage |

**Still not sure?** Start with Simple - you can always regenerate your use cases with a different methodology later using `mucm regenerate UC-XXX-001 --methodology business`.