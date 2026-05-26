use bevy::prelude::*;

// --- Button (WARM PAPER / KRAFT TINTS) ---
// Multiply a warm paper texture. Hover = catching warm sunlight,
// press = dipping into a soft warm shade. Tints stay in the brown/amber
// family so they read against the leather background instead of fighting it.
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.96, 0.94, 0.90); // warm paper, not stark white
pub const BUTTON_HOVERED: Color = Color::srgb(1.00, 0.98, 0.90); // warm sun highlight
pub const BUTTON_PRESSED: Color = Color::srgb(0.86, 0.78, 0.66); // soft warm shade
pub const BUTTON_TEXT: Color = Color::srgb(0.23, 0.15, 0.09); // deep roast-brown ink

// --- Text on the brown background ---
pub const HEADER_TEXT: Color = Color::srgb(0.99, 0.96, 0.92); // warm cream white
pub const LABEL_TEXT: Color = Color::srgb(0.91, 0.83, 0.72); // pale parchment

// --- Screen background tint: warm kraft/leather (if you tint behind the texture) ---
pub const SCREEN_BACKGROUND: Color = Color::srgb(0.27, 0.20, 0.16); // dark leather brown

// --- Accent (sunny pop that belongs on the brown) ---
pub const SEGMENT_FILLED: Color = Color::srgb(0.96, 0.72, 0.25); // warm sun/brick yellow
pub const SEGMENT_EMPTY: Color = Color::srgb(0.43, 0.32, 0.22); // dim ember-brown (unfilled)
