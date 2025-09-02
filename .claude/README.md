# Claude Session Management - Quick Reference

## 🔥 CRITICAL: Session Start Protocol
When Claude begins work on this project:

1. **Read these files first**:
   ```
   SESSION_LOG.md (last 50 entries)
   CLAUDE_NOTES.md (complete file) 
   WARP.md (current status section)
   DECISIONS.md (recent decisions)
   ```

2. **Verify environment**:
   ```bash
   cd /Users/atelaer/Developer/aer-browser/src-tauri && cargo check
   cd /Users/atelaer/Developer/aer-browser/ui && ls package.json
   ```

3. **Log session start** in `SESSION_LOG.md`

## 📝 Documentation Files

- **`SESSION_LOG.md`** - Chronological activity log (update continuously)
- **`CLAUDE_NOTES.md`** - Detailed handoff notes (update at session end)
- **`WARP.md`** - Project overview and status (update at milestones)  
- **`DECISIONS.md`** - Architecture decisions (update when making choices)
- **`SESSION_SYSTEM.md`** - Complete rules and protocols

## 🔄 Session End Protocol

1. Update all documentation files
2. Commit and push changes
3. Ensure next session readiness

## Current Project Status

**Backend**: ✅ Compiles cleanly  
**Frontend**: ✅ Bun dev server working  
**Next Priority**: Test full `cargo tauri dev` integration

See `CLAUDE_NOTES.md` for detailed next steps.
