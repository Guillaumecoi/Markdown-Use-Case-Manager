# Troubleshooting Guide

Common issues and solutions when using the MD Use Case Manager.

## Installation Issues

### "Command not found: mucm"

**Problem**: After installation, the `mucm` command is not recognized.

**Solutions**:

1. **Check Cargo bin directory is in PATH**:
   ```bash
   echo $PATH | grep -q "$HOME/.cargo/bin" || echo "Cargo bin not in PATH"
   
   # Add to shell profile
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc  # or ~/.zshrc
   source ~/.bashrc
   ```

2. **Verify installation**:
   ```bash
   ls -la ~/.cargo/bin/mucm
   cargo install --list | grep mucm
   ```

3. **Reinstall if necessary**:
   ```bash
   cargo uninstall mucm
   cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
   ```

### "Failed to compile" errors

**Problem**: Compilation fails during installation.

**Solutions**:

1. **Update Rust toolchain**:
   ```bash
   rustup update stable
   rustup default stable
   ```

2. **Check system dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install build-essential pkg-config libssl-dev
   
   # macOS
   xcode-select --install
   
   # Windows
   # Install Visual Studio Build Tools or Visual Studio Community
   ```

3. **Clean and retry**:
   ```bash
   cargo clean
   cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager --force
   ```

## Configuration Issues

### "Configuration file not found"

**Problem**: MUCM cannot find the configuration file.

**Solutions**:

1. **Initialize the project**:
   ```bash
   mucm init
   ```

2. **Check directory structure**:
   ```bash
   ls -la .config/.mucm/
   # Should show: mucm.toml, templates/
   ```

3. **Verify working directory**:
   ```bash
   pwd
   # Ensure you're in the project root directory
   ```

### "Invalid methodology specified"

**Problem**: Configuration contains an unknown methodology.

**Solutions**:

1. **Check valid methodologies**:
   ```bash
   mucm init --help
   # Valid options: simple, cockburn, unified_process, bdd_gherkin
   ```

2. **Update configuration**:
   ```toml
   # .config/.mucm/mucm.toml
   [templates]
   methodology = "simple"  # Use valid methodology name
   ```

3. **Reset configuration**:
   ```bash
   mucm init --reset-config
   ```

### "Template not found" errors

**Problem**: MUCM cannot locate template files.

**Solutions**:

1. **Check template directory**:
   ```bash
   ls -la .config/.mucm/templates/methodologies/
   ls -la templates/methodologies/  # Alternative location
   ```

2. **Reinstall templates**:
   ```bash
   mucm init --reset-templates
   ```

3. **Verify template configuration**:
   ```toml
   [directories]
   template_dir = ".config/.mucm/templates"  # Correct path
   ```

## Use Case Management Issues

### "Use case ID not found"

**Problem**: Commands fail with "Use case not found" error.

**Solutions**:

1. **List existing use cases**:
   ```bash
   mucm list
   mucm list --format table  # Better formatting
   ```

2. **Check ID format**:
   ```bash
   # IDs are case-sensitive: UC-AUTH-001, not uc-auth-001
   mucm update-status UC-AUTH-001 --status implemented
   ```

3. **Verify use case exists**:
   ```bash
   find docs/use-cases -name "*.md" | grep -i auth
   ```

### "Validation failed" errors

**Problem**: Use case validation fails during creation or update.

**Solutions**:

1. **Check required fields**:
   ```bash
   mucm create "Test Case" --extended  # Include all required fields
   ```

2. **Review validation settings**:
   ```toml
   [validation]
   enforce_required_fields = false  # Temporary relaxation
   ```

3. **Check field content**:
   ```markdown
   # Ensure required sections are not empty
   ## Actor
   [Must not be empty]
   
   ## Main Flow
   [Must contain at least one step]
   ```

### "Duplicate ID" errors

**Problem**: Attempting to create use case with existing ID.

**Solutions**:

1. **Check existing IDs**:
   ```bash
   mucm list | grep -i "auth"
   ```

2. **Use different category or title**:
   ```bash
   mucm create "User Authentication" --category "Security"      # UC-SEC-001
   mucm create "API Authentication" --category "Integration"    # UC-INT-001
   ```

3. **Manual ID resolution**:
   ```bash
   # Find next available number
   mucm list --category Authentication
   ```

## File System Issues

### "Permission denied" errors

**Problem**: MUCM cannot read or write files.

**Solutions**:

1. **Check directory permissions**:
   ```bash
   ls -la .config/
   ls -la docs/use-cases/
   ```

2. **Fix permissions**:
   ```bash
   chmod -R 755 .config/.mucm/
   chmod -R 755 docs/use-cases/
   ```

3. **Check ownership**:
   ```bash
   sudo chown -R $USER:$USER .config/ docs/
   ```

### "Directory not found" errors

**Problem**: MUCM cannot find specified directories.

**Solutions**:

1. **Check configuration paths**:
   ```toml
   [directories]
   use_case_dir = "docs/use-cases"  # Relative to project root
   test_dir = "tests/use-cases"
   ```

2. **Create missing directories**:
   ```bash
   mkdir -p docs/use-cases
   mkdir -p tests/use-cases
   ```

3. **Use absolute paths if needed**:
   ```toml
   [directories]
   use_case_dir = "/full/path/to/docs/use-cases"
   ```

## Template Issues

### "Template syntax error"

**Problem**: Custom templates contain syntax errors.

**Solutions**:

1. **Validate template syntax**:
   ```bash
   mucm template validate --template custom.hbs
   ```

2. **Check Handlebars syntax**:
   ```handlebars
   {{!-- Valid syntax --}}
   {{#if use_case.title}}
   # {{use_case.title}}
   {{/if}}
   
   {{!-- Invalid syntax --}}
   {{#if use_case.title}  <!-- Missing closing }} -->
   # {{use_case.title}}
   {{/fi}}                <!-- Typo in closing tag -->
   ```

3. **Reset to default templates**:
   ```bash
   mucm init --reset-templates
   ```

### "Variable not found" errors

**Problem**: Template references undefined variables.

**Solutions**:

1. **Check available variables**:
   ```handlebars
   {{!-- Common variables --}}
   {{use_case.id}}
   {{use_case.title}}
   {{use_case.category}}
   {{use_case.status}}
   {{project.name}}
   ```

2. **Use conditional checks**:
   ```handlebars
   {{#if use_case.business_value}}
   ## Business Value
   {{use_case.business_value}}
   {{/if}}
   ```

3. **Review custom field configuration**:
   ```toml
   [templates]
   custom_fields = ["epic", "story_points"]  # Define custom fields
   ```

## Interactive Mode Issues

### "Terminal not supported"

**Problem**: Interactive mode fails to start.

**Solutions**:

1. **Check terminal compatibility**:
   ```bash
   echo $TERM
   # Should be xterm, xterm-256color, etc.
   ```

2. **Use fallback mode**:
   ```bash
   mucm --no-interactive create "Test Case"
   ```

3. **Update terminal settings**:
   ```bash
   export TERM=xterm-256color
   mucm -i
   ```

### "Arrow keys not working"

**Problem**: Navigation in interactive mode is broken.

**Solutions**:

1. **Check terminal emulator**:
   - Use modern terminal: iTerm2, Windows Terminal, GNOME Terminal
   - Avoid basic terminals: Windows Command Prompt, basic xterm

2. **Alternative navigation**:
   ```
   Use Tab/Shift+Tab for navigation
   Use Space/Enter for selection
   Use Ctrl+C to exit
   ```

3. **Enable terminal features**:
   ```bash
   # Enable mouse support if available
   export TERM=xterm-1003
   ```

## Test Generation Issues

### "Test language not supported"

**Problem**: Selected test language is not available.

**Solutions**:

1. **Check supported languages**:
   ```toml
   [generation]
   test_language = "rust"     # rust, python, none
   ```

2. **Verify language templates exist**:
   ```bash
   ls -la .config/.mucm/templates/languages/
   ```

3. **Disable test generation**:
   ```toml
   [generation]
   test_language = "none"
   auto_generate_tests = false
   ```

### "Test framework not found"

**Problem**: Test generation fails due to missing framework.

**Solutions**:

1. **Install required framework**:
   ```bash
   # Rust
   cargo add tokio --features full
   cargo add serde_json
   
   # Python
   pip install pytest pytest-asyncio
   ```

2. **Check framework configuration**:
   ```toml
   [generation]
   test_framework = "default"  # Use default for language
   ```

3. **Use custom test templates**:
   ```bash
   cp templates/languages/rust/test.hbs .config/.mucm/templates/languages/rust/
   # Edit custom template
   ```

## Performance Issues

### "Slow command execution"

**Problem**: MUCM commands take too long to execute.

**Solutions**:

1. **Check project size**:
   ```bash
   find docs/use-cases -name "*.md" | wc -l
   # Large projects (>1000 files) may be slower
   ```

2. **Optimize configuration**:
   ```toml
   [validation]
   check_broken_references = false  # Disable for large projects
   
   [generation]
   auto_generate_tests = false      # Disable if not needed
   ```

3. **Use targeted commands**:
   ```bash
   mucm list --category Authentication  # Filter results
   mucm status --summary               # Less detailed output
   ```

### "High memory usage"

**Problem**: MUCM consumes excessive memory.

**Solutions**:

1. **Check for large files**:
   ```bash
   find docs/use-cases -name "*.md" -size +1M
   ```

2. **Split large use cases**:
   ```bash
   # Instead of one 100-scenario use case
   # Create multiple focused use cases
   ```

3. **Reduce template complexity**:
   ```handlebars
   {{!-- Avoid complex loops in templates --}}
   {{#each scenarios}}
     {{!-- Keep processing simple --}}
   {{/each}}
   ```

## Integration Issues

### "CI/CD pipeline failures"

**Problem**: MUCM commands fail in CI/CD environment.

**Solutions**:

1. **Check installation in CI**:
   ```yaml
   # GitHub Actions
   - name: Install MUCM
     run: cargo install --git https://github.com/GuillaumeCoi/markdown-use-case-manager
   ```

2. **Verify working directory**:
   ```yaml
   - name: Run MUCM
     working-directory: ./project-root
     run: mucm status
   ```

3. **Handle missing interactive features**:
   ```yaml
   - name: Non-interactive validation
     run: mucm validate --all --no-interactive
   ```

### "Static site generation issues"

**Problem**: Generated documentation doesn't build correctly.

**Solutions**:

1. **Check output format**:
   ```bash
   mucm list --format markdown > docs/index.md
   # Verify markdown syntax
   ```

2. **Validate frontmatter**:
   ```markdown
   ---
   title: Use Case Overview
   layout: default
   ---
   ```

3. **Check site generator configuration**:
   ```yaml
   # _config.yml (Jekyll)
   markdown: kramdown
   highlighter: rouge
   ```

## Debugging

### Enable Debug Logging

```bash
export MUCM_LOG_LEVEL=debug
mucm create "Debug Test Case"
```

### Verbose Output

```bash
mucm --verbose status
mucm -v list
```

### Configuration Debugging

```bash
# Show current configuration
mucm config show

# Validate configuration
mucm config validate

# Show template information
mucm template info --methodology cockburn
```

### File System Debugging

```bash
# Check file permissions
ls -la .config/.mucm/
ls -la docs/use-cases/

# Verify file contents
cat .config/.mucm/mucm.toml

# Check template files
find .config/.mucm/templates -name "*.hbs" -exec echo "=== {} ===" \; -exec head -5 {} \;
```

## Getting Help

### Built-in Help

```bash
mucm --help
mucm create --help
mucm -i  # Interactive help
```

### Community Resources

- **GitHub Issues**: [Report bugs](https://github.com/GuillaumeCoi/markdown-use-case-manager/issues)
- **Discussions**: [Ask questions](https://github.com/GuillaumeCoi/markdown-use-case-manager/discussions)
- **Documentation**: [Complete guides](docs/)

### Diagnostic Information

When reporting issues, include:

```bash
# System information
uname -a
rustc --version
cargo --version

# MUCM information
mucm --version
mucm config show

# Project structure
find . -name "mucm.toml" -o -name "*.md" | head -20
```

## Common Error Messages

| Error | Meaning | Solution |
|-------|---------|----------|
| "Configuration file not found" | No mucm.toml file | Run `mucm init` |
| "Invalid methodology" | Unknown methodology name | Use valid methodology or reset config |
| "Template not found" | Missing template file | Reset templates or check paths |
| "Use case ID not found" | ID doesn't exist | Check ID with `mucm list` |
| "Validation failed" | Required fields missing | Add required content or relax validation |
| "Permission denied" | File access issue | Fix file permissions |
| "Directory not found" | Path doesn't exist | Create directories or fix paths |
| "Duplicate ID" | ID already exists | Use different title/category |

## Prevention Best Practices

1. **Regular validation**: Run `mucm status` frequently
2. **Backup configuration**: Version control `.config/.mucm/`
3. **Test in development**: Validate changes before committing
4. **Monitor logs**: Check for warnings and errors
5. **Keep updated**: Regular updates to latest version