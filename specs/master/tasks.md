# Tasks: System Tray RAM Monitor

## Phase 1: Setup (Project Initialization)

- [X] T001 Create project structure per implementation plan
- [ ] T002 Initialize Rust project with `cargo init` in repository root
- [X] T003 Add `windows-rs` (v0.52+) and `tray-icon` (v0.14+) crates to `Cargo.toml`
- [X] T004 Configure `Cargo.toml` for size optimization (LTO, strip symbols, panic=abort)
- [ ] T005 Set up Git repository and initial commit with project structure

## Phase 2: Foundational (Blocking Prerequisites)

- [X] T006 Implement Windows message loop in `src/main.rs`
- [X] T007 Create utility function for polling RAM data using `GlobalMemoryStatusEx`
- [X] T008 Create basic system tray icon using `tray-icon` crate

## Phase 3: User Story 1 - Display Real-Time RAM Usage (Priority: P1)

- [X] T009 [US1] Display RAM usage percentage as overlay on system tray icon
- [X] T010 [US1] Update tray icon every second with latest RAM usage

## Phase 4: User Story 2 - Tooltip with Detailed Stats (Priority: P2)

- [X] T011 [US2] Display tooltip with Used/Total/Free RAM in GB when hovering over tray icon
- [X] T012 [US2] Update tooltip with latest RAM statistics every second

## Phase 5: User Story 3 - Right-Click Context Menu (Priority: P2)

- [X] T013 [US3] Add right-click context menu to tray icon
- [X] T014 [US3] Implement "Exit" option to close the application

## Phase 6: User Story 4 - Auto-Start Capability (Priority: P3)

- [ ] T015 [US4] Add option to enable auto-start by writing to Windows registry
- [ ] T016 [US4] Verify application starts automatically on Windows login when auto-start is enabled

## Final Phase: Polish & Cross-Cutting Concerns

- [ ] T017 Ensure binary size is under 2MB
- [ ] T018 Validate performance metrics (<0.5% CPU, <5MB memory footprint, sub-1ms polling)
- [ ] T019 Add error handling for RAM polling failures
- [ ] T020 Add fallback message for tooltip when RAM data is unavailable
- [ ] T021 Test application on Windows 10 and later for compatibility

## Dependencies

- Phase 1 must complete before Phase 2
- Phase 2 must complete before any user story phases
- User Story 1 (Phase 3) must complete before User Stories 2 and 3 (Phases 4 and 5)
- User Stories 2 and 3 can be developed in parallel
- User Story 4 (Phase 6) is independent of other user stories

## Parallel Execution Opportunities

- Tasks in Phases 4 and 5 can be executed in parallel
- Tasks T017, T018, and T019 in the Final Phase can be executed in parallel

## Implementation Strategy

- Deliver User Story 1 (Phase 3) as the MVP
- Incrementally add User Stories 2, 3, and 4 in subsequent releases
- Final Phase tasks to be completed before production release