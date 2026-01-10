use super::tweaks::{GpoOp, RegistryOp, RegistryValue, Tweak, TweakEffect};
use eframe::egui::{self, Color32, RichText};

pub fn render_effect_badge(ui: &mut egui::Ui, effect: TweakEffect)
{
        let (effect_text, effect_color) = match effect {
                TweakEffect::Immediate => ("Immediate", egui::Color32::from_rgb(100, 220, 100)),
                TweakEffect::ExplorerRestart => ("Explorer Restart", egui::Color32::from_rgb(255, 180, 50)),
                TweakEffect::Logoff => ("Logoff", egui::Color32::from_rgb(255, 120, 50)),
                TweakEffect::Restart => ("Restart", egui::Color32::from_rgb(255, 80, 80)),
        };

        ui.add_space(4.0);
        ui.label(RichText::new(effect_text).small().color(effect_color).strong());
}

pub fn render_registry_op(ui: &mut egui::Ui, op: &RegistryOp)
{
        let value_str = match &op.value {
                RegistryValue::Dword(v) => format!("{v} (DWORD)"),
                RegistryValue::String(s) => format!("\"{s}\" (String)"),
                RegistryValue::ExpandSz(s) => format!("\"{s}\" (ExpandString)"),
                RegistryValue::Binary(v) => format!("{v:?} (Binary)"),
                RegistryValue::Delete => "DELETE".to_string(),
                RegistryValue::DeleteKey => "DELETE KEY".to_string(),
        };

        let stock_str = match &op.stock_value {
                RegistryValue::Dword(v) => format!("{v} (DWORD)"),
                RegistryValue::String(s) => format!("\"{s}\" (String)"),
                RegistryValue::ExpandSz(s) => format!("\"{s}\" (ExpandString)"),
                RegistryValue::Binary(v) => format!("{v:?} (Binary)"),
                RegistryValue::Delete => "NOT PRESENT (Value)".to_string(),
                RegistryValue::DeleteKey => "NOT PRESENT (Key)".to_string(),
        };

        ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("{}:", op.hkey))
                        .strong()
                        .monospace()
                        .size(14.0)
                        .color(ui.style().visuals.selection.stroke.color));
                ui.label(RichText::new(format!("{}\\{}", op.subkey, op.value_name))
                        .monospace()
                        .size(14.0)
                        .weak());
        });

        ui.horizontal_wrapped(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("Target:")
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(200, 200, 200)));
                ui.label(RichText::new(format!(" = {}", value_str))
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(115, 179, 242)));
        });

        ui.horizontal_wrapped(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("Stock: ")
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(200, 200, 200)));
                ui.label(RichText::new(format!(" = {}", stock_str))
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(255, 165, 0)));
        });

        ui.add_space(4.0);
}

pub fn render_gpo_op(ui: &mut egui::Ui, op: &GpoOp)
{
        let value_str = match &op.value {
                RegistryValue::Dword(v) => format!("{v} (DWORD)"),
                RegistryValue::String(s) => format!("\"{s}\" (String)"),
                RegistryValue::ExpandSz(s) => format!("\"{s}\" (ExpandString)"),
                RegistryValue::Binary(v) => format!("{v:?} (Binary)"),
                RegistryValue::Delete => "DELETE".to_string(),
                RegistryValue::DeleteKey => "DELETE KEY".to_string(),
        };

        ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("GPO (Machine):")
                        .strong()
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(255, 100, 255))); // Magenta for GPO
                ui.label(RichText::new(format!("{}\\{}", op.subkey, op.value_name))
                        .monospace()
                        .size(14.0)
                        .weak());
        });

        ui.horizontal_wrapped(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new("Target:")
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(200, 200, 200)));
                ui.label(RichText::new(format!(" = {}", value_str))
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(115, 179, 242)));
        });

        ui.add_space(4.0);
}

pub fn render_command(ui: &mut egui::Ui, command: &str)
{
        ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("Command:")
                        .strong()
                        .monospace()
                        .size(14.0)
                        .color(Color32::from_rgb(100, 255, 100))); // Green for commands
                ui.add_space(4.0);
        });

        egui::Frame::group(ui.style())
                .fill(Color32::from_rgba_premultiplied(10, 10, 10, 200))
                .inner_margin(6.0)
                .show(ui, |ui| {
                        ui.monospace(command);
                });

        ui.add_space(4.0);
}

pub fn render_tweak_description_column(ui: &mut egui::Ui, tweak: &'static Tweak)
{
        ui.vertical(|ui| {
                ui.label(RichText::new("Description").strong().size(16.0));
                ui.add_space(4.0);
                egui::Frame::group(ui.style())
                        .fill(ui.style().visuals.faint_bg_color)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                                ui.label(tweak.description);
                        });
        });
}

pub fn render_tweak_impact_column(ui: &mut egui::Ui, tweak: &'static Tweak)
{
        ui.vertical(|ui| {
                ui.label(RichText::new("Impact & Effect").strong().size(16.0));
                ui.add_space(4.0);
                egui::Frame::group(ui.style())
                        .fill(ui.style().visuals.faint_bg_color)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                                let desc = tweak.effect.description();
                                let is_logoff = desc == "Requires sign out/sign in";

                                if desc != "Requires restart" && !is_logoff {
                                        ui.label(desc);
                                }
                                if tweak.requires_restart {
                                        if desc != "Requires restart" {
                                                ui.add_space(2.0);
                                        }
                                        ui.label(RichText::new("\u{26A0} Requires restart")
                                                .color(Color32::from_rgb(255, 100, 100)));
                                }
                                if is_logoff {
                                        ui.label(RichText::new("\u{26A0} Requires sign out/sign in")
                                                .color(Color32::from_rgb(255, 190, 40)));
                                }
                        });
        });
}

pub fn generate_highlight_job(text: &str, search_query: &str, theme_color: Color32) -> egui::text::LayoutJob
{
        let mut layout_job = egui::text::LayoutJob::default();

        if search_query.is_empty() {
                layout_job.append(
                        text,
                        0.0,
                        egui::TextFormat {
                                font_id: egui::FontId::monospace(14.0),
                                color: theme_color,
                                ..Default::default()
                        },
                );
        } else {
                let haystack = text.to_lowercase();
                let needle = search_query.to_lowercase();
                let mut last_end = 0;

                for (start, part) in haystack.match_indices(&needle) {
                        if start > last_end {
                                layout_job.append(
                                        &text[last_end..start],
                                        0.0,
                                        egui::TextFormat {
                                                font_id: egui::FontId::monospace(14.0),
                                                color: theme_color,
                                                ..Default::default()
                                        },
                                );
                        }

                        let match_end = start + part.len();
                        layout_job.append(
                                &text[start..match_end],
                                0.0,
                                egui::TextFormat {
                                        font_id: egui::FontId::monospace(14.0),
                                        background: Color32::from_rgb(255, 255, 0),
                                        color: Color32::BLACK,
                                        ..Default::default()
                                },
                        );

                        last_end = match_end;
                }

                if last_end < text.len() {
                        layout_job.append(
                                &text[last_end..],
                                0.0,
                                egui::TextFormat {
                                        font_id: egui::FontId::monospace(14.0),
                                        color: theme_color,
                                        ..Default::default()
                                },
                        );
                }
        }
        layout_job
}

pub fn render_log_messages_list(ui: &mut egui::Ui, messages: &[String])
{
        if messages.is_empty() {
                ui.weak("No log messages yet.");
        } else {
                for msg in messages {
                        ui.label(RichText::new(msg).monospace().size(12.0));
                }
        }
}
