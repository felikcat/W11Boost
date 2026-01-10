// W11Boost dark blue theme (converted from ImGui preset)

use eframe::egui::{
        Color32, Context, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Stroke, TextStyle,
        ThemePreference, style::WidgetVisuals,
};

// Colors with 80% opacity for transparent window effect
// Alpha 204 = 80% opacity
const TEXT: Color32 = Color32::from_rgba_premultiplied(235, 242, 250, 255);
const WINDOW_BG: Color32 = Color32::from_rgba_premultiplied(18, 28, 41, 204);
const CHILD_BG: Color32 = Color32::from_rgba_premultiplied(23, 34, 50, 204);
const BORDER: Color32 = Color32::from_rgba_premultiplied(31, 45, 62, 89);
const ACCENT: Color32 = Color32::from_rgba_premultiplied(115, 179, 242, 255);
const BUTTON: Color32 = Color32::from_rgba_premultiplied(28, 43, 64, 200);
const BUTTON_HOVERED: Color32 = Color32::from_rgba_premultiplied(52, 82, 119, 220);
const BUTTON_ACTIVE: Color32 = Color32::from_rgba_premultiplied(77, 122, 179, 255);
const SEPARATOR_HOVERED: Color32 = Color32::from_rgba_premultiplied(70, 109, 159, 199);
const TEXT_SELECTED_BG: Color32 = Color32::from_rgba_premultiplied(31, 49, 71, 89);

/// Apply the custom dark blue theme
pub fn apply_dark_theme(ctx: &Context)
{
        // Force dark theme regardless of Windows system settings
        ctx.options_mut(|opts| {
                opts.theme_preference = ThemePreference::Dark;
        });

        setup_fonts(ctx);

        let mut style = (*ctx.style()).clone();

        // Modern text styles
        style.text_styles = [
                (TextStyle::Small, FontId::new(11.0, FontFamily::Proportional)),
                (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Button, FontId::new(14.0, FontFamily::Proportional)),
                (TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional)),
                (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
        ]
        .into();

        // Modern spacing
        style.spacing.item_spacing = [6.0, 4.0].into();
        style.spacing.button_padding = [10.0, 6.0].into();
        style.spacing.menu_margin = Margin::same(4);
        style.spacing.indent = 12.0;

        // Modern rounded corners
        let rounding = CornerRadius::same(8);

        // Configure visuals
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = WINDOW_BG;
        style.visuals.window_fill = WINDOW_BG;
        style.visuals.extreme_bg_color = Color32::from_rgb(14, 21, 32);
        style.visuals.faint_bg_color = CHILD_BG;
        style.visuals.override_text_color = Some(TEXT);

        // Non-interactive widgets
        style.visuals.widgets.noninteractive = WidgetVisuals {
                bg_fill: Color32::TRANSPARENT,
                weak_bg_fill: Color32::TRANSPARENT,
                bg_stroke: Stroke::NONE,
                fg_stroke: Stroke::new(1.0, TEXT),
                corner_radius: rounding,
                expansion: 0.0,
        };

        // Inactive widgets
        style.visuals.widgets.inactive = WidgetVisuals {
                bg_fill: BUTTON,
                weak_bg_fill: BUTTON,
                bg_stroke: Stroke::new(1.0, BORDER),
                fg_stroke: Stroke::new(1.0, TEXT),
                corner_radius: rounding,
                expansion: 0.0,
        };

        // Hovered widgets
        style.visuals.widgets.hovered = WidgetVisuals {
                bg_fill: BUTTON_HOVERED,
                weak_bg_fill: BUTTON_HOVERED,
                bg_stroke: Stroke::new(1.0, SEPARATOR_HOVERED),
                fg_stroke: Stroke::new(1.0, TEXT),
                corner_radius: rounding,
                expansion: 0.0,
        };

        // Active/pressed widgets
        style.visuals.widgets.active = WidgetVisuals {
                bg_fill: BUTTON_ACTIVE,
                weak_bg_fill: BUTTON_ACTIVE,
                bg_stroke: Stroke::new(1.0, ACCENT),
                fg_stroke: Stroke::new(1.0, TEXT),
                corner_radius: rounding,
                expansion: 0.0,
        };

        // Selection
        style.visuals.selection.bg_fill = TEXT_SELECTED_BG;
        style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);

        // Window styling
        style.visuals.window_corner_radius = rounding;
        style.visuals.window_shadow.color = Color32::from_black_alpha(80);
        style.visuals.window_shadow.offset = [0, 8];
        style.visuals.window_shadow.blur = 16;

        // Sliders and handles
        style.visuals.handle_shape = eframe::egui::style::HandleShape::Circle;

        ctx.set_style(style);
}

fn setup_fonts(ctx: &Context)
{
        let mut fonts = FontDefinitions::default();

        // Embed Roboto font
        fonts.font_data.insert(
                "Roboto".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                        "../../fonts/Roboto-VariableFont_wdth,wght.ttf"
                ))),
        );

        // Load Segoe UI Emoji for color icons
        const EMOJI_FONT: &str = "C:\\Windows\\Fonts\\seguiemj.ttf";
        if let Ok(emoji_data) = std::fs::read(EMOJI_FONT) {
                fonts.font_data.insert(
                        "Segoe UI Emoji".to_owned(),
                        std::sync::Arc::new(FontData::from_owned(emoji_data)),
                );
        }

        // Configure Proportional
        if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
                family.insert(0, "Roboto".to_owned());
                family.push("Segoe UI Emoji".to_owned());
        }

        // Configure Monospace
        if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
                family.push("Segoe UI Emoji".to_owned());
        }

        ctx.set_fonts(fonts);
}
