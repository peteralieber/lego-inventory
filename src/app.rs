use std::collections::HashMap;
use std::fmt;

/// Represents a color in the Lego inventory system
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum LegoColor {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Brown,
    DarkTan,
    LightGray,
    DarkGray,
    Clear,
    Custom(String),
}

impl fmt::Display for LegoColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LegoColor::Black => write!(f, "Black"),
            LegoColor::White => write!(f, "White"),
            LegoColor::Red => write!(f, "Red"),
            LegoColor::Green => write!(f, "Green"),
            LegoColor::Blue => write!(f, "Blue"),
            LegoColor::Yellow => write!(f, "Yellow"),
            LegoColor::Orange => write!(f, "Orange"),
            LegoColor::Brown => write!(f, "Brown"),
            LegoColor::DarkTan => write!(f, "Dark Tan"),
            LegoColor::LightGray => write!(f, "Light Gray"),
            LegoColor::DarkGray => write!(f, "Dark Gray"),
            LegoColor::Clear => write!(f, "Clear"),
            LegoColor::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl LegoColor {
    pub fn all_standard_colors() -> Vec<LegoColor> {
        vec![
            LegoColor::Black,
            LegoColor::White,
            LegoColor::Red,
            LegoColor::Green,
            LegoColor::Blue,
            LegoColor::Yellow,
            LegoColor::Orange,
            LegoColor::Brown,
            LegoColor::DarkTan,
            LegoColor::LightGray,
            LegoColor::DarkGray,
            LegoColor::Clear,
        ]
    }
}

/// Categories for organization by color
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum ColorCategory {
    Basic,
    Curved,
    Cutoff,
    Slopes,
    TileJump,
    Textured,
    ClipsBars,
    Modified,
    Technic,
    Car,
    Items,
    Minifig,
    Organic,
    ClearParts,
    Tracks,
    Pillar,
    Large,
}

impl fmt::Display for ColorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorCategory::Basic => write!(f, "Basic"),
            ColorCategory::Curved => write!(f, "Curved"),
            ColorCategory::Cutoff => write!(f, "Cutoff"),
            ColorCategory::Slopes => write!(f, "Slopes"),
            ColorCategory::TileJump => write!(f, "Tile+Jump"),
            ColorCategory::Textured => write!(f, "Textured"),
            ColorCategory::ClipsBars => write!(f, "Clips & Bars"),
            ColorCategory::Modified => write!(f, "Modified"),
            ColorCategory::Technic => write!(f, "Technic"),
            ColorCategory::Car => write!(f, "Car"),
            ColorCategory::Items => write!(f, "Items"),
            ColorCategory::Minifig => write!(f, "Minifig"),
            ColorCategory::Organic => write!(f, "Organic"),
            ColorCategory::ClearParts => write!(f, "Clear"),
            ColorCategory::Tracks => write!(f, "Tracks"),
            ColorCategory::Pillar => write!(f, "Pillar"),
            ColorCategory::Large => write!(f, "Large"),
        }
    }
}

impl ColorCategory {
    pub fn all_categories() -> Vec<ColorCategory> {
        vec![
            ColorCategory::Basic,
            ColorCategory::Curved,
            ColorCategory::Cutoff,
            ColorCategory::Slopes,
            ColorCategory::TileJump,
            ColorCategory::Textured,
            ColorCategory::ClipsBars,
            ColorCategory::Modified,
            ColorCategory::Technic,
            ColorCategory::Car,
            ColorCategory::Items,
            ColorCategory::Minifig,
            ColorCategory::Organic,
            ColorCategory::ClearParts,
            ColorCategory::Tracks,
            ColorCategory::Pillar,
            ColorCategory::Large,
        ]
    }

    pub fn description(&self) -> &'static str {
        match self {
            ColorCategory::Basic => "Standard bricks and plates",
            ColorCategory::Curved => "Arcs, circles, and rounded elements",
            ColorCategory::Cutoff => "Wedges and angular cut pieces",
            ColorCategory::Slopes => "Inclined bricks, including studless variants",
            ColorCategory::TileJump => "Flat tiles and jumper plates",
            ColorCategory::Textured => "Bricks with surface texture or holes",
            ColorCategory::ClipsBars => "Hinges, clips, rods, and bars",
            ColorCategory::Modified => "Unique or altered shapes not fitting other categories",
            ColorCategory::Technic => "Gears, axles, pins, and chain-compatible parts",
            ColorCategory::Car => "Wheels, fenders, chassis components",
            ColorCategory::Items => "Accessories, tools, and miscellaneous objects",
            ColorCategory::Minifig => "Figures, dolls, and related gear",
            ColorCategory::Organic => "Plants, horns, rocks, and natural shapes",
            ColorCategory::ClearParts => "Transparent or translucent parts",
            ColorCategory::Tracks => "Train tracks and rail-compatible pieces",
            ColorCategory::Pillar => "Structural supports and columns",
            ColorCategory::Large => "Oversized bricks, baseplates, and specialty elements",
        }
    }
}

