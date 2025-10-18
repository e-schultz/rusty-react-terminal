use crate::models::{FieldGuideData, GlitchSanctuaryData, View};

/// Field Guide UI state
#[derive(Debug, Clone)]
pub struct FieldGuideState {
    pub data: FieldGuideData,
    pub selected_section_idx: usize,
    pub selected_entry_idx: usize,
    pub active_filter: Option<String>, // None = no filter, Some(color) = filter by color
    pub viewing_details: bool,         // Whether we're viewing entry details
    pub scroll_offset: usize,          // Vertical scroll offset for grid (in rows of 3)
}

impl FieldGuideState {
    pub fn new(data: FieldGuideData) -> Self {
        Self {
            data,
            selected_section_idx: 0,
            selected_entry_idx: 0,
            active_filter: None,
            viewing_details: false,
            scroll_offset: 0,
        }
    }

    /// Get filtered sections based on active filter
    pub fn filtered_sections(&self) -> Vec<usize> {
        if let Some(ref color) = self.active_filter {
            self.data
                .sections
                .iter()
                .enumerate()
                .filter(|(_, s)| s.color == *color)
                .map(|(i, _)| i)
                .collect()
        } else {
            (0..self.data.sections.len()).collect()
        }
    }

    pub fn current_section(&self) -> Option<&crate::models::Section> {
        self.data.sections.get(self.selected_section_idx)
    }

    pub fn current_entry(&self) -> Option<&crate::models::Entry> {
        self.current_section()
            .and_then(|s| s.entries.get(self.selected_entry_idx))
    }

    pub fn next_section(&mut self) {
        let filtered = self.filtered_sections();
        let current_pos = filtered.iter().position(|&i| i == self.selected_section_idx);

        if let Some(pos) = current_pos {
            if pos + 1 < filtered.len() {
                self.selected_section_idx = filtered[pos + 1];
                self.selected_entry_idx = 0;
            }
        }
    }

    pub fn prev_section(&mut self) {
        let filtered = self.filtered_sections();
        let current_pos = filtered.iter().position(|&i| i == self.selected_section_idx);

        if let Some(pos) = current_pos {
            if pos > 0 {
                self.selected_section_idx = filtered[pos - 1];
                self.selected_entry_idx = 0;
            }
        }
    }

    pub fn next_entry(&mut self) {
        if let Some(section) = self.current_section() {
            if self.selected_entry_idx + 1 < section.entries.len() {
                self.selected_entry_idx += 1;
            }
        }
    }

    pub fn prev_entry(&mut self) {
        if self.selected_entry_idx > 0 {
            self.selected_entry_idx -= 1;
        }
    }

    pub fn toggle_filter(&mut self, color: String) {
        if self.active_filter.as_ref() == Some(&color) {
            self.active_filter = None;
        } else {
            self.active_filter = Some(color);
        }
        self.selected_section_idx = 0;
        self.selected_entry_idx = 0;
        self.scroll_offset = 0;
    }

    /// Scroll down by one row (3 sections per row)
    pub fn scroll_down(&mut self) {
        let filtered = self.filtered_sections();
        let total_rows = (filtered.len() + 2) / 3; // Ceiling division
        if self.scroll_offset + 1 < total_rows {
            self.scroll_offset += 1;
        }
    }

    /// Scroll up by one row
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Get visible sections based on scroll offset (max 3 sections per row)
    pub fn visible_sections(&self) -> Vec<usize> {
        let filtered = self.filtered_sections();
        let start = self.scroll_offset * 3;
        let end = (start + 3).min(filtered.len());
        if start >= filtered.len() {
            vec![]
        } else {
            filtered[start..end].to_vec()
        }
    }
}

/// Glitch Sanctuary UI state
#[derive(Debug, Clone)]
pub struct SanctuaryState {
    pub data: GlitchSanctuaryData,
    pub active_program_idx: usize,
    pub selected_record_idx: usize,
    pub expanded_record_idx: Option<usize>,
    pub record_scroll_offset: usize,  // Vertical scroll offset for records list
}

impl SanctuaryState {
    pub fn new(data: GlitchSanctuaryData) -> Self {
        Self {
            data,
            active_program_idx: 0,
            selected_record_idx: 0,
            expanded_record_idx: None,
            record_scroll_offset: 0,
        }
    }

    pub fn current_program(&self) -> Option<&crate::models::Program> {
        self.data.programs.get(self.active_program_idx)
    }

    pub fn current_program_mut(&mut self) -> Option<&mut crate::models::Program> {
        self.data.programs.get_mut(self.active_program_idx)
    }

    pub fn current_record(&self) -> Option<&crate::models::Record> {
        self.current_program()
            .and_then(|p| p.records.get(self.selected_record_idx))
    }

    pub fn next_program(&mut self) {
        if self.active_program_idx + 1 < self.data.programs.len() {
            self.active_program_idx += 1;
            self.selected_record_idx = 0;
            self.record_scroll_offset = 0;
        }
    }

    pub fn prev_program(&mut self) {
        if self.active_program_idx > 0 {
            self.active_program_idx -= 1;
            self.selected_record_idx = 0;
            self.record_scroll_offset = 0;
        }
    }

    pub fn next_record(&mut self) {
        if let Some(program) = self.current_program() {
            if self.selected_record_idx + 1 < program.records.len() {
                self.selected_record_idx += 1;
            }
        }
    }

    pub fn prev_record(&mut self) {
        if self.selected_record_idx > 0 {
            self.selected_record_idx -= 1;
        }
    }

    pub fn toggle_record_details(&mut self, idx: usize) {
        if self.expanded_record_idx == Some(idx) {
            self.expanded_record_idx = None;
        } else {
            self.expanded_record_idx = Some(idx);
        }
    }

    /// Scroll down in records list
    pub fn scroll_records_down(&mut self) {
        if let Some(program) = self.current_program() {
            if self.record_scroll_offset + 1 < program.records.len() {
                self.record_scroll_offset += 1;
            }
        }
    }

    /// Scroll up in records list
    pub fn scroll_records_up(&mut self) {
        if self.record_scroll_offset > 0 {
            self.record_scroll_offset -= 1;
        }
    }

    /// Reset scroll offset when switching programs
    pub fn reset_scroll_offset(&mut self) {
        self.record_scroll_offset = 0;
    }
}

/// Global application state
#[derive(Debug)]
pub struct AppState {
    pub current_view: View,
    pub field_guide: FieldGuideState,
    pub sanctuary: SanctuaryState,
    pub command_input: String,
    pub show_command_palette: bool,
}

impl AppState {
    pub fn new(field_guide_data: FieldGuideData, sanctuary_data: GlitchSanctuaryData) -> Self {
        Self {
            current_view: View::FieldGuide,
            field_guide: FieldGuideState::new(field_guide_data),
            sanctuary: SanctuaryState::new(sanctuary_data),
            command_input: String::new(),
            show_command_palette: false,
        }
    }

    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn toggle_command_palette(&mut self) {
        self.show_command_palette = !self.show_command_palette;
        if self.show_command_palette {
            self.command_input.clear();
        }
    }
}
