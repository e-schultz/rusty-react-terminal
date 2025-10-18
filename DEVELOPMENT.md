# Development Workflow & Lessons Learned

This document captures the development process, architectural decisions, and lessons learned as the project evolves. Updated after major milestones.

## Development Philosophy

### Core Principles
1. **Document as you go**: Every issue includes implementation notes and decisions
2. **Self-verification before review**: Each feature must pass internal checklist
3. **Lessons captured early**: Record what worked/didn't work immediately after implementation
4. **GitHub Issues as source of truth**: All work tracked in issues, not external docs
5. **Minimal dependencies**: Prefer ratatui ecosystem over external crates

### Issue Lifecycle
```
Planning → In Progress → Self Verify → Review → Done
   (✓)         (Code)      (Checklist)  (Polish)  (Merged)
```

Each issue includes:
- **Requirements**: What needs to be done
- **Tasks**: Specific implementation steps
- **Implementation Notes**: Decisions made while coding
- **Lessons**: What was learned from the implementation
- **Blockers**: What stopped progress and how it was resolved

---

## Milestone 1: MVP Implementation (Commit e3bc610)

### What Was Built
- Complete data model (Entry, Section, Program, Record)
- State management for both views
- Keyboard input handling with vim-like navigation
- Basic ratatui rendering
- TOML data loading system
- Command palette infrastructure

### Lessons Learned

#### ✓ What Worked Well
1. **Ratatui 0.28 API is clean and simple**
   - No generic backend parameter needed
   - `Frame` is non-generic, much simpler than older versions
   - `f.area()` method is intuitive
   - Layout constraint system is powerful

2. **Separating state, input, and rendering**
   - Pure functions for rendering made UI stateless
   - State management is centralized and predictable
   - Input handlers are isolated and testable
   - Easy to understand data flow

3. **TOML format for data**
   - Human-readable and easy to edit
   - Strong typing with serde
   - Nested arrays work well for sections/entries
   - Clear structure mirrors the data hierarchy

4. **Vim-like navigation (hjkl)**
   - Natural for terminal users
   - Consistent across both views
   - Easy to remember and type quickly
   - Plus arrow key support for mouse users

5. **Zelij-inspired command palette (`:` prefix)**
   - Modal feel without being intrusive
   - Familiar to Zelij/Neovim users
   - Easy to extend with new commands
   - Can show help inline

#### ✗ What Was Difficult

1. **Ratatui API version mismatch**
   - Initial code used deprecated methods (`size()` instead of `area()`)
   - Buffer cell mutation API changed in 0.28
   - Solution: Simplified cursor rendering to text rather than direct buffer manipulation
   - Lesson: Always check latest docs and examples

2. **Generic Backend parameter confusion**
   - Tried `Frame<B: Backend>` but Frame in 0.28 is non-generic
   - Error messages pointed to right place, but took iteration to fix
   - Lesson: When in doubt, check the actual struct definition

3. **Temporary value lifetime in `format!()`**
   - `Span::styled(&format!(...))` created temporary that couldn't be borrowed
   - Fixed by creating variable before use
   - Lesson: Rust's borrow checker catches real bugs; work with it

4. **Unreachable pattern in match**
   - Matching `Char(c)` before `Char(c) if modifier` made second arm unreachable
   - Fixed by reordering: guards first, then general pattern
   - Lesson: Order matters in match arms

#### ? Unknowns / Questions for Next Iteration

1. How should scrolling work for very long lists?
   - Do we need a scrollbar widget?
   - Should we paginate instead?
   - What's the max reasonable list size in terminal?

2. Should details expand inline or in a separate view?
   - Current: marking `viewing_details` but not rendering it
   - Option A: Show details below the list
   - Option B: Replace list with details view
   - Need to decide based on screen real estate

3. How complex should the command palette get?
   - Current: simple `view <name>` command
   - Could support: `:set focus 85`, `:export json`, etc.
   - Risk: command parser complexity grows quickly

4. Data persistence and saving?
   - Currently data is read-only from TOML
   - Should we support saving changes?
   - Would need file watching and conflict resolution

### Code Quality Observations

**Strengths**:
- Models are clean and derive-friendly
- State transitions are explicit and trackable
- No unwrap() calls except in main (proper error handling)
- Module separation is clear

**Areas for Improvement**:
- Some unused methods (marked with warnings)
- UI rendering could benefit from helper functions
- Input handling could be more modular
- No tests yet (critical for refactoring)

### Technical Debt

