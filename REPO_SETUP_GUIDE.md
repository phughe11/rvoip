# GitHub Repository Setup Guide

**Purpose**: Configure repository settings after deploying improvements  
**Status**: Ready for manual configuration  
**Date**: January 11, 2026

---

## ✅ Already Deployed

- [x] CI/CD workflow (`.github/workflows/ci.yml`)
- [x] Issue templates (`.github/ISSUE_TEMPLATE/`)
- [x] PR template (`.github/PULL_REQUEST_TEMPLATE.md`)
- [x] All documentation files

---

## 🔧 Manual Configuration Required

### Step 1: Enable GitHub Actions

**Location**: Repository Settings → Actions → General

1. **Actions permissions**:
   - [ ] Select "Allow all actions and reusable workflows"

2. **Workflow permissions**:
   - [ ] Select "Read and write permissions"
   - [ ] Check "Allow GitHub Actions to create and approve pull requests"

3. **Save changes**

---

### Step 2: Enable Discussions

**Location**: Repository Settings → General (scroll to Features)

1. **Features section**:
   - [ ] Check "Discussions"

2. **Set up discussions**:
   - Click "Set up discussions"
   - Create categories:
     - **General** (open-ended discussion)
     - **Ideas** (feature requests and ideas)
     - **Q&A** (questions and answers)
     - **Show and Tell** (share projects/examples)
     - **Announcements** (important updates)

---

### Step 3: Configure Branch Protection

**Location**: Repository Settings → Branches

1. **Add rule**:
   - Branch name pattern: `main`
   - [ ] Require a pull request before merging
   - [ ] Require approvals: 1
   - [ ] Dismiss stale pull request approvals when new commits are pushed
   - [ ] Require status checks to pass before merging
     - Search for and select: `test (ubuntu-latest)`
     - Search for and select: `test (macos-latest)`
     - Search for and select: `test (windows-latest)`
   - [ ] Require conversation resolution before merging
   - [ ] Do not allow bypassing the above settings

2. **Create** or **Save changes**

---

### Step 4: Configure Issue Templates

**Location**: Repository Settings → General (scroll to Features)

1. **Features section**:
   - Click "Set up templates" next to "Issues"
   - Verify templates appear:
     - [ ] Bug report
     - [ ] Feature request
     - [ ] Question

---

### Step 5: Add Repository Topics

**Location**: Repository Settings → General (scroll to bottom)

Add these topics:
- `rust`
- `voip`
- `sip`
- `telephony`
- `real-time`
- `networking`
- `webRTC`
- `telecom`

---

### Step 6: Configure Security Policy

**Location**: Repository Settings → General (scroll to bottom)

1. **Security** section:
   - [ ] Check "Allow dependabot to open security advisories"
   - [ ] Check "Allow GitHub to perform automated security updates"

---

### Step 7: Set Repository Description

**Location**: Repository Settings → General

Update description to:
```
A production-ready, 100% Rust VoIP stack - alternative to FreeSWITCH/Asterisk
```

---

### Step 8: Add Repository Website

**Location**: Repository Settings → General

Set website to:
```
https://phughe11.github.io/rvoip/
```

---

## 📊 Verification Checklist

### Actions Tab
- [ ] CI workflow appears and runs on pushes
- [ ] All jobs pass (Linux, macOS, Windows)
- [ ] Status checks appear in PRs

### Discussions Tab
- [ ] Discussions enabled
- [ ] All categories created
- [ ] Can create new discussions

### Branch Protection
- [ ] Cannot push directly to main
- [ ] PRs require CI to pass
- [ ] PRs require approval
- [ ] PRs require conversation resolution

### Issues Tab
- [ ] Templates appear when creating issues
- [ ] Bug report, feature request, question templates available

---

## 🚨 Troubleshooting

### CI Not Running
- Check Actions permissions in Settings
- Verify workflow file exists: `.github/workflows/ci.yml`
- Check workflow syntax (GitHub will show errors)

### Branch Protection Not Working
- Ensure CI has run at least once to create status checks
- Status checks may take a few minutes to appear
- Check that job names match exactly

### Templates Not Appearing
- Verify files are in correct location: `.github/ISSUE_TEMPLATE/`
- Check YAML frontmatter syntax
- May need to refresh repository page

### Discussions Not Enabled
- Check repository permissions (must be admin)
- Some organizations may restrict discussions

---

## 📈 Next Steps After Configuration

1. **Post Community Announcement** in Discussions
2. **Monitor CI Results** and address any warnings
3. **Test Issue/PR Templates** by creating sample issues
4. **Set Up Code Coverage** (optional, requires Codecov account)
5. **Configure Dependabot** for dependency updates

---

## 📞 Support

If you encounter issues:
- Check [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md) for detailed steps
- Open an issue using the bug report template
- Ask in Discussions (after enabling)

---

**Configuration Guide**: January 11, 2026  
**Repository**: phughe11/rvoip  
**Status**: Ready for manual setup
