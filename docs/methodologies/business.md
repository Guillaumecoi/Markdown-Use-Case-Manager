# Business Analysis Methodology

Enterprise-focused approach emphasizing stakeholder value, business requirements, and strategic alignment.

## What Is It?

The Business Analysis methodology combines elements from various business analysis frameworks to create comprehensive documentation that speaks to both technical and business stakeholders. It emphasizes business value, stakeholder impact, and strategic alignment.

## Why Use Business Analysis Style?

- **Business-focused** - Emphasizes business value and stakeholder needs
- **Strategic alignment** - Connects use cases to business objectives
- **Stakeholder communication** - Clear communication with business users
- **Enterprise-ready** - Suitable for formal business environments
- **Decision support** - Provides data for business decisions

## When Should I Use This?

### ✅ Great for:
- **Enterprise projects** - Large organizational initiatives
- **Business transformation** - Digital transformation projects
- **Stakeholder management** - Multiple business stakeholders involved
- **ROI justification** - Need to demonstrate business value
- **Compliance projects** - Regulatory or audit requirements
- **Strategic initiatives** - Projects aligned with business strategy

### ❌ Maybe not the best for:
- **Quick prototypes** - Might be too heavy ([try Simple](simple.md))
- **Internal tools only** - Technical teams might prefer [Testing approach](testing.md)
- **Pure technical projects** - No business stakeholder involvement
- **Very small teams** - Overhead might not be justified

## Template Structure

### Enhanced Metadata
- **Business Value Statement** - Clear ROI and benefit articulation
- **Stakeholder Mapping** - Primary and secondary stakeholders
- **Business Priority** - Strategic importance ranking
- **Success Metrics** - Measurable business outcomes
- **Risk Assessment** - Business and technical risks

### Business Context Sections
- **Business Background** - Why this use case matters
- **Stakeholder Value** - What each stakeholder gains
- **Business Rules** - Constraints and policies
- **Success Criteria** - Definition of business success
- **Impact Analysis** - Effect on business processes

## Key Features

### Stakeholder Focus
Every use case includes detailed stakeholder analysis:
- Primary users and their goals
- Secondary stakeholders and their interests
- Business sponsors and their success criteria

### Business Value Emphasis
Clear articulation of business benefits:
- Quantified benefits where possible
- Strategic alignment statements
- ROI considerations
- Risk mitigation value

### Enterprise Integration
Designed for enterprise environments:
- Process integration points
- System dependencies
- Compliance considerations
- Change management implications

## Example Output

```markdown
# UC-BIZ-001: Customer Onboarding Automation

**Business Value:** Reduces onboarding time by 60% and improves customer satisfaction
**Strategic Priority:** High - Supports digital transformation initiative
**Business Sponsor:** VP Customer Experience

## Stakeholder Value
- **New Customers:** Faster, easier onboarding experience
- **Customer Service:** Reduced manual workload, focus on complex cases
- **Business:** Improved conversion rates and customer satisfaction
- **Compliance:** Automated audit trail and regulatory compliance

## Business Context
This use case supports the company's strategic initiative to improve customer experience
while reducing operational costs. Current manual onboarding takes 3-5 business days
and requires significant manual intervention.

## Success Criteria
- Onboarding time reduced from 3-5 days to same-day completion
- Customer satisfaction score improvement of 15%
- 40% reduction in customer service onboarding calls
- 100% compliance with regulatory requirements
```

## Best Practices

### 1. Start with Business Value
Always begin by clearly articulating why this use case matters to the business.

### 2. Map All Stakeholders
Don't just focus on primary users - identify everyone who is impacted.

### 3. Quantify Where Possible
Use specific metrics and measurements for business benefits.

### 4. Connect to Strategy
Link each use case to broader business objectives and initiatives.

### 5. Consider the Entire Process
Think beyond the immediate use case to broader business process implications.

## Integration with Other Methodologies

### Complementary Approaches
- Can be combined with [Testing methodology](testing.md) for quality focus
- [Simple methodology](simple.md) can be used for rapid prototyping before business analysis

### When to Switch
- Start with Simple for initial exploration
- Move to Business for stakeholder communication and approval
- Add Testing methodology for implementation phase

## Configuration Example

```toml
[templates]
methodology = "business"
use_extended_metadata = true
business_context_enabled = true

[metadata]
business_value_required = true
stakeholder_mapping_enabled = true
success_metrics_required = true
```

## Related Documentation

- [CLI Reference](../reference/cli-reference.md) - Command-line usage
- [Configuration Guide](../guides/configuration.md) - Setup and customization
- [Simple Methodology](simple.md) - Alternative lightweight approach
- [Testing Methodology](testing.md) - Quality-focused approach