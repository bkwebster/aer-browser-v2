# Aer Browser - Session Management System

## Overview
This document establishes a session tracking and documentation system for continuous development of the Aer Browser project across multiple Claude interactions.

## Core Session Files

### 1. `CLAUDE_NOTES.md` - Primary Session Handoff
**Purpose**: Detailed technical notes for Claude to resume development
**Updated**: At the END of every session
**Contents**:
- Current code status and compilation state
- Specific next steps with code examples
- Research topics and technical insights
- Test cases to validate
- Files to focus on next

### 2. `WARP.md` - Project Documentation & History
**Purpose**: Overall project status and session chronology
**Updated**: During significant milestones and at session end
**Contents**:
- Project overview and architecture
- Development environment setup
- Session tracking with dates and accomplishments
- Long-term roadmap and planning

### 3. `DECISIONS.md` - Architecture & Design Decisions
**Purpose**: Record of major technical decisions and rationale
**Updated**: When making significant architectural choices
**Contents**:
- Technology choices (CEF vs alternatives, database selection, etc.)
- Architecture decisions with reasoning
- Trade-offs and alternatives considered

### 4. `SESSION_LOG.md` - Chronological Activity Log
**Purpose**: Detailed chronological record of all development activities
**Updated**: CONTINUOUSLY during development sessions
**Contents**:
- Timestamped development activities
- Commands run and their outcomes
- Problems encountered and solutions
- Code changes made

## Session Management Rules for Claude

### 🔥 MANDATORY SESSION START PROTOCOL
When beginning ANY new session with this project:

1. **ALWAYS** read the following files in order:
   ```
   - SESSION_LOG.md (last 50 entries)
   - CLAUDE_NOTES.md (complete file)
   - WARP.md (current status section)
   - DECISIONS.md (recent decisions)
   ```

2. **VERIFY** current code state:
   ```bash
   cd /Users/atelaer/Developer/aer-browser/src-tauri && cargo check
   cd /Users/atelaer/Developer/aer-browser/ui && ls package.json
   ```

3. **UPDATE** `SESSION_LOG.md` with session start entry

### 🔥 MANDATORY SESSION END PROTOCOL
When ending ANY session with this project:

1. **UPDATE** `CLAUDE_NOTES.md` with:
   - Current status summary
   - Detailed next steps
   - Any new insights or research findings
   - Priority tasks for next session

2. **UPDATE** `WARP.md` session notes section with:
   - What was accomplished this session
   - Current project status
   - Updated timestamp

3. **UPDATE** `SESSION_LOG.md` with final entries

4. **COMMIT AND PUSH** all changes:
   ```bash
   git add -A
   git commit -m "Session YYYY-MM-DD: [brief summary]"
   git push origin main
   ```

### 🔄 CONTINUOUS UPDATE RULES

#### Update `SESSION_LOG.md` when:
- Starting any development task
- Running significant commands
- Encountering errors or problems
- Making code changes
- Completing tasks or milestones
- Making discoveries or insights

#### Update `DECISIONS.md` when:
- Choosing between technical alternatives
- Making architectural decisions
- Changing project direction
- Selecting libraries or frameworks
- Deciding on implementation approaches

#### Update `WARP.md` when:
- Project status changes significantly
- Major milestones are reached
- Architecture evolves
- New phases of development begin

#### Update `CLAUDE_NOTES.md` when:
- Session is ending
- Significant blockers are encountered
- Research yields new approaches
- Implementation strategy changes

## File Templates

### SESSION_LOG.md Entry Format:
```markdown
### [YYYY-MM-DD HH:MM] - [Activity Title]
**Status**: [Started/In Progress/Completed/Blocked]
**Action**: [What was done]
**Result**: [Outcome/Result]
**Next**: [What's next, if applicable]
```

### CLAUDE_NOTES.md Update Format:
```markdown
## Session [YYYY-MM-DD] - [Session Title]

**Status**: [Brief status]
**Completed**:
- [List of completed items]

**In Progress**:
- [Current work items]

**Next Steps**:
1. [Priority 1 task with details]
2. [Priority 2 task with details]

**Technical Notes**:
- [Key insights or findings]
```

## Git Commit Standards

### Commit Message Format:
```
Session YYYY-MM-DD: [Brief summary]

[Category] [Details]:
- [Specific change 1]
- [Specific change 2]

[Category] [Details]:
- [More changes]

🔄 Ready for next session: [next priority]
```

### Categories:
- ✅ **Completed**: Features/fixes that work
- 🔄 **Progress**: Ongoing development
- 📝 **Docs**: Documentation updates
- 🐛 **Fix**: Bug fixes
- 🚀 **Feature**: New functionality
- 🔬 **Research**: Investigation/exploration

## Emergency Recovery Protocol

If session tracking files are missing or corrupted:

1. Check git history: `git log --oneline -10`
2. Restore from last commit: `git checkout HEAD -- [filename]`
3. If completely lost, recreate using:
   - Git commit messages
   - Current code state analysis
   - Any remaining documentation

## Validation Checklist

Before ending any session, verify:
- [ ] All session files updated
- [ ] Code compiles without errors
- [ ] Next steps are clearly documented
- [ ] All changes committed and pushed
- [ ] Session logged in `SESSION_LOG.md`

## Integration with Development Workflow

### Before coding:
1. Read session files
2. Log session start
3. Verify environment

### During coding:
1. Log significant actions
2. Update relevant docs when making decisions
3. Commit frequently with good messages

### After coding:
1. Update all session files
2. Final commit and push
3. Verify next session readiness

---

**Created**: 2025-09-02 08:01 UTC  
**Purpose**: Ensure continuity and context preservation across Claude sessions  
**Status**: Active - Must be followed for all development sessions
