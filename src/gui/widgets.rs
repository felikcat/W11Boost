use super::tweaks::{GpoOp, RegistryOp, RegistryValue, Tweak};
use eframe::egui::{self, Color32, RichText, TextBuffer};

struct ReadOnlyBuffer(String);

impl TextBuffer for ReadOnlyBuffer
{
        fn as_str(&self) -> &str
        {
                &self.0
        }
        fn insert_text(&mut self, _text: &str, _char_index: usize) -> usize
        {
                0
        }
        fn delete_char_range(&mut self, _char_range: std::ops::Range<usize>) {}

        fn is_mutable(&self) -> bool
        {
                false
        }

        fn type_id(&self) -> std::any::TypeId
        {
                std::any::TypeId::of::<Self>()
        }
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
                let mut text = ReadOnlyBuffer(messages.join("\n"));
                ui.add(egui::TextEdit::multiline(&mut text)
                        .font(egui::FontId::monospace(12.0))
                        .desired_width(f32::INFINITY)
                        .frame(false));
        }
}