/// Categories for organization by part type
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum TypeCategory {
    Dots,          // 1×1 round tiles
    Dotst,         // Dot-shaped decorative elements
    Stud,          // 1×1 round plates
    OneByOne,      // 1×1 square plates
    OneByOneTile,  // 1×1 flat tiles
    OneByOneSlope, // Small angled bricks
    X2Slope,       // 1×2 slopes, studless or smooth
    Snot,          // Studs Not On Top bricks
}

impl fmt::Display for TypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeCategory::Dots => write!(f, "Dots"),
            TypeCategory::Dotst => write!(f, "Dotst"),
            TypeCategory::Stud => write!(f, "Stud"),
            TypeCategory::OneByOne => write!(f, "1×1"),
            TypeCategory::OneByOneTile => write!(f, "1×1 Tile"),
            TypeCategory::OneByOneSlope => write!(f, "1×1 Slope"),
            TypeCategory::X2Slope => write!(f, "x2 Slope"),
            TypeCategory::Snot => write!(f, "SNOT"),
        }
    }
}

impl TypeCategory {
    pub fn all_categories() -> Vec<TypeCategory> {
        vec![
            TypeCategory::Dots,
            TypeCategory::Dotst,
            TypeCategory::Stud,
            TypeCategory::OneByOne,
            TypeCategory::OneByOneTile,
            TypeCategory::OneByOneSlope,
            TypeCategory::X2Slope,
            TypeCategory::Snot,
        ]
    }

    pub fn description(&self) -> &'static str {
        match self {
            TypeCategory::Dots => "1×1 round tiles",
            TypeCategory::Dotst => "Dot-shaped decorative elements",
            TypeCategory::Stud => "1×1 round plates",
            TypeCategory::OneByOne => "1×1 square plates",
            TypeCategory::OneByOneTile => "1×1 flat tiles",
            TypeCategory::OneByOneSlope => "Small angled bricks",
            TypeCategory::X2Slope => "1×2 slopes, studless or smooth",
            TypeCategory::Snot => "Studs Not On Top bricks (side-stud parts)",
        }
    }
}

/// Represents a Lego part in the inventory
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LegoPart {
    pub name: String,
    pub color: LegoColor,
    pub color_category: Option<ColorCategory>,
    pub type_category: Option<TypeCategory>,
    pub quantity: u32,
}

