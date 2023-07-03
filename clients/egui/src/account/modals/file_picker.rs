use std::sync::Arc;

use eframe::egui;
use lb::File;

use crate::{model::DocType, theme::Icon, widgets::Button};

pub struct FilePicker {
    core: Arc<lb::Core>,
    panels: Vec<lb::File>,
    target: File,
}
pub struct FilePickerParams {
    pub target: File,
    pub parent: File,
}

impl FilePicker {
    pub fn new(core: Arc<lb::Core>, target: File) -> Self {
        let root = core.get_root().unwrap();

        Self { core, panels: vec![root], target }
    }
}

impl super::Modal for FilePicker {
    type Response = Option<FilePickerParams>;

    fn title(&self) -> &str {
        "File Picker"
    }

    fn show(&mut self, ui: &mut egui::Ui) -> Self::Response {
        ui.set_max_width(750.0);
        egui::ScrollArea::horizontal()
            .stick_to_right(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.set_height(350.0);
                    ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                    for (i, f) in self.panels.clone().iter().enumerate() {
                        show_file_panel(ui, self, f, i);
                        ui.separator();
                    }
                });
            });

        ui.separator();

        egui::Frame::default()
            .inner_margin(egui::Margin::symmetric(20.0, 10.0))
            .show(ui, |ui| show_bottom_bar(ui, self))
            .inner
    }
}

fn show_file_panel(
    ui: &mut egui::Ui, file_picker: &mut FilePicker, root: &lb::File, file_panel_index: usize,
) {
    egui::ScrollArea::vertical()
        .id_source(root.name.clone())
        .show(ui, |ui| {
            ui.set_width(235.0);
            ui.add_space(15.0);
            ui.with_layout(
                egui::Layout::top_down(egui::Align::Min).with_cross_justify(true),
                |ui| {
                    ui.add_space(15.0);
                    let children = file_picker.core.get_children(root.id).unwrap_or_default();
                    let mut children: Vec<&File> = children
                        .iter()
                        .filter(|f| f.file_type == lb::FileType::Folder)
                        .collect();
                    children.sort_by(|a, b| a.name.cmp(&b.name));

                    for child in children {
                        show_node(ui, file_picker, child, file_panel_index, NodeMode::Panel);
                    }
                },
            );
        });
}

fn show_bottom_bar(ui: &mut egui::Ui, file_picker: &mut FilePicker) -> Option<FilePickerParams> {
    ui.horizontal(|ui| {
        egui::ScrollArea::horizontal()
            .max_width(ui.available_width() - 100.0) // allow some room for the cta
            .show(ui, |ui| {
                for (i, f) in file_picker.panels.clone().iter().enumerate() {
                    show_node(ui, file_picker, f, i, NodeMode::BottomBar);

                    ui.label(
                        egui::RichText::new(">")
                            .size(15.0)
                            .color(egui::Color32::GRAY),
                    );
                }

                let icon = match file_picker.target.file_type {
                    lb::FileType::Folder => Icon::FOLDER,
                    _ => DocType::from_name(&file_picker.target.name).to_icon(),
                };

                icon.show(ui);

                ui.label(&file_picker.target.name);
            });
        ui.spacing_mut().button_padding = egui::vec2(25.0, 5.0);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            if ui.button("Select").clicked() {
                return Some(FilePickerParams {
                    parent: file_picker.panels.last().unwrap().clone(), // there's always one panel (the root), so th unwrap is sage
                    target: file_picker.target.clone(),
                });
            }
            None
        })
    })
    .inner
    .inner
}

enum NodeMode {
    Panel,
    BottomBar,
}

fn show_node(
    ui: &mut egui::Ui, file_picker: &mut FilePicker, node: &File, file_panel_index: usize,
    mode: NodeMode,
) {
    let mut icon_style = (*ui.ctx().style()).clone();
    let icon_stroke = egui::Stroke { color: ui.visuals().hyperlink_color, ..Default::default() };
    icon_style.visuals.widgets.inactive.fg_stroke = icon_stroke;
    icon_style.visuals.widgets.active.fg_stroke = icon_stroke;
    icon_style.visuals.widgets.hovered.fg_stroke = icon_stroke;

    let is_child_open = file_picker.panels.iter().any(|f| f.eq(node));
    let is_node_grayed_out = match mode {
        NodeMode::Panel => !is_child_open && file_panel_index != file_picker.panels.len() - 1,
        NodeMode::BottomBar => file_panel_index < file_picker.panels.len().saturating_sub(2),
    };

    if is_node_grayed_out {
        let icon_stroke = egui::Stroke {
            color: ui.visuals().hyperlink_color.gamma_multiply(0.3),
            ..Default::default()
        };
        icon_style.visuals.widgets.inactive.fg_stroke = icon_stroke;
        icon_style.visuals.widgets.active.fg_stroke = icon_stroke;
        icon_style.visuals.widgets.hovered.fg_stroke = icon_stroke;

        ui.visuals_mut().widgets.inactive.fg_stroke =
            egui::Stroke { color: egui::Color32::GRAY, ..Default::default() };
    }

    if Button::default()
        .text(node.name.clone().as_str())
        .icon(&Icon::FOLDER)
        .icon_style(icon_style)
        .show(ui)
        .clicked()
    {
        let drain_index = match mode {
            NodeMode::Panel => file_panel_index + 1,
            NodeMode::BottomBar => file_panel_index,
        };

        file_picker.panels.drain((drain_index)..);
        file_picker.panels.push(node.clone());
    };
}
