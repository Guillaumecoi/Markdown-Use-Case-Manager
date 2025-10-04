# Template System Architecture

Understanding the template system architecture and how methodology-based templates work in MUCM.

## Overview

The MUCM template system is designed around methodology-specific templates that adapt to different software development approaches. Each methodology provides a complete set of templates optimized for that approach's philosophy and practices.

## Architecture Components

### Template Registry

The `TemplateRegistry` manages all template metadata and relationships:

```rust
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub methodology: String,
    pub literature_source: Option<String>,
    pub use_case_template_content: String,
    pub persona_template_content: String,
}
```

### Methodology Organization

Templates are organized by methodology in the filesystem:

```
templates/methodologies/
├── simple/
│   ├── use_case_simple.hbs
│   ├── persona.hbs
│   └── overview.hbs
├── cockburn/
│   ├── use_case_cockburn.hbs
│   ├── persona.hbs
│   └── overview.hbs
├── rup/
│   ├── use_case_detailed.hbs
│   ├── persona.hbs
│   └── overview.hbs
└── bdd/
    ├── use_case_bdd.hbs
    ├── persona.hbs
    └── overview.hbs
```

### Template Resolution

The system resolves templates based on configuration:

1. **Configuration Check**: Read methodology from `mucm.toml`
2. **Template Lookup**: Find templates in methodology directory
3. **Fallback Handling**: Use simple methodology if specified methodology not found
4. **Custom Override**: Allow custom template directory override

## Template Types

### Use Case Templates

Primary templates for documenting use cases:

- **`use_case_simple.hbs`** - Minimal structure for rapid development
- **`use_case_cockburn.hbs`** - Goal-oriented with stakeholder analysis
- **`use_case_detailed.hbs`** - Comprehensive RUP-style documentation
- **`use_case_bdd.hbs`** - Collaborative BDD format with scenarios

### Persona Templates

Methodology-specific persona documentation:

- **Simple**: Basic user description format
- **Cockburn**: Stakeholder interests and goal analysis
- **RUP**: Formal actor classification and attributes
- **BDD**: User story format with behavioral characteristics

### Overview Templates

Auto-generated documentation overviews:

- **`overview.hbs`** - Project-wide use case summary
- Methodology-agnostic structure
- Supports all template approaches

## Template Variables

### Core Variables

Available in all templates:

```handlebars
{{project.name}}                    <!-- Project name -->
{{project.description}}             <!-- Project description -->
{{use_case.id}}                    <!-- Auto-generated ID -->
{{use_case.title}}                 <!-- Use case title -->
{{use_case.category}}              <!-- Category classification -->
{{use_case.priority}}              <!-- Priority level -->
{{use_case.status}}                <!-- Current status -->
{{use_case.actor}}                 <!-- Primary actor -->
{{use_case.preconditions}}         <!-- Prerequisites -->
{{use_case.main_flow}}             <!-- Main scenario -->
{{use_case.alternative_flows}}     <!-- Alternative scenarios -->
{{use_case.postconditions}}        <!-- Success outcomes -->
```

### Extended Metadata Variables

When `use_extended_metadata = true`:

```handlebars
{{use_case.business_value}}        <!-- Business justification -->
{{use_case.acceptance_criteria}}   <!-- Definition of done -->
{{use_case.author}}               <!-- Author information -->
{{use_case.reviewer}}             <!-- Reviewer information -->
{{use_case.assumptions}}          <!-- Design assumptions -->
{{use_case.constraints}}          <!-- Limitations -->
{{use_case.prerequisites}}        <!-- Dependencies -->
```

### Methodology-Specific Variables

#### Cockburn Variables
```handlebars
{{use_case.goal}}                  <!-- Goal statement -->
{{use_case.scope}}                 <!-- System boundary -->
{{use_case.level}}                 <!-- Goal level -->
{{use_case.stakeholders}}          <!-- Stakeholder analysis -->
{{use_case.success_guarantee}}     <!-- Postcondition guarantee -->
{{use_case.extensions}}            <!-- Extension scenarios -->
```

#### RUP Variables
```handlebars
{{use_case.brief_description}}     <!-- Summary description -->
{{use_case.classification}}        <!-- Formal classification -->
{{use_case.relationships}}         <!-- Use case relationships -->
{{use_case.special_requirements}}  <!-- Non-functional requirements -->
{{use_case.revision_history}}      <!-- Change tracking -->
```

#### BDD Variables
```handlebars
{{use_case.feature}}              <!-- Feature description -->
{{use_case.background}}           <!-- Common context -->
{{use_case.scenarios}}            <!-- BDD scenarios -->
{{use_case.business_rules}}       <!-- Domain rules -->
{{use_case.automation_tags}}      <!-- Test automation tags -->
```

### Scenario Variables

For scenario-specific templates:

```handlebars
{{scenario.id}}                   <!-- Scenario ID -->
{{scenario.title}}                <!-- Scenario title -->
{{scenario.priority}}             <!-- Scenario priority -->
{{scenario.status}}               <!-- Scenario status -->
{{scenario.steps}}                <!-- Scenario steps -->
{{scenario.given}}                <!-- BDD Given statements -->
{{scenario.when}}                 <!-- BDD When statements -->
{{scenario.then}}                 <!-- BDD Then statements -->
```

## Handlebars Helpers

### Conditional Helpers