| Issue | Severity | Notes |
|-------|----------|-------|
| No unit tests | High | Makes refactoring risky |
| Unused state methods | Low | Clean up once used |
| Cursor rendering simplified | Medium | Could be enhanced later |
| No keyboard config | Low | Hardcoded hjkl navigation |
| Limited error feedback | Medium | Error messages could be more helpful |

---

## Milestone 2: Testing Infrastructure (In Progress)

### Plan
- Add unit tests for data loading
- Add state transition tests
- Set up test data fixtures
- Create integration test template

### Key Questions Before Starting
1. Should tests use real TOML files or embed test data?
2. How to test UI rendering without visual inspection?
3. What's the minimum test coverage target?

---

## Architecture Decisions

### Why Separate State, Input, and UI?

**Decision**: Keep these three layers completely separate

**Rationale**:
- UI rendering is pure (no side effects)
- State changes only happen in specific input handlers
- Makes testing easier (can test each layer independently)
- Easier to add features without touching core logic

**Alternative Considered**: Monolithic view components like React
- Rejected: TUI rendering is simpler; full OOP overhead not needed
- Rejected: Makes testing harder; everything becomes a side effect

### Why TOML Instead of Embedded Data?

**Decision**: Data stored in TOML files in `data/` directory

**Rationale**:
- Users can edit operational patterns without recompiling
- Clear separation of code and data
- Easy to version control data changes
- Reduces binary size

**Alternative Considered**: Embed data in binary with include!()
- Would be simpler deployment (single binary)
- But sacrifices editability and versioning clarity

### Why Vim Keybindings (hjkl)?

**Decision**: Support hjkl plus arrows for navigation

**Rationale**:
- Familiar to terminal/editor users
- Reduces hand movement (especially on laptop)
- Tab to switch views is standard
- Easy to learn and remember

**Alternative Considered**: Mouse + arrow keys only
- Would require implementing mouse tracking in crossterm
- Less efficient for keyboard-focused workflow
- More complex event handling

---

## Performance Considerations

### Current Approach
- Render entire UI every frame (250ms polling interval)
- No caching of rendered output
- Layout recalculated each frame

### Potential Optimizations (not done yet)
- Cache immutable parts of layout
- Only rerender changed areas
- Reduce polling frequency when idle
- Use dirty flag pattern

**Current Performance**: Acceptable for terminal use (~100% CPU is normal during active input)

---

## Future Architectural Decisions Needed

### 1. Plugin System
- Should we support user scripts/plugins?
- How would they interact with TUI?
- What's the use case?

### 2. Data Persistence
- Save user changes to TOML files?
- Auto-save intervals?
- Conflict resolution if file changed externally?

### 3. Multi-Window Support
- Should we support split views (like Zelij)?
- Tab system for multiple instances?
- Window focus and switching?

### 4. Configuration
- Support `~/.evan-tui/config.toml`?
- User-defined keybindings?
- Color themes?

---

## How to Contribute Using This Workflow

### When Starting Work
1. Create/find a GitHub Issue
2. Move to `in-progress` label
3. Document decisions as you code (in issue comments)
4. Update implementation notes with blockers/solutions

### When Finishing Feature
1. Add `self-verify` checklist:
   - [ ] Code compiles without warnings (except known)
   - [ ] Follows project patterns
   - [ ] No duplicate code with existing modules
   - [ ] Comments explain non-obvious logic
   - [ ] Matches architectural design from planning

2. Update issue with:
   - What was implemented
   - How it was tested
   - Lessons learned
   - Remaining questions

3. Create PR with link to issue
4. Request review

### After Merging
- Add milestone summary to DEVELOPMENT.md
- Close related issues
- Create new issues for lessons learned

---

## Key Takeaways

1. **Start with clear architecture**: Separating concerns pays dividends
2. **Document decisions immediately**: Memory fades; write it down now
3. **GitHub Issues are your design log**: Not just task tracking
4. **Lessons matter more than features**: What you learn is reusable
5. **Simple is better than clever**: Vim bindings beat custom keys

---

## Quick Reference: Common Issues & Solutions

### Issue: Code doesn't compile with ratatui
**Solution**: Check ratatui version (currently 0.28). API changes between versions.

### Issue: Terminal state corrupted after crash
**Solution**: Run `reset` command in terminal. Application uses alt screen.

### Issue: Navigation state gets inconsistent
**Solution**: All state mutations go through AppState methods. Don't mutate directly.

### Issue: Command palette input not working
**Solution**: Check if `show_command_palette` flag is set. Input handlers check this first.

---

*Last Updated: Initial Implementation (2025-10-18)*
*Next Review: After Milestone 2 (Testing)*
