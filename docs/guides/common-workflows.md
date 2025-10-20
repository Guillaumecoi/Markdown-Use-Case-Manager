# Common Workflows

This guide shows practical, step-by-step workflows for typical use cases.

## 🚀 Quick Start: Your First Use Case

```bash
# 1. Initialize a new project
mucm init -l rust

# 2. Create your first use case
mucm create "User Registration" -c "Authentication" \
  -d "Allow new users to create an account"

# Note the ID returned (e.g., UC-AUT-001)

# 3. Add scenarios
mucm add-scenario UC-AUT-001 "Register with email and password"
mucm add-scenario UC-AUT-001 "Register with social login"
mucm add-scenario UC-AUT-001 "Handle duplicate email"

# 4. Track progress
mucm update-status UC-AUT-001-S01 --status implemented
mucm update-status UC-AUT-001-S02 --status in_progress

# 5. View your work
mucm list
mucm status
```

## 📋 Planning a Feature: Complete Workflow

### Scenario: Building a Shopping Cart Feature

```bash
# Step 1: Initialize with methodology
mucm init -l rust --methodology business

# Step 2: Create the main use case
mucm create "Shopping Cart Management" -c "E-commerce" \
  -d "Enable customers to manage items before purchase"
# Returns: UC-E-C-001

# Step 3: Add all scenarios upfront
mucm add-scenario UC-E-C-001 "Add item to cart"
mucm add-scenario UC-E-C-001 "Remove item from cart"
mucm add-scenario UC-E-C-001 "Update item quantity"
mucm add-scenario UC-E-C-001 "View cart total"
mucm add-scenario UC-E-C-001 "Handle out-of-stock items"
mucm add-scenario UC-E-C-001 "Apply discount code"

# Step 4: Track implementation
# As you implement each scenario:
mucm update-status UC-E-C-001-S01 --status implemented
mucm update-status UC-E-C-001-S02 --status implemented
mucm update-status UC-E-C-001-S03 --status in_progress

# Step 5: Check progress anytime
mucm status
# Shows overall project health and what's left to do
```

## 🔄 Working with Multiple Categories

```bash
# Create use cases across different areas
mucm create "User Login" -c "Authentication"
mucm create "Password Reset" -c "Authentication"
mucm create "Product Search" -c "Catalog"
mucm create "Product Filtering" -c "Catalog"
mucm create "Checkout Process" -c "Payment"

# View organized by category
mucm list

# The overview (docs/use-cases/README.md) groups by category automatically
```

## 🎯 Tracking Sprint Progress

```bash
# Start of sprint - mark planned work
mucm update-status UC-AUT-001-S01 --status in_progress
mucm update-status UC-CAT-001-S01 --status in_progress

# During sprint - update as you go
mucm update-status UC-AUT-001-S01 --status implemented
mucm update-status UC-AUT-001-S01 --status tested

# End of sprint - check what's done
mucm status
# Quickly see what's TESTED vs IN_PROGRESS vs PLANNED
```

## 📝 Documentation-Driven Development

```bash
# 1. Write the use cases BEFORE coding
mucm create "API Rate Limiting" -c "Infrastructure" \
  -d "Prevent API abuse through request throttling"

mucm add-scenario UC-INF-001 "Allow requests under limit"
mucm add-scenario UC-INF-001 "Block requests over limit"
mucm add-scenario UC-INF-001 "Reset counter after time window"

# 2. Use the generated docs for implementation guidance
cat docs/use-cases/infrastructure/UC-INF-001.md

# 3. Use the generated tests as a starting point
cat tests/use-cases/infrastructure/uc_inf_001.rs

# 4. Implement the code

# 5. Update status as you go
mucm update-status UC-INF-001-S01 --status implemented
mucm update-status UC-INF-001-S01 --status tested
mucm update-status UC-INF-001-S01 --status deployed
```

## 🏢 Enterprise Workflow with Methodologies

### Using Business Methodology for Stakeholder Communication

```bash
# Initialize with business methodology
mucm init --methodology business

# Create use cases with business focus
mucm create "Monthly Sales Report" -c "Reporting" \
  --methodology business

# The generated documentation includes:
# - Business value proposition
# - Stakeholder identification
# - Success metrics
# - ROI expectations
```

### Using Testing Methodology for QA Teams

```bash
# Initialize with testing focus
mucm init --methodology testing

# Creates use cases optimized for test coverage
mucm create "User Authentication Flow" -c "Security" \
  --methodology tester

# Generated docs include:
# - Test complexity assessment
# - Automation requirements
# - Quality metrics
# - Test scenarios in Given/When/Then format
```

## 🔧 Switching Methodologies for Existing Use Cases

```bash
# You created a use case with simple methodology
mucm create "Data Export" -c "Reports"
# Created: UC-REP-001

# Later, you want more detail for a specific use case
mucm regenerate UC-REP-001 --methodology business

# Now UC-REP-001.md has business-focused template
# while other use cases remain simple
```

## 📊 Tracking Multiple Projects

```bash
# Each project directory gets its own .config/.mucm/
cd ~/projects/web-app
mucm init -l javascript
mucm create "User Dashboard" -c "Frontend"

cd ~/projects/api-service  
mucm init -l rust
mucm create "REST API Endpoints" -c "Backend"

# Separate configurations, separate documentation
# Each tracked independently
```

## 🚢 Pre-Release Checklist Workflow

```bash
# Check what needs attention before release
mucm status

# Ensure critical scenarios are tested
mucm list | grep "IMPLEMENTED"  # Should be empty
mucm list | grep "IN_PROGRESS"  # What's still being worked on

# Generate fresh overview for stakeholders
# The README.md is auto-updated, just commit it
git add docs/use-cases/README.md
git commit -m "Update use case status for v2.0 release"
```

## 💡 Tips

### Use Case ID Format
- Format: `UC-XXX-NNN` where XXX is category abbreviation (first 3 letters, uppercase)
- Examples:
  - "Authentication" → `UC-AUT-001`, `UC-AUT-002`
  - "Payment" → `UC-PAY-001`
  - "E-commerce" → `UC-E-C-001` (special case with hyphen)

### Scenario ID Format  
- Format: `{USE_CASE_ID}-S{NN}`
- Examples: `UC-AUT-001-S01`, `UC-PAY-001-S03`

### Status Progression
Typical flow: `planned → in_progress → implemented → tested → deployed`

### Best Practices
1. **Create all scenarios upfront** during planning
2. **Update status frequently** to keep docs current
3. **Run `mucm status`** regularly to track progress
4. **Use methodologies consistently** within a project
5. **Commit generated docs** to version control

## 🔗 See Also
- [CLI Reference](../reference/cli-reference.md) - All available commands
- [Best Practices](best-practices.md) - Writing better use cases
- [Configuration Guide](configuration.md) - Customizing your setup
