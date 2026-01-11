## Description

<!-- Provide a clear description of your changes -->

## Type of Change

<!-- Mark the relevant option with an 'x' -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Code refactoring
- [ ] Test addition/improvement

## Related Issues

<!-- Link related issues using keywords like "Fixes #123" or "Relates to #456" -->

Fixes #
Relates to #

## Component

<!-- Which component does this PR affect? -->

- [ ] sip-core
- [ ] dialog-core
- [ ] media-core
- [ ] rtp-core
- [ ] session-core-v3 (recommended)
- [ ] session-core-v2 (transitional)
- [ ] session-core-v1 (maintenance)
- [ ] call-engine
- [ ] registrar-core
- [ ] Other: ___________

## Testing

<!-- Describe the tests you ran and their results -->

### Test Coverage

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All existing tests pass
- [ ] Doc tests added/updated (if applicable)

### Test Commands Run

```bash
# List the commands you used to test
cargo test -p rvoip-<component>
cargo test --workspace
```

### Test Results

<!-- Paste relevant test output or describe results -->

```
Test output here
```

## Code Quality Checklist

- [ ] My code follows the project's style guidelines
- [ ] I have run `cargo fmt --all`
- [ ] I have run `cargo clippy --workspace -- -D warnings` with no errors
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works

## Documentation

- [ ] I have updated relevant README.md files
- [ ] I have added doc comments to public APIs
- [ ] I have updated CHANGELOG.md (for significant changes)
- [ ] I have added/updated examples (if applicable)
- [ ] I have checked that documentation renders correctly

## Breaking Changes

<!-- If this is a breaking change, describe the impact and migration path -->

**Does this PR introduce breaking changes?**
- [ ] Yes
- [ ] No

**If yes, please describe:**
- What breaks?
- Why is this necessary?
- Migration path for users?
- Should this wait for a major version?

## Performance Impact

<!-- Describe any performance implications -->

- [ ] This change improves performance
- [ ] This change may negatively impact performance
- [ ] This change has no significant performance impact
- [ ] Performance impact is unknown

**If there is a performance impact, please provide details:**

## Backwards Compatibility

- [ ] This change is fully backwards compatible
- [ ] This change may break some use cases (documented above)
- [ ] This change requires version bump (major/minor/patch)

## Screenshots/Logs (if applicable)

<!-- Add screenshots or log output for visual changes or debugging info -->

## Additional Context

<!-- Add any other context about the PR here -->

## Reviewer Notes

<!-- Anything specific you want reviewers to focus on? -->

**Areas needing extra attention:**
- 
- 

**Questions for reviewers:**
- 
- 

## Pre-submission Checklist

<!-- Verify everything before marking as ready for review -->

- [ ] I have read [CONTRIBUTING.md](../CONTRIBUTING.md)
- [ ] I am targeting the correct version (v3 for new features)
- [ ] My branch is up to date with main
- [ ] All CI checks pass (or will pass once pushed)
- [ ] I have resolved any merge conflicts
- [ ] This PR is ready for review

## Post-merge Actions

<!-- What needs to happen after this PR merges? -->

- [ ] Documentation needs to be updated elsewhere
- [ ] Examples need to be updated
- [ ] Release notes need to mention this change
- [ ] Follow-up issues need to be created
- [ ] None

---

**Thank you for contributing to RVOIP!** 🚀

<!-- Maintainer use only -->
## Maintainer Checklist

- [ ] Code review completed
- [ ] Tests pass
- [ ] Documentation reviewed
- [ ] Breaking changes documented
- [ ] CHANGELOG.md updated
- [ ] Version bump needed (yes/no)
