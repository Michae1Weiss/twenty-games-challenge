use bevy::prelude::*;

// --- Button (PAPER TEXTURE TINTS) ---
// Multiply the white paper texture. Hover = catching sunlight (cool white),
// press = a soft sky-blue shadow, like the paper dipping into shade.
pub const BUTTON_BACKGROUND: Color = Color::srgb(1.00, 1.00, 1.00); // paper as-is
pub const BUTTON_HOVERED: Color = Color::srgb(0.90, 0.96, 1.00); // cool sky highlight
pub const BUTTON_PRESSED: Color = Color::srgb(0.72, 0.82, 0.92); // soft blue shadow
pub const BUTTON_TEXT: Color = Color::srgb(0.10, 0.16, 0.26); // deep ink-navy

// --- Text on the sky background ---
pub const HEADER_TEXT: Color = Color::srgb(1.00, 1.00, 1.00); // crisp cloud white
pub const LABEL_TEXT: Color = Color::srgb(0.88, 0.94, 1.00); // pale sky-white

// --- Screen background: bright sky blue, not dark ---
pub const SCREEN_BACKGROUND: Color = Color::srgb(0.42, 0.66, 0.88); // open sky blue

// --- Accent (sunny pop against the blue) ---
pub const SEGMENT_FILLED: Color = Color::srgb(1.00, 0.84, 0.30); // warm sun yellow
pub const SEGMENT_EMPTY: Color = Color::srgb(0.34, 0.54, 0.74); // muted deeper sky
