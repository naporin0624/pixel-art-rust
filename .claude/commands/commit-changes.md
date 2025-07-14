---
description: Make fine-grained git commits with proper commit messages
---

# Commit Changes

I'll create fine-grained git commits by analyzing changes at a granular level. This command will:

1. Check the current git status
2. Review all changes (staged and unstaged)
3. Group related changes into logical units
4. Create separate commits for different types of changes:
   - Feature additions
   - Bug fixes
   - Refactoring
   - Configuration changes
   - Test updates
   - Documentation changes
5. Add only relevant files for each commit type
6. Create descriptive commit messages following the project's conventions
7. Make multiple commits with proper formatting

## Fine-Grained Commit Strategy

- **Feature commits**: New functionality grouped by related components
- **Fix commits**: Bug fixes isolated by scope and impact
- **Refactor commits**: Code improvements without behavior changes
- **Config commits**: Environment, build, or tool configuration changes
- **Test commits**: Test additions or modifications
- **Docs commits**: Documentation updates

This ensures clean git history with atomic, meaningful commits that follow established patterns and make code review easier.
