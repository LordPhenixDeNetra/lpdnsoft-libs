//! Thème egui commun aux apps LPDNSoft et quelques composants.

use egui::{Color32, CornerRadius, Frame, InnerResponse, Theme, Ui, vec2};

/// Couleur d'accent LPDNSoft.
pub const ACCENT: Color32 = Color32::from_rgb(0x2E, 0x7D, 0xD2);

/// Applique le thème aux deux variantes, claire et sombre ; le système choisit laquelle.
pub fn install(ctx: &egui::Context) {
    for theme in [Theme::Light, Theme::Dark] {
        ctx.style_mut_of(theme, |style| {
            let v = &mut style.visuals;
            v.selection.bg_fill = ACCENT;
            v.hyperlink_color = ACCENT;
            let radius = CornerRadius::same(8);
            v.window_corner_radius = radius;
            v.menu_corner_radius = radius;
            for w in [
                &mut v.widgets.noninteractive,
                &mut v.widgets.inactive,
                &mut v.widgets.hovered,
                &mut v.widgets.active,
                &mut v.widgets.open,
            ] {
                w.corner_radius = radius;
            }
            style.spacing.item_spacing = vec2(8.0, 8.0);
        });
    }
}

/// État affiché par une pastille de couleur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    Off,
    Busy,
    On,
    Alert,
}

pub fn status_color(tone: Tone) -> Color32 {
    match tone {
        Tone::Off => Color32::from_rgb(0x9E, 0x9E, 0x9E),
        Tone::Busy => Color32::from_rgb(0xF9, 0xA8, 0x25),
        Tone::On => Color32::from_rgb(0x2E, 0x7D, 0x32),
        Tone::Alert => Color32::from_rgb(0xC6, 0x28, 0x28),
    }
}

/// Carte : bloc encadré sur fond légèrement teinté.
pub fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Frame::group(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(12.0)
        .show(ui, add)
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Theme;
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    #[test]
    fn install_sets_accent_in_both_themes() {
        let ctx = egui::Context::default();
        install(&ctx);
        for theme in [Theme::Dark, Theme::Light] {
            assert_eq!(ctx.style_of(theme).visuals.selection.bg_fill, ACCENT);
        }
    }

    #[test]
    fn status_colors_are_distinct() {
        let all = [Tone::Off, Tone::Busy, Tone::On, Tone::Alert].map(status_color);
        for (i, a) in all.iter().enumerate() {
            assert!(all[i + 1..].iter().all(|b| a != b));
        }
    }

    #[test]
    fn card_shows_its_content() {
        let harness = Harness::new_ui(|ui| {
            card(ui, |ui| ui.label("x"));
        });
        harness.get_by_label("x");
    }
}
