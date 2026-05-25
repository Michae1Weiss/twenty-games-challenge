## Game 01 - Breakout
- [ ] User input is spread all over the code with hardcoded KeyCodes. It would be nice to consolidate the input logic in one place and expose actions so callers don't have to be responsible to define key bindings etc.
- [ ] `in_state` is all over the codebase. Replace with `SystemSet`!
