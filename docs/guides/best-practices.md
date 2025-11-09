# Writing Better Use Cases

Quick tips to help you write clear, useful use cases with MUCM.

## Know Your Purpose

Before writing, ask yourself: **Who will read this and what do they need to know?**

**For feature proposals** - Keep it high-level:
- Focus on user value and business impact  
- Skip technical details for now
- Explain the "why" before the "what"

**For implementation** - Add technical depth:
- Include integration points and error handling
- Specify performance requirements
- Detail API specifications and data models

## Writing for Different Audiences

**Business stakeholders** want to see value and outcomes. Lead with business impact, use their language, keep technical details minimal.

**Developers** need implementation clarity. Include enough detail for estimation, specify integration points, cover error scenarios thoroughly.

**Testers** focus on verification. Write testable acceptance criteria, include edge cases, define quality thresholds.

## Good Use Case Structure
ed for formal requireme
### Clear Titles
Use action-oriented names that immediately tell you what happens:

**Good:** "Customer Places Order", "Admin Generates Report", "User Resets Password"  
**Avoid:** "Order Processing", "Reports", "Password Stuff"

### Organize by Business Area
```bash
# Group related use cases together
mucm create "User Login" --category "Authentication"
mucm create "Password Reset" --category "Authentication"  
mucm create "Two-Factor Setup" --category "Authentication"
```

### Focus on Behavior, Not Implementation
Describe what happens, not how the system does it:

**Good:**
```markdown
1. Customer selects products for purchase
2. Customer provides shipping information  
3. System processes order and generates confirmation
```

**Avoid:**
```markdown
1. User clicks "Add to Cart" button
2. JavaScript updates cart display
3. React component renders payment form
```

## Writing Great Personas

Personas drive better design decisions, but only if they're specific and realistic. Generic personas are useless.

**Create personas with real context:**

```markdown
# Sarah - The Busy Manager

## Background
- Department head at 50-person company
- Manages 8 direct reports
- Uses mobile 70% of the time
- Values speed over features

## Daily Reality  
- Checks dashboard during 20-minute commute
- Approves requests between back-to-back meetings
- Reviews weekly reports on Sunday mornings

## Pain Points
- Too many systems to check
- Slow mobile interfaces  
- Complex approval workflows

## Goals
- Approve requests in under 30 seconds
- Get team status at a glance
- Never miss urgent approvals
```

**Use personas to guide your use cases:**
- Reference personas by name: "When Sarah needs to approve..."
- Write scenarios that match their real context
- Consider their device usage and time constraints
- Address their specific pain points

## Methodology Quick Reference

**Simple methodology** - Essential information only. Great for small teams and quick projects.

**Business methodology** - Focus on stakeholder value and business impact. Perfect for enterprise projects.

**Testing methodology** - Emphasize quality gates and test scenarios. Ideal for quality-critical systems.

Choose based on your team size, project complexity, and quality requirements.

## Common Mistakes to Avoid

**Writing implementation details instead of user goals**
- Don't describe button clicks and database calls
- Do describe what the user accomplishes

**Generic, useless personas**  
- Don't create "User who wants good UX"
- Do create specific people with real contexts and constraints

**Vague acceptance criteria**
- Don't write "System should work well"  
- Do write "System responds within 2 seconds"

**Missing the human element**
- Don't forget why people need this feature
- Do reference your personas and their specific needs

## Quick Persona Tips

Personas should feel like real people you could have lunch with. Give them:

- **Specific job title and responsibilities**
- **Real time constraints** ("checks email during 15-minute train ride")  
- **Actual pain points** ("current system takes 5 clicks to approve")
- **Clear goals** ("wants to approve requests without opening laptop")

Then reference them in your use cases: "When Sarah is between meetings and needs to quickly approve..." This keeps your use cases grounded in real user needs instead of abstract requirements.