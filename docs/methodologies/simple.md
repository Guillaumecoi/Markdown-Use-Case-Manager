# Simple Style

The easiest way to write use cases! No complicated rules, just get your ideas down.

## What Is It?

Simple style is all about keeping things... well, simple! You write just enough to capture your ideas without getting bogged down in formal rules.

## Why Use Simple Style?

- **Quick to write** - No long forms to fill out
- **Easy to read** - Anyone can understand it
- **Flexible** - Change it however you want
- **Great for starting out** - You can always add more detail later

## When Should I Use This?

### ✅ Great for:
- **New projects** - Getting ideas down quickly
- **Small teams** - 5 people or fewer
- **Prototypes** - Testing out ideas
- **Internal tools** - Stuff only your team uses
- **When you're in a hurry** - Need something now, not perfect

### ❌ Maybe not the best for:
- **Large enterprise projects** - They usually want more formal documentation (try Business methodology)
- **Complex business analysis** - Might need Business methodology for better stakeholder analysis
- **Quality-focused development** - Testing methodology works better with automated testing workflows
- **Compliance/regulations** - You'll probably need more formal documentation

## Template Structure

### Core Sections
```markdown
# Use Case Title
Brief description of what this use case achieves

## Actor
Who performs this use case

## Preconditions
What must be true before this use case can start

## Main Flow
1. Step-by-step description
2. Of the primary scenario
3. Using simple, clear language

## Alternative Flows
- Brief descriptions of alternative paths
- Error conditions and edge cases

## Postconditions
What is true after successful completion
```

### Optional Sections
The Simple methodology allows you to add any sections that provide value:
- Business value
- Acceptance criteria
- Technical notes
- Dependencies
- Implementation hints

## Best Practices

### Writing Guidelines
- **Use active voice** - "User clicks button" not "Button is clicked"
- **Be specific** - "User enters email address" not "User enters information"
- **Keep it short** - Aim for 5-10 steps in the main flow
- **One action per step** - Don't combine multiple actions

### Organization Tips
- **Group related use cases** - Use consistent category naming
- **Link dependencies** - Reference other use cases by ID
- **Version your changes** - Use git to track evolution
- **Regular reviews** - Keep documentation current

## Example Use Case

```markdown
# UC-AUTH-001: User Login

Simple authentication flow for registered users.

## Actor
Registered User

## Preconditions
- User has a valid account
- User is not already logged in

## Main Flow
1. User navigates to login page
2. User enters email address
3. User enters password
4. User clicks "Login" button
5. System validates credentials
6. System redirects user to dashboard

## Alternative Flows
- **Invalid credentials**: System shows error message, user tries again
- **Account locked**: System shows lockout message with contact info
- **Forgot password**: User clicks "Forgot Password" link, system sends reset email

## Postconditions
- User is authenticated
- User session is created
- User has access to protected features
```

## Evolution Path

The Simple methodology is designed to grow with your project:

### Phase 1: Start Simple
- Basic use case structure
- Essential scenarios only
- Minimal metadata

### Phase 2: Add Details
- More comprehensive alternative flows
- Business value descriptions
- Acceptance criteria

### Phase 3: Choose Direction
- **More testing focus?** → Migrate to Testing methodology
- **Complex business analysis?** → Migrate to Business methodology
- **Need more structure?** → Migrate to Business methodology

You can regenerate any use case with a different methodology using:
```bash
mucm regenerate UC-XXX-001 --methodology business
```

## Template Customization

### Common Customizations
```markdown
## Business Value
Why this use case matters to the business

## Technical Notes
Implementation considerations or constraints

## Acceptance Criteria
- [ ] Specific testable conditions
- [ ] That define "done"

## Dependencies
- Requires UC-REG-001 (User Registration)
- Uses external payment service
```

### Persona Integration
When using persona templates, the Simple methodology focuses on straightforward user descriptions:

```markdown
## Primary Persona: Sarah (Customer)
- **Goal**: Quick and easy login
- **Pain Points**: Forgotten passwords, complex forms
- **Behavior**: Uses mobile device 70% of the time
```

## Tools and Integration

### Recommended Tools
- **Static Site Generators**: MkDocs, Docusaurus, GitBook
- **Version Control**: Git with pull request reviews
- **Issue Tracking**: Link use cases to GitHub/GitLab issues
- **Test Tools**: Manual testing checklists, basic automation

### CI/CD Integration
```yaml
# Example GitHub Action for documentation
name: Deploy Docs
on:
  push:
    branches: [main]
    paths: ['docs/**']
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs
```

## Literature and Sources

While the Simple methodology doesn't derive from specific academic sources, it incorporates principles from:

### Influential Works
- **"Getting Real" by 37signals** - Emphasis on simplicity and shipping
- **"Lean Startup" by Eric Ries** - Minimum viable product concepts
- **Agile Manifesto** - Working software over comprehensive documentation
- **"Don't Make Me Think" by Steve Krug** - Simplicity in design

### Modern Practices
- **README-driven development** - Documentation-first approach
- **Living documentation** - Keep docs close to code
- **Progressive disclosure** - Show complexity only when needed

## FAQ

**Q: Is Simple methodology suitable for large projects?**
A: Simple methodology works best for projects with fewer than 50 use cases. Larger projects benefit from more formal methodologies like [RUP](rup.md) or [Cockburn](cockburn.md).

**Q: How do I handle complex business rules?**
A: For complex business logic, consider migrating to [Cockburn methodology](cockburn.md) which provides better stakeholder analysis and goal decomposition.

**Q: Can I add custom fields?**
A: Absolutely! The Simple methodology encourages adding any sections that provide value to your team.

**Q: How do I ensure consistency across team members?**
A: Create a team style guide with examples and use pull request reviews to maintain consistency.

---

## Next Steps

- **[Try it out](../getting-started.md)** - Initialize a project with Simple methodology
- **[See examples](../template-comparisons.md)** - Compare with other methodologies
- **[Customize templates](../template-customization.md)** - Adapt to your team's needs
- **[Migrate later](../migration.md)** - Evolve to other methodologies as needed