```handlebars
{{#if use_case.business_value}}
## Business Value
{{use_case.business_value}}
{{/if}}

{{#unless use_case.assumptions}}
No assumptions documented.
{{/unless}}
```

### Iteration Helpers

```handlebars
{{#each use_case.scenarios}}
### Scenario {{@index}}: {{this.title}}
{{this.description}}
{{/each}}

{{#each use_case.stakeholders}}
- **{{this.role}}**: {{this.interest}}
{{/each}}
```

### Formatting Helpers

```handlebars
{{upper use_case.category}}        <!-- UPPERCASE -->
{{lower use_case.id}}              <!-- lowercase -->
{{capitalize use_case.title}}      <!-- Title Case -->
{{slug use_case.title}}            <!-- url-friendly-slug -->
```

### Date Helpers

```handlebars
{{date "Y-m-d"}}                   <!-- 2024-10-03 -->
{{date "F j, Y"}}                  <!-- October 3, 2024 -->
{{timestamp}}                      <!-- Unix timestamp -->
```

## Custom Template Development

### Creating Custom Templates

1. **Copy Existing Template**
   ```bash
   cp templates/methodologies/simple/use_case_simple.hbs my_custom.hbs
   ```

2. **Modify Template Structure**
   ```handlebars
   # {{use_case.id}}: {{use_case.title}}
   
   **Custom Field**: {{use_case.custom_field}}
   
   ## My Custom Section
   {{#if use_case.my_field}}
   {{use_case.my_field}}
   {{/if}}
   ```

3. **Update Configuration**
   ```toml
   [templates]
   custom_template = "my_custom.hbs"
   custom_fields = ["custom_field", "my_field"]
   ```

### Template Validation

Templates are validated for:

- **Syntax**: Valid Handlebars syntax
- **Variables**: Referenced variables exist
- **Logic**: Conditional statements are well-formed
- **Output**: Generated content is valid Markdown

### Testing Templates

```bash
# Test template rendering
mucm template test --template custom.hbs --data test-data.json

# Validate template syntax
mucm template validate --template custom.hbs

# Preview template output
mucm template preview --template custom.hbs --use-case UC-TEST-001
```

## Template Inheritance

### Base Template Pattern

```handlebars
<!-- base.hbs -->
# {{use_case.id}}: {{use_case.title}}

{{> methodology_specific_content}}

## Common Sections
{{> common_postconditions}}
```

### Methodology-Specific Overrides

```handlebars
<!-- cockburn_content.hbs -->
**Scope**: {{use_case.scope}}
**Level**: {{use_case.level}}

## Stakeholders and Interests
{{#each use_case.stakeholders}}
- **{{this.role}}**: {{this.interest}}
{{/each}}
```

### Partial Templates

```handlebars
<!-- common_postconditions.hbs -->
## Postconditions
{{#if use_case.postconditions}}
{{use_case.postconditions}}
{{else}}
Success conditions to be documented.
{{/if}}
```

## Performance Optimization

### Template Caching

Templates are cached after first load:

```rust
lazy_static! {
    static ref TEMPLATE_CACHE: Mutex<HashMap<String, Template>> = 
        Mutex::new(HashMap::new());
}
```

### Compilation Optimization

- Templates compiled once per session
- Variable resolution optimized for common patterns
- Partial template reuse reduces compilation overhead

### Memory Management

- Template registry loaded on demand
- Unused templates garbage collected
- Configuration changes trigger cache invalidation

## Debugging Templates

### Debug Output

Enable debug output for template rendering:

```bash
MUCM_LOG_LEVEL=debug mucm create "Test Case"
```

### Template Variables Inspection

```handlebars
<!-- Debug helper -->
{{#debug}}
Available variables: {{@root}}
{{/debug}}
```

### Common Issues

**Variable Not Found**
```
Error: Variable 'use_case.missing_field' not found
Fix: Add field to configuration or template data
```

**Syntax Error**
```
Error: Unclosed handlebars expression
Fix: Check for missing }} or {{/if}} statements
```

**Infinite Loop**
```
Error: Template recursion limit exceeded
Fix: Check for circular references in partials
```

## Integration Points

### CLI Integration

Templates integrate with CLI commands:

```rust
// Create use case with template
let rendered = template_registry.render_use_case(&use_case, &config)?;
file_service.write_use_case_file(&rendered)?;
```

### Configuration Integration

Templates respond to configuration changes:

```rust
// Reload templates on config change
if config.templates.methodology != old_methodology {
    template_registry.reload_methodology_templates(&config.templates.methodology)?;
}
```

### Test Generation Integration

Templates drive test generation:

```handlebars
<!-- Test template -->
#[test]
fn test_{{slug use_case.title}}() {
    // Test generated from use case: {{use_case.title}}
    {{#each use_case.scenarios}}
    test_scenario_{{@index}}();
    {{/each}}
}
```

## Future Enhancements

### Planned Features

- **Visual Template Editor** - GUI template editing
- **Template Marketplace** - Community template sharing
- **AI-Assisted Generation** - Smart template suggestions
- **Real-time Preview** - Live template rendering
- **Version Control** - Template change tracking

### Extension Points

- **Custom Helpers** - Domain-specific template functions
- **Plugin System** - Third-party template extensions
- **API Integration** - External data source templates
- **Multi-language Output** - Templates in multiple languages