/// Organization mode for viewing the inventory
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum OrganizationMode {
    ByColor,
    ByType,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Inventory data
    parts: Vec<LegoPart>,

    // UI state for adding parts
    new_part_name: String,
    new_part_color: LegoColor,
    new_part_color_category: Option<ColorCategory>,
    new_part_type_category: Option<TypeCategory>,
    new_part_quantity: String,
    new_part_custom_color: String,
    show_custom_color_input: bool,

    // Organization settings
    organization_mode: OrganizationMode,
    show_empty_categories: bool,

    // Filter/search
    search_query: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            parts: Vec::new(),
            new_part_name: String::new(),
            new_part_color: LegoColor::Black,
            new_part_color_category: None,
            new_part_type_category: None,
            new_part_quantity: "1".to_string(),
            new_part_custom_color: String::new(),
            show_custom_color_input: false,
            organization_mode: OrganizationMode::ByColor,
            show_empty_categories: false,
            search_query: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn add_part(&mut self) {
        let quantity = match self.new_part_quantity.parse::<u32>() {
            Ok(q) if q > 0 => q,
            _ => {
                // Invalid quantity - we'll add validation feedback in the UI
                return;
            }
        };

        if self.new_part_name.is_empty() {
            return;
        }

        let color = if self.show_custom_color_input && !self.new_part_custom_color.is_empty() {
            LegoColor::Custom(self.new_part_custom_color.clone())
        } else {
            self.new_part_color.clone()
        };

        self.parts.push(LegoPart {
            name: self.new_part_name.clone(),
            color,
            color_category: self.new_part_color_category.clone(),
            type_category: self.new_part_type_category.clone(),
            quantity,
        });

        // Reset all form fields
        self.new_part_name.clear();
        self.new_part_color = LegoColor::Black;
        self.new_part_color_category = None;
        self.new_part_type_category = None;
        self.new_part_quantity = "1".to_string();
        self.new_part_custom_color.clear();
        self.show_custom_color_input = false;
    }

    fn remove_part(&mut self, index: usize) {
        if index < self.parts.len() {
            self.parts.remove(index);
        }
    }

    fn get_parts_by_color(&self) -> HashMap<LegoColor, HashMap<Option<ColorCategory>, Vec<usize>>> {
        let mut result = HashMap::new();
        for (idx, part) in self.parts.iter().enumerate() {
            if self.matches_search(part) {
                result
                    .entry(part.color.clone())
                    .or_insert_with(HashMap::new)
                    .entry(part.color_category.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
        result
    }

    fn get_parts_by_type(&self) -> HashMap<Option<TypeCategory>, HashMap<LegoColor, Vec<usize>>> {
        let mut result = HashMap::new();
        for (idx, part) in self.parts.iter().enumerate() {
            if self.matches_search(part) {
                result
                    .entry(part.type_category.clone())
                    .or_insert_with(HashMap::new)
                    .entry(part.color.clone())
                    .or_insert_with(Vec::new)
                    .push(idx);
            }
        }
        result
    }

    fn matches_search(&self, part: &LegoPart) -> bool {
        if self.search_query.is_empty() {
            return true;
        }
        let query = self.search_query.to_lowercase();
        part.name.to_lowercase().contains(&query)
            || format!("{}", part.color).to_lowercase().contains(&query)
    }

    fn render_by_color_view(&mut self, ui: &mut egui::Ui) {
        let parts_by_color = self.get_parts_by_color();
        let mut to_remove = None;

        if parts_by_color.is_empty() {
            ui.label("No parts in inventory. Add some parts using the panel on the left!");
            return;
        }

        // Collect all colors that have parts (including custom colors)
        let mut colors_with_parts: Vec<_> = parts_by_color.keys().collect();
        colors_with_parts.sort_by_key(|color| format!("{}", color));

        for color in colors_with_parts {
            if let Some(categories) = parts_by_color.get(color) {
                ui.collapsing(
                    format!(
                        "🎨 {} ({} parts)",
                        color,
                        categories
                            .values()
                            .flatten()
                            .map(|&idx| self.parts[idx].quantity)
                            .sum::<u32>()
                    ),
                    |ui| {
                        for category in ColorCategory::all_categories() {
                            if let Some(part_indices) = categories.get(&Some(category.clone())) {
                                ui.group(|ui| {
                                    ui.strong(format!("{}", category));
                                    ui.label(
                                        egui::RichText::new(category.description())
                                            .small()
                                            .italics(),
                                    );
                                    ui.separator();

                                    for &idx in part_indices {
                                        let part = &self.parts[idx];
                                        ui.horizontal(|ui| {
                                            ui.label(&part.name);
                                            ui.label(format!("({})", part.quantity));
                                            if ui.button("🗑").clicked() {
                                                to_remove = Some(idx);
                                            }
                                        });
                                    }
                                });
                            } else if self.show_empty_categories {
                                ui.group(|ui| {
                                    ui.strong(format!("{}", category));
                                    ui.label(
                                        egui::RichText::new(category.description())
                                            .small()
                                            .italics(),
                                    );
                                    ui.separator();
                                    ui.label(egui::RichText::new("(empty)").italics().weak());
                                });
                            }
                        }

                        // Uncategorized parts
                        if let Some(part_indices) = categories.get(&None) {
                            ui.group(|ui| {
                                ui.strong("Uncategorized");
                                ui.separator();

                                for &idx in part_indices {
                                    let part = &self.parts[idx];
                                    ui.horizontal(|ui| {
                                        ui.label(&part.name);
                                        ui.label(format!("({})", part.quantity));
                                        if ui.button("🗑").clicked() {
                                            to_remove = Some(idx);
                                        }
                                    });
                                }
                            });
                        }
                    },
                );
            }
        }

        if let Some(idx) = to_remove {
            self.remove_part(idx);
        }
    }

    fn render_by_type_view(&mut self, ui: &mut egui::Ui) {
        let parts_by_type = self.get_parts_by_type();
        let mut to_remove = None;

        if parts_by_type.is_empty() {
            ui.label("No parts in inventory. Add some parts using the panel on the left!");
            return;
        }

        for type_category in TypeCategory::all_categories() {
            if let Some(colors) = parts_by_type.get(&Some(type_category.clone())) {
                ui.collapsing(
                    format!(
                        "🔧 {} ({} parts)",
                        type_category,
                        colors
                            .values()
                            .flatten()
                            .map(|&idx| self.parts[idx].quantity)
                            .sum::<u32>()
                    ),
                    |ui| {
                        ui.label(
                            egui::RichText::new(type_category.description())
                                .small()
                                .italics(),
                        );
                        ui.separator();

                        for (color, part_indices) in colors {
                            ui.group(|ui| {
                                ui.strong(format!("{}", color));

                                for &idx in part_indices {
                                    let part = &self.parts[idx];
                                    ui.horizontal(|ui| {
                                        ui.label(&part.name);
                                        ui.label(format!("({})", part.quantity));
                                        if ui.button("🗑").clicked() {
                                            to_remove = Some(idx);
                                        }
                                    });
                                }
                            });
                        }
                    },
                );
            }
        }

        // Uncategorized parts
        if let Some(colors) = parts_by_type.get(&None) {
            ui.collapsing(
                format!(
                    "🔧 Uncategorized ({} parts)",
                    colors
                        .values()
                        .flatten()
                        .map(|&idx| self.parts[idx].quantity)
                        .sum::<u32>()
                ),
                |ui| {
                    for (color, part_indices) in colors {
                        ui.group(|ui| {
                            ui.strong(format!("{}", color));

                            for &idx in part_indices {
                                let part = &self.parts[idx];
                                ui.horizontal(|ui| {
                                    ui.label(&part.name);
                                    ui.label(format!("({})", part.quantity));
                                    if ui.button("🗑").clicked() {
                                        to_remove = Some(idx);
                                    }
                                });
                            }
                        });
                    }
                },
            );
        }

        if let Some(idx) = to_remove {
            self.remove_part(idx);
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("control_panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Add New Part");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Part Name:");
                    ui.text_edit_singleline(&mut self.new_part_name);
                });

                ui.horizontal(|ui| {
                    ui.label("Color:");
                    if !self.show_custom_color_input {
                        egui::ComboBox::from_id_salt("color_selector")
                            .selected_text(format!("{}", self.new_part_color))
                            .show_ui(ui, |ui| {
                                for color in LegoColor::all_standard_colors() {
                                    ui.selectable_value(
                                        &mut self.new_part_color,
                                        color.clone(),
                                        format!("{}", color),
                                    );
                                }
                            });
                        if ui.small_button("+ Custom").clicked() {
                            self.show_custom_color_input = true;
                        }
                    } else {
                        ui.text_edit_singleline(&mut self.new_part_custom_color);
                        if ui.small_button("Standard").clicked() {
                            self.show_custom_color_input = false;
                            self.new_part_custom_color.clear();
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Color Category:");
                    egui::ComboBox::from_id_salt("color_category_selector")
                        .selected_text(
                            self.new_part_color_category
                                .as_ref()
                                .map(|c| format!("{}", c))
                                .unwrap_or_else(|| "None".to_string()),
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_part_color_category, None, "None");
                            for category in ColorCategory::all_categories() {
                                ui.selectable_value(
                                    &mut self.new_part_color_category,
                                    Some(category.clone()),
                                    format!("{}", category),
                                );
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Type Category:");
                    egui::ComboBox::from_id_salt("type_category_selector")
                        .selected_text(
                            self.new_part_type_category
                                .as_ref()
                                .map(|c| format!("{}", c))
                                .unwrap_or_else(|| "None".to_string()),
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.new_part_type_category, None, "None");
                            for category in TypeCategory::all_categories() {
                                ui.selectable_value(
                                    &mut self.new_part_type_category,
                                    Some(category.clone()),
                                    format!("{}", category),
                                );
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Quantity:");
                    ui.text_edit_singleline(&mut self.new_part_quantity);
                });

                // Show validation feedback for quantity
                if !self.new_part_quantity.is_empty() {
                    if self.new_part_quantity.parse::<u32>().is_err() {
                        ui.label(
                            egui::RichText::new("⚠ Please enter a valid positive number")
                                .color(egui::Color32::from_rgb(255, 100, 100)),
                        );
                    } else if self.new_part_quantity.parse::<u32>().unwrap_or(0) == 0 {
                        ui.label(
                            egui::RichText::new("⚠ Quantity must be greater than 0")
                                .color(egui::Color32::from_rgb(255, 100, 100)),
                        );
                    }
                }

                if ui.button("➕ Add Part").clicked() {
                    self.add_part();
                }

                ui.separator();
                ui.heading("Settings");

                ui.horizontal(|ui| {
                    ui.label("Organization:");
                    ui.selectable_value(
                        &mut self.organization_mode,
                        OrganizationMode::ByColor,
                        "By Color",
                    );
                    ui.selectable_value(
                        &mut self.organization_mode,
                        OrganizationMode::ByType,
                        "By Type",
                    );
                });

                ui.checkbox(&mut self.show_empty_categories, "Show empty categories");

                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.search_query);
                });

                ui.separator();
                ui.label(format!(
                    "Total parts: {}",
                    self.parts.iter().map(|p| p.quantity).sum::<u32>()
                ));
                ui.label(format!("Unique parts: {}", self.parts.len()));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🧱 Lego Inventory");

            egui::ScrollArea::vertical().show(ui, |ui| match self.organization_mode {
                OrganizationMode::ByColor => self.render_by_color_view(ui),
                OrganizationMode::ByType => self.render_by_type_view(ui),
            });
        });
    }
}
