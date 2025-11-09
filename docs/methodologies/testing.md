# Testing & QA Methodology

Test-driven approach focusing on automated testing, quality assurance, and comprehensive coverage.

## What Is It?

The Testing & QA methodology prioritizes quality assurance, test automation, and comprehensive coverage. It emphasizes testable requirements, automation focus, and quality metrics to ensure robust software delivery.

## Why Use Testing & QA Style?

- **Quality-first** - Built-in quality considerations from the start
- **Test automation** - Designed for automated testing frameworks
- **Comprehensive coverage** - Ensures all scenarios are testable
- **CI/CD ready** - Integrates well with continuous integration
- **Risk mitigation** - Identifies potential quality issues early
- **Measurable quality** - Provides clear quality metrics

## When Should I Use This?

### ✅ Great for:
- **Quality-critical systems** - Healthcare, financial, safety systems
- **Test automation projects** - Heavy automated testing focus
- **Agile/DevOps teams** - Continuous integration and delivery
- **Regression-heavy projects** - Frequent changes requiring testing
- **Compliance testing** - Regulatory or certification requirements
- **API and service testing** - Backend systems and microservices

### ❌ Maybe not the best for:
- **Quick prototypes** - Testing overhead not justified ([try Simple](simple.md))
- **Pure business analysis** - Stakeholder focus might prefer [Business approach](business.md)
- **Documentation-only projects** - No actual implementation to test
- **One-off scripts** - Testing infrastructure overhead too high

## Template Structure

### Test-Focused Metadata
- **Test Complexity** - Simple, Medium, Complex assessment
- **Automation Level** - Manual, Semi-automated, Fully automated
- **Test Priority** - Critical, High, Medium, Low
- **Coverage Requirements** - Expected test coverage percentage
- **Test Types** - Unit, Integration, E2E, Performance

### Quality Assurance Sections
- **Test Strategy** - Overall testing approach
- **Test Scenarios** - Detailed test cases with expected outcomes
- **Automation Notes** - Automation feasibility and approach
- **Quality Metrics** - Success criteria and measurements
- **Risk Assessment** - Testing risks and mitigation strategies

## Key Features

### Automation Focus
Every use case includes automation considerations:
- Automation feasibility assessment
- Test framework recommendations
- CI/CD integration points
- Automated test generation guidance

### Quality Metrics
Built-in quality measurement:
- Test coverage expectations
- Performance benchmarks
- Quality gates and criteria
- Success/failure metrics

### Risk-Based Testing
Systematic risk assessment:
- High-risk scenario identification
- Test priority assignment
- Coverage optimization
- Quality risk mitigation

## Example Output

```markdown
# UC-TES-001: Payment Processing Validation

**Test Complexity:** High - Multiple integration points and edge cases
**Automation Level:** Fully Automated - Critical for CI/CD pipeline
**Quality Priority:** Critical - Financial transaction system

## Test Strategy
Comprehensive testing approach covering unit tests, integration tests, 
and end-to-end scenarios with focus on error handling and edge cases.

## Test Scenarios

### Primary Flow: Successful Payment
**Expected Outcome:** Payment processed, confirmation sent, transaction logged
**Test Type:** Integration
**Automation:** Yes - API testing framework
**Coverage:** Critical path - must pass

### Error Scenarios
1. **Invalid Credit Card**
   - Expected: Validation error, user notification
   - Test Type: Unit + Integration
   - Automation: Yes

2. **Network Timeout**
   - Expected: Retry logic, fallback handling
   - Test Type: Integration
   - Automation: Yes - using network simulation

## Quality Metrics
- **Unit Test Coverage:** 95%+ required
- **Integration Coverage:** 90%+ required
- **Performance:** <2 second response time
- **Error Rate:** <0.1% in production
```

## Best Practices

### 1. Design for Testability
Structure use cases with clear, testable acceptance criteria.

### 2. Automate Everything Possible
Identify automation opportunities early in the process.

### 3. Risk-Based Prioritization
Focus testing effort on high-risk, high-impact scenarios.

### 4. Continuous Quality
Integrate quality checks throughout the development process.

### 5. Measurable Outcomes
Define clear, measurable quality criteria for success.

## Test Types and Focus

### Unit Testing
- Individual component validation
- Fast feedback loops
- High coverage expectations
- Developer-friendly assertions

### Integration Testing
- Component interaction validation
- API contract testing
- Database integration checks
- Third-party service mocking

### End-to-End Testing
- Complete user journey validation
- Cross-system functionality
- Real-world scenario simulation
- Performance under load

### Quality Assurance
- Manual exploratory testing
- Usability and accessibility
- Security vulnerability testing
- Compliance verification

## Integration with Development

### CI/CD Pipeline
- Automated test execution
- Quality gate enforcement
- Performance monitoring
- Deployment validation

### Test Framework Integration
- Compatible with popular frameworks
- Test generation support
- Reporting and metrics
- Failure analysis tools

## Configuration Example

```toml
[templates]
methodology = "testing"
test_generation_enabled = true
automation_focus = true

[generation]
test_language = "rust"
auto_generate_tests = true
test_framework = "criterion"

[quality]
coverage_threshold = 90
automation_required = true
performance_testing = true
```

## Related Documentation

- [CLI Reference](../reference/cli-reference.md) - Command-line usage
- [Configuration Guide](../guides/configuration.md) - Setup and customization
- [Simple Methodology](simple.md) - Alternative lightweight approach
- [Business Methodology](business.md) - Stakeholder-focused approach